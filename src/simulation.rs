use rapier2d::crossbeam;
use std::collections::HashMap;
use web_sys::console;

use crate::interpreter::object::Command;
use crate::scene::{PhysicsMode, Point, Scene, Transform};
use rapier2d::{
    dynamics::BodyStatus,
    geometry::IntersectionEvent,
    na::Vector2,
    pipeline::{ChannelEventCollector, EventHandler},
};
use rapier2d::{
    dynamics::RigidBodyHandle,
    geometry::{BroadPhase, ColliderSet, NarrowPhase},
};
use rapier2d::{
    dynamics::{IntegrationParameters, JointSet, RigidBodyBuilder, RigidBodySet},
    geometry::ColliderBuilder,
};
use rapier2d::{geometry::ContactEvent, pipeline::PhysicsPipeline};

pub struct Simulation {
    pipeline: PhysicsPipeline,
    world: PhysicsWorldParameters,
    pub entity_dictionary: HashMap<usize, RigidBodyHandle>,
    pub commands: Vec<Command>,
}

impl Simulation {
    pub fn new(scene: &Scene) -> Simulation {
        let pipeline = PhysicsPipeline::new();
        let mut entity_dictionary: HashMap<usize, RigidBodyHandle> = HashMap::new();

        let mut world = PhysicsWorldParameters {
            gravity: Vector2::new(0.0, 100.0),
            integration_parameters: IntegrationParameters::default(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            bodies: RigidBodySet::new(),
            colliders: ColliderSet::new(),
            joints: JointSet::new(),
        };

        for entity in &scene.entities {
            let body_status = match entity.physics_mode {
                PhysicsMode::Dynamic => BodyStatus::Dynamic,
                PhysicsMode::Static => BodyStatus::Static,
            };

            let entity_rb = RigidBodyBuilder::new(body_status)
                .translation(
                    entity.rigidbody.transform.position.x,
                    entity.rigidbody.transform.position.y,
                )
                .rotation(entity.rigidbody.transform.rotation)
                .mass(entity.rigidbody.mass)
                .build();
            let entity_handle = world.bodies.insert(entity_rb);

            let mut points = Vec::new();

            for point in &entity.shape.vertices {
                points.push(rapier2d::na::Point2::new(point.x, point.y));
            }

            let entity_collider = ColliderBuilder::convex_hull(&points)
                .unwrap()
                .sensor(entity.shape.is_sensor)
                .build();
            world
                .colliders
                .insert(entity_collider, entity_handle, &mut world.bodies);

            entity_dictionary.insert(entity.id, entity_handle);
        }

        Simulation {
            pipeline,
            world,
            entity_dictionary,
            commands: vec![],
        }
    }

    pub fn next_state(&mut self) {
        for command in self.commands.iter() {
            match command {
                Command::SetThrust { force } => {
                    let rot = self.get_entity_transform(0).rotation.clone();

                    // TODO: Apply force on a specfic body with the correct vector
                    let rigid_body = self
                        .world
                        .bodies
                        .get_mut(*self.entity_dictionary.get(&0).unwrap())
                        .unwrap();

                    rigid_body.apply_impulse(
                        Vector2::new(
                            1.0 - (*force as f32) * rot.sin(), // cos(0) - sin(⍺) = 1 - sin(⍺)
                            (*force as f32) * rot.cos(),       // sin(1) + cos(⍺) = 0 + cos(⍺)
                        ),
                        true,
                    );
                }
                &Command::SetTorque { force } => {
                    let rigid_body = self
                        .world
                        .bodies
                        .get_mut(*self.entity_dictionary.get(&0).unwrap())
                        .unwrap();
                    rigid_body.apply_torque_impulse(force as f32, true)
                }
            }
        }

        let (contact_send, contact_recv) = crossbeam::channel::unbounded();
        let (intersection_send, intersection_recv) = crossbeam::channel::unbounded();
        let event_handler = ChannelEventCollector::new(intersection_send, contact_send);

        self.pipeline.step(
            &self.world.gravity,
            &self.world.integration_parameters,
            &mut self.world.broad_phase,
            &mut self.world.narrow_phase,
            &mut self.world.bodies,
            &mut self.world.colliders,
            &mut self.world.joints,
            &(),
            &event_handler,
        );

        while let Ok(intersection_event) = intersection_recv.try_recv() {
            let parent = self
                .world
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

    pub fn get_entity_transform(&self, id: usize) -> Transform {
        let physics_state = self
            .world
            .bodies
            .get(*self.entity_dictionary.get(&id).unwrap())
            .unwrap();

        Transform {
            position: Point {
                x: physics_state.position().translation.x,
                y: physics_state.position().translation.y,
            },
            rotation: physics_state.position().rotation.angle(),
        }
    }

    pub fn get_entity_velocity(&self, id: usize) -> Point {
        let physics_state = self
            .world
            .bodies
            .get(*self.entity_dictionary.get(&id).unwrap())
            .unwrap();

        Point {
            x: physics_state.linvel().x,
            y: physics_state.linvel().y,
        }
    }

    pub fn get_entity_radial_velocity(&self, id: usize) -> f32 {
        let physics_state = self
            .world
            .bodies
            .get(*self.entity_dictionary.get(&id).unwrap())
            .unwrap();

        physics_state.angvel()
    }
}

struct PhysicsWorldParameters {
    pub gravity: Vector2<f32>,
    pub integration_parameters: IntegrationParameters,
    pub broad_phase: BroadPhase,
    pub narrow_phase: NarrowPhase,
    pub bodies: RigidBodySet,
    pub colliders: ColliderSet,
    pub joints: JointSet,
}
