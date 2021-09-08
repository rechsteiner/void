use crate::components::physics_mode::PhysicsMode;
use crate::components::program::Program;
use crate::components::rigid_body::{RigidBody, Transform};
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
    geometry::{ColliderBuilder, ContactEvent, IntersectionEvent},
    pipeline::{ChannelEventCollector, EventHandler},
};
use web_sys::console;

pub struct SimulationSystem {
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
            spaceship_handle: None,
            physics_pipeline: PhysicsPipeline::new(),
            gravity: Vector2::new(0.0, 100.0),
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
            for (index, (rigid_body, shape, physics_mode)) in world
                .query::<(&RigidBody, &Shape, &PhysicsMode)>()
                .iter()
                .enumerate()
            {
                let body_status = match physics_mode {
                    PhysicsMode::Dynamic => BodyStatus::Dynamic,
                    PhysicsMode::Static => BodyStatus::Static,
                };
                let entity_rb = RigidBodyBuilder::new(body_status)
                    .translation(
                        rigid_body.transform.position.x,
                        rigid_body.transform.position.y,
                    )
                    .rotation(rigid_body.transform.rotation)
                    .mass(rigid_body.mass)
                    .build();

                let entity_handle = self.bodies.insert(entity_rb);

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
            let parent = self
                .colliders
                .get(intersection_event.collider1)
                .unwrap()
                .parent();

            unsafe {
                console::log_1(&"It intersects".into());
            }
        }

        while let Ok(contact_event) = contact_recv.try_recv() {
            unsafe {
                console::log_1(&"It contacts".into());
            }
        }
    }
}
