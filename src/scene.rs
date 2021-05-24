use std::{collections::HashMap, fmt};

pub struct Entity {
    pub rigidbody: RigidBody,
    pub shape: Shape,
    pub physics_mode: PhysicsMode,
}

pub struct Shape {
    pub vertices: Vec<Point>,
    pub color: ColorRGBA, //Mosly for debug
    pub is_sensor: bool,
}

pub enum ObjectiveType {
    Target { target_id: usize, ship_id: usize },
    Orbit,
}

pub struct Objective {
    pub objective_type: ObjectiveType,
}

pub struct Scene {
    pub entities: HashMap<usize, Entity>,
    pub objectives: Vec<Objective>,
}

impl Scene {
    pub fn new(entities: HashMap<usize, Entity>, objectives: Vec<Objective>) -> Scene {
        Scene {
            entities,
            objectives,
        }
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
