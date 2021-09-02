use crate::components::shape::Point;

pub struct RigidBody {
    pub transform: Transform,
    pub mass: f32,
    pub linear_velocity: Point,
    pub angular_velocity: f32,
}

pub struct Transform {
    pub position: Point,
    pub rotation: f32, // radians, so 2Pi == 360 deg
}
