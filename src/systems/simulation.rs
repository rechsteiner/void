use std::collections::HashMap;

use crate::components::gravity::{GravityAffected, GravitySource};
use crate::components::program::Program;
use crate::components::rigid_body::{PhysicsMode, RigidBody, Transform};
use crate::components::shape::{Point, Shape};
use crate::interpreter::object::Command;
use crate::systems::System;
use crate::world::World;
use rapier2d::crossbeam;
use rapier2d::geometry::{BroadPhase, ColliderSet, NarrowPhase};
use rapier2d::pipeline::PhysicsPipeline;
use rapier2d::{dynamics::BodyStatus, na::Vector2};
use rapier2d::{
    dynamics::{IntegrationParameters, JointSet, RigidBodyBuilder, RigidBodyHandle, RigidBodySet},
    geometry::ColliderBuilder,
    pipeline::ChannelEventCollector,
};
pub struct SimulationSystem {
    body_handles: HashMap<usize, Option<RigidBodyHandle>>,
    spaceship_handle: Option<RigidBodyHandle>,
    physics_pipeline: PhysicsPipeline,
    gravity: Vector2<f32>,
    integration_parameters: IntegrationParameters,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    bodies: RigidBodySet,
    colliders: ColliderSet,
    joints: JointSet,
}

impl SimulationSystem {
    pub fn new() -> SimulationSystem {
        SimulationSystem {
            body_handles: HashMap::new(),
            spaceship_handle: None,
            physics_pipeline: PhysicsPipeline::new(),
            gravity: Vector2::new(0.0, 0.0),
            integration_parameters: IntegrationParameters::default(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            bodies: RigidBodySet::new(),
            colliders: ColliderSet::new(),
            joints: JointSet::new(),
        }
    }
}

impl System for SimulationSystem {
    fn update(&mut self, world: &mut World) {
        // TODO: For now we are just checking weather the bodies are empty and
        // so we only insert our components once. This won't work when we start
        // adding and removing components.
        if self.bodies.len() == 0 {
            for (index, (rigid_body, shape)) in
                world.query::<(&RigidBody, &Shape)>().iter().enumerate()
            {
                let body_status = match rigid_body.physics_mode {
                    PhysicsMode::Dynamic => BodyStatus::Dynamic,
                    PhysicsMode::Static => BodyStatus::Static,
                };
                let entity_rb = RigidBodyBuilder::new(body_status)
                    .translation(
                        rigid_body.transform.position.x,
                        rigid_body.transform.position.y,
                    )
                    .rotation(rigid_body.transform.rotation)
                    .linvel(rigid_body.linear_velocity.x, rigid_body.linear_velocity.y)
                    .angvel(rigid_body.angular_velocity)
                    .mass(rigid_body.mass)
                    .build();

                // Add loose mapping between rigid_body components and physics bodies
                let entity_handle = self.bodies.insert(entity_rb);
                self.body_handles.insert(rigid_body.id, Some(entity_handle));

                // TODO: We should probably apply commands to all entities with "Program" component instead of doing this
                if index == 0 {
                    self.spaceship_handle = Some(entity_handle);
                }

                let mut points = Vec::new();

                for point in &shape.vertices {
                    points.push(rapier2d::na::Point2::new(point.x, point.y));
                }

                let entity_collider = ColliderBuilder::convex_hull(&points)
                    .unwrap()
                    .sensor(shape.is_sensor)
                    .build();
                self.colliders
                    .insert(entity_collider, entity_handle, &mut self.bodies);
            }
        }

        // TODO: Apply force on a specfic body with the correct vector
        let spaceship_body = self.bodies.get_mut(self.spaceship_handle.unwrap()).unwrap();

        for program in world.query::<&Program>() {
            for command in &program.commands {
                match command {
                    Command::SetThrust { force } => {
                        let rotation = spaceship_body.position().rotation.angle();
                        spaceship_body.apply_impulse(
                            Vector2::new(
                                1.0 - (*force as f32) * rotation.sin(), // cos(0) - sin(⍺) = 1 - sin(⍺)
                                (*force as f32) * rotation.cos(), // sin(1) + cos(⍺) = 0 + cos(⍺)
                            ),
                            true,
                        );
                    }
                    Command::SetTorque { force } => {
                        spaceship_body.apply_torque_impulse(*force as f32, true)
                    }
                }
            }
        }

        {
            // Sum and apply all gravity forces a body is subjected to.
            // Get all the gravity sources in the world and store them for later iteration by each entity
            let gravity_sources = world.query::<(&GravitySource, &RigidBody)>();

            // Find all the entities that are supposed to be affected by gravity
            for (_gravity_affected, rigid_body) in world.query::<(&GravityAffected, &RigidBody)>() {
                let mut sum_gravity_vector = Vector2::new(0.0, 0.0);

                // For each gravity source, accumulate its force into the sum_gravity_vector
                // This enables the support for multiple gravity sources in a world
                for (gravity, gravity_rb) in &gravity_sources {
                    let offset = Vector2::new(
                        gravity_rb.transform.position.x - rigid_body.transform.position.x,
                        gravity_rb.transform.position.y - rigid_body.transform.position.y,
                    );
                    let distance: f32 = offset.magnitude();

                    // Make sure we don't divide by near-zero
                    // (Got some strange bugs if I didn't do this and objects overlapped with gravity sources)
                    if distance >= 0.01 {
                        sum_gravity_vector += offset.normalize() * (gravity.strength / distance.powf(2.0));
                    }
                }

                let handle = self.body_handles.get(&rigid_body.id).unwrap().unwrap();
                let body = self.bodies.get_mut(handle).unwrap();

                // Multiply gravity force with object mass to get consistent acceleration for bodies of different masses
                body.apply_force(sum_gravity_vector * body.mass(), true);
            }
        }

        let (contact_send, contact_recv) = crossbeam::channel::unbounded();
        let (intersection_send, intersection_recv) = crossbeam::channel::unbounded();
        let event_handler = ChannelEventCollector::new(intersection_send, contact_send);

        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.joints,
            &(),
            &event_handler,
        );

        let physics_bodies = self.bodies.iter();
        let rigid_bodies = world.query_mut::<&mut RigidBody>().into_iter();

        for ((_handle, physics_body), rigid_body) in physics_bodies.zip(rigid_bodies) {
            rigid_body.transform = Transform {
                position: Point {
                    x: physics_body.position().translation.x,
                    y: physics_body.position().translation.y,
                },
                rotation: physics_body.position().rotation.angle(),
            };
            rigid_body.linear_velocity = Point {
                x: physics_body.linvel().x,
                y: physics_body.linvel().y,
            };
            rigid_body.angular_velocity = physics_body.angvel();
        }

        while let Ok(intersection_event) = intersection_recv.try_recv() {
            let _parent = self
                .colliders
                .get(intersection_event.collider1)
                .unwrap()
                .parent();
        }

        while let Ok(_contact_event) = contact_recv.try_recv() {}
    }
}
