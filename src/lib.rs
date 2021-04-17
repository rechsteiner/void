use scene::{ColorRGBA, Entity, PhysicsMode, Point, RigidBody, Shape, Transform};
use simulation::Simulation;
use wasm_bindgen::prelude::*;
mod interpreter;
mod renderer;
mod scene;
mod simulation;

extern crate wasm_bindgen;
use crate::interpreter::evaluator::Evaluator;
use crate::interpreter::lexer::Lexer;
use crate::interpreter::object::Command;
use crate::interpreter::object::Environment;
use crate::interpreter::object::Object;
use crate::interpreter::parser::Parser;
use crate::renderer::Renderer;
use crate::scene::Scene;

#[wasm_bindgen]
pub struct Game {
    renderer: Renderer,
    simulation: Simulation,
    scene: Scene,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Game {
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

        let entities = vec![
            Entity {
                id: 0,
                rigidbody: RigidBody {
                    transform: Transform {
                        position: Point { x: 200.0, y: 200.0 },
                        rotation: 0.0,
                    },
                    mass: 1.0,
                },
                physics_mode: PhysicsMode::Dynamic,
                shape: Shape {
                    vertices: vec![
                        Point { x: -10.0, y: -15.0 },
                        Point { x: 10.0, y: -15.0 },
                        Point { x: 15.0, y: 10.0 },
                        Point { x: -15.0, y: 10.0 },
                    ],
                    color: color_cyan,
                },
            },
            Entity {
                id: 1,
                rigidbody: RigidBody {
                    transform: Transform {
                        position: Point { x: 200.0, y: 160.0 },
                        rotation: 0.0,
                    },
                    mass: 0.2,
                },
                physics_mode: PhysicsMode::Dynamic,
                shape: Shape {
                    vertices: vec![
                        Point { x: -10.0, y: -10.0 },
                        Point { x: 10.0, y: -10.0 },
                        Point { x: 10.0, y: 10.0 },
                        Point { x: -10.0, y: 10.0 },
                    ],
                    color: color_orange,
                },
            },
            // --- Static colliders ---
            Entity {
                id: 998,
                rigidbody: RigidBody {
                    transform: Transform {
                        position: Point { x: 200.0, y: 400.0 },
                        rotation: 0.0,
                    },
                    mass: 1.0,
                },
                physics_mode: PhysicsMode::Static,
                shape: Shape {
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
            Entity {
                id: 999,
                rigidbody: RigidBody {
                    transform: Transform {
                        position: Point { x: 0.0, y: 390.0 },
                        rotation: 0.0,
                    },
                    mass: 1.0,
                },
                physics_mode: PhysicsMode::Static,
                shape: Shape {
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
        ];

        let scene = Scene::new(entities);

        Game {
            renderer: Renderer::new(),
            simulation: Simulation::new(&scene),
            scene,
        }
    }

    pub fn change_program(&mut self, input: String) {
        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut environment = Environment::new();
        let mut evaluator = Evaluator::new();

        environment.set(
            String::from("set_thrust"),
            Object::Command {
                function: |arguments| {
                    if arguments.len() != 1 {
                        return Result::Err(format!(
                            "wrong number of arguments. got={}, want=1",
                            arguments.len()
                        ));
                    }
                    match arguments[0].clone() {
                        Object::Integer(value) => Result::Ok(Command::SetThrust { force: value }),
                        _ => Result::Err(format!(
                            "argument not supported, got {}",
                            arguments[0].name()
                        )),
                    }
                },
            },
        );

        environment.set(
            String::from("set_torque"),
            Object::Command {
                function: |arguments| {
                    if arguments.len() != 1 {
                        return Result::Err(format!(
                            "wrong number of arguments. got={}, want=1",
                            arguments.len()
                        ));
                    }
                    match arguments[0].clone() {
                        Object::Integer(value) => Result::Ok(Command::SetTorque { force: value }),
                        _ => Result::Err(format!(
                            "argument not supported, got {}",
                            arguments[0].name()
                        )),
                    }
                },
            },
        );

        environment.set(
            String::from("altitude"),
            Object::Integer(400 - self.simulation.get_entity_transform(0).position.y as isize), // 400 is height of canvas
        );

        environment.set(
            String::from("longitude"),
            Object::Integer(self.simulation.get_entity_transform(0).position.x as isize),
        );

        environment.set(
            String::from("angle"),
            Object::Integer((self.simulation.get_entity_transform(0).rotation * 58.122) as isize), // multiply to convert radians to deg
        );

        environment.set(
            String::from("long_vel"),
            Object::Integer((self.simulation.get_entity_velocity(0).x) as isize), // multiply to convert radians to deg
        );

        environment.set(
            String::from("alt_vel"),
            Object::Integer((self.simulation.get_entity_velocity(0).y) as isize), // multiply to convert radians to deg
        );

        environment.set(
            String::from("ang_vel"),
            Object::Integer((self.simulation.get_entity_radial_velocity(0)) as isize), // multiply to convert radians to deg
        );

        let _ = evaluator.eval(program, &mut environment);
        self.simulation.commands = evaluator.commands;
    }

    pub fn next_simulation_step(&mut self) {
        self.renderer.move_viewport_toward_target(); // For smooth viewport motion
        self.simulation.next_state();
        self.renderer.draw(&self.scene, &self.simulation);
    }

    pub fn move_render_viewport(&mut self, delta_x: f32, delta_y: f32, delta_zoom: f32) {
        self.renderer
            .move_viewport_target(delta_x, delta_y, delta_zoom);
    }
}
