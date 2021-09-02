mod components;
mod ecs;
mod interpreter;
mod renderer;
mod simulation;
mod systems;

extern crate wasm_bindgen;

use components::physics_mode::PhysicsMode;
use components::rigid_body::{RigidBody, Transform};
use components::shape::{ColorRGBA, Point, Shape};
use ecs::{System, World};
use renderer::Renderer;
use simulation::Simulation;
use systems::interpreter::InterpreterSystem;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Game {
    world: World,
    systems: Vec<Box<dyn System>>,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Game {
        let mut world = World::new();
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

        // Entity 1

        world.insert_component(RigidBody {
            transform: Transform {
                position: Point { x: 200.0, y: 200.0 },
                rotation: 0.0,
            },
            mass: 1.0,
            linear_velocity: Point { x: 0.0, y: 0.0 },
            angular_velocity: 0.0,
        });

        world.insert_component(Shape {
            vertices: vec![
                Point { x: -10.0, y: -15.0 },
                Point { x: 10.0, y: -15.0 },
                Point { x: 15.0, y: 10.0 },
                Point { x: -15.0, y: 10.0 },
            ],
            color: color_cyan,
        });

        world.insert_component(PhysicsMode::Dynamic);

        // Entity 2

        world.insert_component(RigidBody {
            transform: Transform {
                position: Point { x: 200.0, y: 160.0 },
                rotation: 0.0,
            },
            mass: 0.2,
            linear_velocity: Point { x: 0.0, y: 0.0 },
            angular_velocity: 0.0,
        });

        world.insert_component(Shape {
            vertices: vec![
                Point { x: -10.0, y: -10.0 },
                Point { x: 10.0, y: -10.0 },
                Point { x: 10.0, y: 10.0 },
                Point { x: -10.0, y: 10.0 },
            ],
            color: color_orange,
        });

        world.insert_component(PhysicsMode::Dynamic);

        // Entity 3

        world.insert_component(RigidBody {
            transform: Transform {
                position: Point { x: 200.0, y: 400.0 },
                rotation: 0.0,
            },
            mass: 1.0,
            linear_velocity: Point { x: 0.0, y: 0.0 },
            angular_velocity: 0.0,
        });

        world.insert_component(Shape {
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
        });

        world.insert_component(PhysicsMode::Static);

        // Entity 4

        world.insert_component(RigidBody {
            transform: Transform {
                position: Point { x: 0.0, y: 390.0 },
                rotation: 0.0,
            },
            mass: 1.0,
            linear_velocity: Point { x: 0.0, y: 0.0 },
            angular_velocity: 0.0,
        });

        world.insert_component(Shape {
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
        });

        world.insert_component(PhysicsMode::Static);
        Game {
            world: world,
            systems: vec![
                Box::new(InterpreterSystem::new()),
                Box::new(Simulation::new()),
                Box::new(Renderer::new()),
            ],
        }
    }

    pub fn change_program(&mut self, input: String) {
        self.world.program = input;
    }

    pub fn tick(&mut self) {
        // self.renderer.move_viewport_toward_target(); // For smooth viewport motion

        for system in &self.systems {
            system.update(&mut self.world);
        }
    }

    pub fn move_render_viewport(&mut self, delta_x: f32, delta_y: f32, delta_zoom: f32) {
        // self.renderer.move_viewport_target(delta_x, delta_y, delta_zoom);
    }
}
