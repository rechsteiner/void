use rapier2d::geometry::{BroadPhase, ColliderSet, NarrowPhase};
use rapier2d::na::Vector2;
use rapier2d::pipeline::PhysicsPipeline;
use rapier2d::{
    dynamics::{IntegrationParameters, JointSet, RigidBodyBuilder, RigidBodySet},
    geometry::ColliderBuilder,
};

pub struct SimulationState {
    pipeline: PhysicsPipeline,
    gravity: Vector2<f32>,
    integration_parameters: IntegrationParameters,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    bodies: RigidBodySet,
    colliders: ColliderSet,
    joints: JointSet,
}

pub struct Position {
    pub x: f64,
    pub y: f64,
}

pub struct Entity {
    pub position: Position,
}

impl SimulationState {
    pub fn new() -> SimulationState {
        let mut pipeline = PhysicsPipeline::new();
        let gravity = Vector2::new(0.0, 10.0);
        let integration_parameters = IntegrationParameters::default();
        let mut broad_phase = BroadPhase::new();
        let mut narrow_phase = NarrowPhase::new();
        let mut bodies = RigidBodySet::new();
        let mut colliders = ColliderSet::new();
        let mut joints = JointSet::new();

        let cube_rb = RigidBodyBuilder::new_dynamic()
            .translation(100.0, 100.0)
            .mass(1.0)
            .build();
        let cube_handle = bodies.insert(cube_rb);
        let cube_collider = ColliderBuilder::cuboid(2.0, 2.0).build();
        colliders.insert(cube_collider, cube_handle, &mut bodies);

        SimulationState {
            pipeline,
            gravity,
            integration_parameters,
            broad_phase,
            narrow_phase,
            bodies,
            colliders,
            joints,
        }
    }

    pub fn entities(&self) -> Vec<Entity> {
        self.bodies
            .iter()
            .map(|(_, body)| {
                let position = body.position();
                Entity {
                    position: Position {
                        x: position.translation.x as f64,
                        y: position.translation.y as f64,
                    },
                }
            })
            .collect()
    }

    pub fn next_state(&mut self) {
        self.pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.joints,
            &(),
            &(),
        )
    }
}
