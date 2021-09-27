use std::collections::HashMap;
use std::collections::HashSet;

use crate::components::gravity::GravitySource;
use crate::components::program::Program;
use crate::components::rigid_body::{PhysicsMode, RigidBody, Transform};
use crate::components::shape::Shape;
use crate::components::thrusters::Thrusters;
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
    body_handles: HashMap<usize, RigidBodyHandle>,
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

    fn insert_body(&mut self, rigid_body: &RigidBody, shape: &Shape) {
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
        self.body_handles.insert(rigid_body.id, entity_handle);

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

    fn remove_body(&mut self, id: &usize) {
        let handle = self.body_handles.get(id).unwrap();
        self.bodies
            .remove(*handle, &mut self.colliders, &mut self.joints);
        self.body_handles.remove(id);
    }
}

impl System for SimulationSystem {
    fn update(&mut self, world: &mut World) {
        let mut ids = HashSet::<usize>::new();

        for (rigid_body, shape) in world.query::<(&RigidBody, &Shape)>() {
            // Store the id of the rigid body in a set that we can use later to
            // check if any entities have been removed.
            ids.insert(rigid_body.id);

            // Check if we have a rigid body handle for the given id. If it
            // doesn't exists we insert it into Rapier.
            if !self.body_handles.contains_key(&rigid_body.id) {
                self.insert_body(rigid_body, shape);
            }
        }

        // Compare the ids stored in the body handles cache with the current
        // list of ids. If any of them are only present in our cache, it means
        // that the entity have been removed.
        let removed_ids: Vec<usize> = self
            .body_handles
            .keys()
            .filter(|id| !ids.contains(id))
            .map(|id| (*id).clone())
            .collect();

        for id in removed_ids {
            self.remove_body(&id);
        }

        for (program, rigid_body, thrusters) in world.query::<(&Program, &RigidBody, &Thrusters)>()
        {
            let handle = self.body_handles.get(&rigid_body.id).unwrap();
            let body = self.bodies.get_mut(*handle).unwrap();

            if thrusters.get_throttle() > 0.0 {
                let ship_rotation = rigid_body.transform.rotation;
                let thrust_force_magnutude = thrusters.get_total_thrust();
                let thrust_force = Vector2::new(
                    1.0 - thrust_force_magnutude as f32 * (ship_rotation).sin(), // cos(0) - sin(⍺) = 1 - sin(⍺)
                    thrust_force_magnutude as f32 * (ship_rotation).cos(), // sin(1) + cos(⍺) = 0 + cos(⍺)
                );

                if thrusters.fuel > 0.0 {
                    body.apply_impulse(thrust_force, true);
                }
            }

            for command in &program.commands {
                match command {
                    Command::SetTorque { force } => body.apply_torque_impulse(*force as f32, true),
                    _ => (),
                }
            }
        }

        {
            // Sum and apply all gravity forces a body is subjected to.
            // Get all the gravity sources in the world and store them for later iteration by each entity
            let gravity_sources = world.query::<(&GravitySource, &RigidBody)>();

            // Find all the entities that are supposed to be affected by gravity
            for rigid_body in world.query::<&RigidBody>() {
                // It's like I say to the ladies;
                // I won't process any static bodies
                if let PhysicsMode::Static = rigid_body.physics_mode {
                    continue;
                }

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
                        sum_gravity_vector +=
                            offset.normalize() * (gravity.strength / distance.powf(2.0));
                    }
                }

                let handle = self.body_handles.get(&rigid_body.id).unwrap();
                let body = self.bodies.get_mut(*handle).unwrap();

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
                position: Vector2::new(
                    physics_body.position().translation.x,
                    physics_body.position().translation.y,
                ),
                rotation: physics_body.position().rotation.angle(),
            };
            rigid_body.linear_velocity =
                Vector2::new(physics_body.linvel().x, physics_body.linvel().y);
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::components::shape::{ColorRGBA, Point, Shape};

    #[test]
    fn test_update_inserts_new_bodies() {
        let mut system = SimulationSystem::new();
        let mut world = World::new();

        world.register_component::<RigidBody>();
        world.register_component::<Shape>();
        world.register_component::<Program>();
        world.register_component::<GravitySource>();
        system.update(&mut world);

        world
            .create_entity()
            .with_component(Shape {
                is_sensor: false,
                vertices: vec![
                    Point { x: -1.0, y: -1.0 },
                    Point { x: 1.0, y: -1.0 },
                    Point { x: 1.0, y: 1.0 },
                    Point { x: -1.0, y: 1.0 },
                ],
                color: ColorRGBA {
                    r: 255,
                    g: 255,
                    b: 255,
                    a: 1.0,
                },
            })
            .with_component(RigidBody {
                id: 0,
                transform: Transform {
                    position: Vector2::new(0.0, 0.0),
                    rotation: 0.0,
                },
                mass: 1.0,
                linear_velocity: Vector2::new(0.0, 0.0),
                angular_velocity: 0.0,
                physics_mode: PhysicsMode::Static,
            });

        system.update(&mut world);

        assert!(system.body_handles.contains_key(&0));
        assert_eq!(system.colliders.len(), 1);
        assert_eq!(system.bodies.len(), 1);
    }

    #[test]
    fn test_update_removes_old_bodies() {
        let mut system = SimulationSystem::new();
        let mut world = World::new();

        world.register_component::<RigidBody>();
        world.register_component::<Shape>();
        world.register_component::<Program>();
        world.register_component::<GravitySource>();

        world
            .create_entity()
            .with_component(Shape {
                is_sensor: false,
                vertices: vec![
                    Point { x: -1.0, y: -1.0 },
                    Point { x: 1.0, y: -1.0 },
                    Point { x: 1.0, y: 1.0 },
                    Point { x: -1.0, y: 1.0 },
                ],
                color: ColorRGBA {
                    r: 255,
                    g: 255,
                    b: 255,
                    a: 1.0,
                },
            })
            .with_component(RigidBody {
                id: 0,
                transform: Transform {
                    position: Vector2::new(0.0, 0.0),
                    rotation: 0.0,
                },
                mass: 1.0,
                linear_velocity: Vector2::new(0.0, 0.0),
                angular_velocity: 0.0,
                physics_mode: PhysicsMode::Static,
            });

        system.update(&mut world);

        world.remove_entity(0);

        system.update(&mut world);

        assert!(system.body_handles.contains_key(&0) == false);
        assert_eq!(system.colliders.len(), 0);
        assert_eq!(system.bodies.len(), 0);
    }
}
