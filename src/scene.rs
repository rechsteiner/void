use rapier2d::geometry::{BroadPhase, ColliderSet, NarrowPhase};
use rapier2d::na::Vector2;
use rapier2d::{
    dynamics::{IntegrationParameters, JointSet, RigidBodyBuilder, RigidBodySet},
    geometry::ColliderBuilder,
};

pub enum Command {
    SetThrust(usize),
}

pub struct Scene {
    pub gravity: Vector2<f32>,
    pub integration_parameters: IntegrationParameters,
    pub broad_phase: BroadPhase,
    pub narrow_phase: NarrowPhase,
    pub bodies: RigidBodySet,
    pub colliders: ColliderSet,
    pub joints: JointSet,
    // pub commands: Vec<Command>,
}

impl Scene {
    pub fn new() -> Scene {
        let gravity = Vector2::new(0.0, 10.0);
        let integration_parameters = IntegrationParameters::default();
        let broad_phase = BroadPhase::new();
        let narrow_phase = NarrowPhase::new();
        let mut bodies = RigidBodySet::new();
        let mut colliders = ColliderSet::new();
        let joints = JointSet::new();

        let cube_rb = RigidBodyBuilder::new_dynamic()
            .translation(100.0, 100.0)
            .mass(1.0)
            .build();
        let cube_handle = bodies.insert(cube_rb);
        let cube_collider = ColliderBuilder::cuboid(2.0, 2.0).build();
        colliders.insert(cube_collider, cube_handle, &mut bodies);

        Scene {
            gravity,
            integration_parameters,
            broad_phase,
            narrow_phase,
            bodies,
            colliders,
            joints,
        }
    }
}
