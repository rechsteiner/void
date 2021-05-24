use std::collections::HashMap;

use crate::scene::{
    ColorRGBA, Entity, Objective, ObjectiveType, PhysicsMode, Point, RigidBody, Scene, Shape,
    Transform,
};

pub fn generate_scene() -> Scene {
    let color_cyan = ColorRGBA {
        r: 0,
        g: 255,
        b: 209,
        a: 1.0,
    };

    let color_orange = ColorRGBA {
        r: 255,
        g: 153,
        b: 34,
        a: 1.0,
    };

    let mut entities = HashMap::new();

    entities.insert(
        0,
        Entity {
            rigidbody: RigidBody {
                transform: Transform {
                    position: Point { x: 200.0, y: 200.0 },
                    rotation: 0.0,
                },
                mass: 1.0,
            },
            physics_mode: PhysicsMode::Dynamic,
            shape: Shape {
                is_sensor: false,
                vertices: vec![
                    Point { x: -10.0, y: -15.0 },
                    Point { x: 10.0, y: -15.0 },
                    Point { x: 15.0, y: 10.0 },
                    Point { x: -15.0, y: 10.0 },
                ],
                color: color_cyan,
            },
        },
    );

    entities.insert(
        1,
        Entity {
            rigidbody: RigidBody {
                transform: Transform {
                    position: Point { x: 200.0, y: 160.0 },
                    rotation: 0.0,
                },
                mass: 0.2,
            },
            physics_mode: PhysicsMode::Dynamic,
            shape: Shape {
                is_sensor: false,
                vertices: vec![
                    Point { x: -10.0, y: -10.0 },
                    Point { x: 10.0, y: -10.0 },
                    Point { x: 10.0, y: 10.0 },
                    Point { x: -10.0, y: 10.0 },
                ],
                color: color_orange,
            },
        },
    );
    entities.insert(
        2,
        Entity {
            rigidbody: RigidBody {
                transform: Transform {
                    position: Point { x: 200.0, y: 160.0 },
                    rotation: 0.0,
                },
                mass: 0.2,
            },
            physics_mode: PhysicsMode::Static,
            shape: Shape {
                is_sensor: true,
                vertices: vec![
                    Point { x: -10.0, y: -10.0 },
                    Point { x: 10.0, y: -10.0 },
                    Point { x: 10.0, y: 10.0 },
                    Point { x: -10.0, y: 10.0 },
                ],
                color: ColorRGBA {
                    r: 255,
                    g: 255,
                    b: 255,
                    a: 1.0,
                },
            },
        },
    );

    // --- Static colliders ---
    entities.insert(
        998,
        Entity {
            rigidbody: RigidBody {
                transform: Transform {
                    position: Point { x: 200.0, y: 400.0 },
                    rotation: 0.0,
                },
                mass: 1.0,
            },
            physics_mode: PhysicsMode::Static,
            shape: Shape {
                is_sensor: false,
                vertices: vec![
                    Point {
                        x: -300.0,
                        y: -10.0,
                    },
                    Point { x: 300.0, y: -10.0 },
                    Point { x: 300.0, y: 0.0 },
                    Point { x: -300.0, y: 0.0 },
                ],
                color: ColorRGBA {
                    r: 255,
                    g: 255,
                    b: 255,
                    a: 1.0,
                },
            },
        },
    );

    entities.insert(
        999,
        Entity {
            rigidbody: RigidBody {
                transform: Transform {
                    position: Point { x: 0.0, y: 390.0 },
                    rotation: 0.0,
                },
                mass: 1.0,
            },
            physics_mode: PhysicsMode::Static,
            shape: Shape {
                is_sensor: false,
                vertices: vec![
                    Point {
                        x: -40.0,
                        y: -120.0,
                    },
                    Point { x: 40.0, y: -120.0 },
                    Point { x: 40.0, y: 0.0 },
                    Point { x: -40.0, y: 0.0 },
                ],
                color: ColorRGBA {
                    r: 255,
                    g: 255,
                    b: 255,
                    a: 1.0,
                },
            },
        },
    );

    let objectives = vec![Objective {
        objective_type: ObjectiveType::Target {
            ship_id: 0,
            target_id: 2,
        },
    }];

    Scene::new(entities, objectives)
}
