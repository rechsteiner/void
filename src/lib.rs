use scene::{Entity, PhysicsMode, Point, RigidBody, Shape, Transform};
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
                    width: 20.0,
                    height: 20.0,
                    color: String::from("#ff0000"),
                },
            },
            Entity {
                id: 1,
                rigidbody: RigidBody {
                    transform: Transform {
                        position: Point { x: 50.0, y: 50.0 },
                        rotation: 1.0,
                    },
                    mass: 1.0,
                },
                physics_mode: PhysicsMode::Dynamic,
                shape: Shape {
                    width: 20.0,
                    height: 20.0,
                    color: String::from("#00ff00"),
                },
            },
            Entity {
                id: 2,
                rigidbody: RigidBody {
                    transform: Transform {
                        position: Point { x: 60.0, y: 90.0 },
                        rotation: 2.0,
                    },
                    mass: 1.0,
                },
                physics_mode: PhysicsMode::Dynamic,
                shape: Shape {
                    width: 20.0,
                    height: 20.0,
                    color: String::from("#0000ff"),
                },
            },
            Entity {
                id: 3,
                rigidbody: RigidBody {
                    transform: Transform {
                        position: Point { x: 110.0, y: 150.0 },
                        rotation: 2.7,
                    },
                    mass: 1.0,
                },
                physics_mode: PhysicsMode::Dynamic,
                shape: Shape {
                    width: 40.0,
                    height: 20.0,
                    color: String::from("#ff00ff"),
                },
            },
            Entity {
                id: 4,
                rigidbody: RigidBody {
                    transform: Transform {
                        position: Point { x: 140.0, y: 50.0 },
                        rotation: 4.0,
                    },
                    mass: 1.0,
                },
                physics_mode: PhysicsMode::Dynamic,
                shape: Shape {
                    width: 20.0,
                    height: 20.0,
                    color: String::from("#00ffff"),
                },
            },
            Entity {
                id: 998,
                rigidbody: RigidBody {
                    transform: Transform {
                        position: Point { x: 0.0, y: 200.0 },
                        rotation: 0.6,
                    },
                    mass: 1.0,
                },
                physics_mode: PhysicsMode::Static,
                shape: Shape {
                    width: 200.0,
                    height: 10.0,
                    color: String::from("#000000"),
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
                    width: 400.0,
                    height: 10.0,
                    color: String::from("#000000"),
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

        // Does not appear to update continuously, only gets its value on init
        environment.set(
            String::from("altitude"),
            Object::Integer(self.simulation.get_entity_transform(0).position.y as isize),
        );

        let _ = evaluator.eval(program, &mut environment);
        self.simulation.commands = evaluator.commands;
    }

    pub fn next_simulation_step(&mut self) {
        self.simulation.next_state();
        self.renderer.draw(&self.scene, &self.simulation);
    }
}
