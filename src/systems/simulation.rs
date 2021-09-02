use crate::components::physics_mode::PhysicsMode;
use crate::components::rigid_body::{RigidBody, Transform};
use crate::components::shape::{Point, Shape};
use crate::ecs::{System, World};
use crate::interpreter::object::Command;
use rapier2d::geometry::{BroadPhase, ColliderSet, NarrowPhase};
use rapier2d::pipeline::PhysicsPipeline;
use rapier2d::{dynamics::BodyStatus, na::Vector2};
use rapier2d::{
    dynamics::{IntegrationParameters, JointSet, RigidBodyBuilder, RigidBodyHandle, RigidBodySet},
    geometry::ColliderBuilder,
};

pub struct SimulationSystem {
    spaceship_handle: Option<RigidBodyHandle>,
    physics_pipeline: PhysicsPipeline,
    physics_world: PhysicsWorldParameters,
}

impl SimulationSystem {
    pub fn new() -> SimulationSystem {
        SimulationSystem {
            spaceship_handle: None,
            physics_pipeline: PhysicsPipeline::new(),
            physics_world: PhysicsWorldParameters {
                gravity: Vector2::new(0.0, 100.0),
                integration_parameters: IntegrationParameters::default(),
                broad_phase: BroadPhase::new(),
                narrow_phase: NarrowPhase::new(),
                bodies: RigidBodySet::new(),
                colliders: ColliderSet::new(),
                joints: JointSet::new(),
            },
        }
    }
}

impl System for SimulationSystem {
    fn update(&mut self, world: &mut World) {
        // TODO: For now we are just checking weather the bodies are empty and
        // so we only insert our components once. This won't work when we start
        // adding and removing components.
        if self.physics_world.bodies.len() == 0 {
            for (index, (rigid_body, shape, physics_mode)) in world
                .query3::<RigidBody, Shape, PhysicsMode>()
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

                let entity_handle = self.physics_world.bodies.insert(entity_rb);

                if index == 0 {
                    self.spaceship_handle = Some(entity_handle);
                }

                let mut points = Vec::new();

                for point in &shape.vertices {
                    points.push(rapier2d::na::Point2::new(point.x, point.y));
                }

                let entity_collider = ColliderBuilder::convex_hull(&points).unwrap().build();
                self.physics_world.colliders.insert(
                    entity_collider,
                    entity_handle,
                    &mut self.physics_world.bodies,
                );
            }
        }

        // TODO: Apply force on a specfic body with the correct vector
        let spaceship_body = self
            .physics_world
            .bodies
            .get_mut(self.spaceship_handle.unwrap())
            .unwrap();

        for command in world.commands.iter() {
            match command {
                Command::SetThrust { force } => {
                    let rotation = spaceship_body.position().rotation.angle();

                    spaceship_body.apply_impulse(
                        Vector2::new(
                            1.0 - (*force as f32) * rotation.sin(), // cos(0) - sin(⍺) = 1 - sin(⍺)
                            (*force as f32) * rotation.cos(),       // sin(1) + cos(⍺) = 0 + cos(⍺)
                        ),
                        true,
                    );
                }
                &Command::SetTorque { force } => {
                    spaceship_body.apply_torque_impulse(force as f32, true)
                }
            }
        }

        self.physics_pipeline.step(
            &self.physics_world.gravity,
            &self.physics_world.integration_parameters,
            &mut self.physics_world.broad_phase,
            &mut self.physics_world.narrow_phase,
            &mut self.physics_world.bodies,
            &mut self.physics_world.colliders,
            &mut self.physics_world.joints,
            &(),
            &(),
        );

        let physics_bodies = self.physics_world.bodies.iter();
        let rigid_bodies = world.query_mut::<RigidBody>().into_iter();

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
