use rapier2d::na::Vector2;

pub struct RigidBody {
    pub id: usize,
    pub transform: Transform,
    pub mass: f32,
    pub linear_velocity: Vector2<f32>,
    pub angular_velocity: f32,
    pub physics_mode: PhysicsMode,
}

pub struct Transform {
    pub position: Vector2<f32>,
    pub rotation: f32, // radians, so 2Pi == 360 deg
}

pub enum PhysicsMode {
    Static,
    Dynamic,
}
