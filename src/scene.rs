use std::fmt;

pub struct Entity {
    pub rigidbody: RigidBody,
    pub shape: Shape,
    pub id: usize,
    pub physics_mode: PhysicsMode,
}

pub struct Shape {
    pub vertices: Vec<Point>,
    // pub width: f32,
    // pub height: f32,
    pub color: ColorRGBA, //Mosly for debug
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

pub struct ColorRGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: f32,
}

impl fmt::Display for ColorRGBA {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rgba({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
}
