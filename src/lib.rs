mod components;
mod interpreter;
mod systems;
mod world;

extern crate console_error_panic_hook;
extern crate wasm_bindgen;

use components::physics_mode::PhysicsMode;
use components::program::Program;
use components::rigid_body::{RigidBody, Transform};
use components::shape::{ColorRGBA, Point, Shape};
use systems::interpreter::InterpreterSystem;
use systems::renderer::RenderSystem;
use systems::simulation::SimulationSystem;
use systems::System;
use wasm_bindgen::prelude::*;
use world::World;

#[wasm_bindgen]
pub struct Game {
    world: World,
    systems: Vec<Box<dyn System>>,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Game {
        console_error_panic_hook::set_once();
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

        // Register components

        // TODO: Would be nice if we didn't have to remember to register
        // components before using them. Right now it will panic.
        world.register_component::<RigidBody>();
        world.register_component::<Shape>();
        world.register_component::<PhysicsMode>();
        world.register_component::<Program>();

        // Entity 1

        world
            .create_entity()
            .with_component(RigidBody {
                transform: Transform {
                    position: Point { x: 200.0, y: 200.0 },
                    rotation: 0.0,
                },
                mass: 1.0,
                linear_velocity: Point { x: 0.0, y: 0.0 },
                angular_velocity: 0.0,
            })
            .with_component(Shape {
                vertices: vec![
                    Point { x: -10.0, y: -15.0 },
                    Point { x: 10.0, y: -15.0 },
                    Point { x: 15.0, y: 10.0 },
                    Point { x: -15.0, y: 10.0 },
                ],
                color: color_cyan,
            })
            .with_component(PhysicsMode::Dynamic)
            .with_component(Program::new());

        // Entity 2

        world
            .create_entity()
            .with_component(RigidBody {
                transform: Transform {
                    position: Point { x: 200.0, y: 160.0 },
                    rotation: 0.0,
                },
                mass: 0.2,
                linear_velocity: Point { x: 0.0, y: 0.0 },
                angular_velocity: 0.0,
            })
            .with_component(Shape {
                vertices: vec![
                    Point { x: -10.0, y: -10.0 },
                    Point { x: 10.0, y: -10.0 },
                    Point { x: 10.0, y: 10.0 },
                    Point { x: -10.0, y: 10.0 },
                ],
                color: color_orange,
            })
            .with_component(PhysicsMode::Dynamic);

        // Entity 3

        world
            .create_entity()
            .with_component(RigidBody {
                transform: Transform {
                    position: Point { x: 200.0, y: 400.0 },
                    rotation: 0.0,
                },
                mass: 1.0,
                linear_velocity: Point { x: 0.0, y: 0.0 },
                angular_velocity: 0.0,
            })
            .with_component(Shape {
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
            })
            .with_component(PhysicsMode::Static);

        // Entity 4

        world
            .create_entity()
            .with_component(RigidBody {
                transform: Transform {
                    position: Point { x: 0.0, y: 390.0 },
                    rotation: 0.0,
                },
                mass: 1.0,
                linear_velocity: Point { x: 0.0, y: 0.0 },
                angular_velocity: 0.0,
            })
            .with_component(Shape {
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
            })
            .with_component(PhysicsMode::Static);

        // Initialize the game with our systems

        Game {
            world: world,
            systems: vec![
                Box::new(InterpreterSystem::new()),
                Box::new(SimulationSystem::new()),
                Box::new(RenderSystem::new()),
            ],
        }
    }

    pub fn change_program(&mut self, input: String) {
        let mut programs = self.world.query_mut::<Program>();
        let mut program = programs.get_mut(0).unwrap();
        program.input = input;
    }

    pub fn tick(&mut self) {
        // self.renderer.move_viewport_toward_target(); // For smooth viewport motion

        for system in self.systems.iter_mut() {
            system.update(&mut self.world);
        }
    }

    pub fn move_render_viewport(&mut self, delta_x: f32, delta_y: f32, delta_zoom: f32) {
        // self.renderer.move_viewport_target(delta_x, delta_y, delta_zoom);
    }
}
