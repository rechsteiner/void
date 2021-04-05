pub struct Entity {
    pub rigidbody: RigidBody,
    pub shape: Shape,
    pub id: usize,
    pub physics_mode: PhysicsMode,
}

pub struct Shape {
    pub width: f32,
    pub height: f32,
}

pub struct Scene {
    pub entities: Vec<Entity>,
}

impl Scene {
    pub fn new(entities: Vec<Entity>) -> Scene {
        Scene { entities }
    }
}

pub struct RigidBody {
    pub transform: Transform,
    pub mass: f32,
}

pub struct Transform {
    pub position: Point,
    pub rotation: f32, // radians, so 2Pi == 360 deg
}

pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub enum PhysicsMode {
    Static,
    Dynamic,
}
