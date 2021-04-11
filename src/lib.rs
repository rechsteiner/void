use scene::{Entity, PhysicsMode, Point, RigidBody, Shape, Transform};
use simulation::Simulation;
use wasm_bindgen::prelude::*;
mod interpreter;
mod renderer;
mod scene;
mod simulation;

extern crate wasm_bindgen;
use crate::interpreter::evaluator;
use crate::interpreter::lexer::Lexer;
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
                        position: Point { x: 100.0, y: 100.0 },
                        rotation: 1.8,
                    },
                    mass: 1.0,
                },
                physics_mode: PhysicsMode::Dynamic,
                shape: Shape {
                    width: 20.0,
                    height: 20.0,
                },
            },
            Entity {
                id: 1,
                rigidbody: RigidBody {
                    transform: Transform {
                        position: Point { x: 200.0, y: 100.0 },
                        rotation: 2.0,
                    },
                    mass: 1.0,
                },
                physics_mode: PhysicsMode::Dynamic,
                shape: Shape {
                    width: 20.0,
                    height: 20.0,
                },
            },
            Entity {
                id: 2,
                rigidbody: RigidBody {
                    transform: Transform {
                        position: Point { x: 300.0, y: 100.0 },
                        rotation: 2.8,
                    },
                    mass: 1.0,
                },
                physics_mode: PhysicsMode::Dynamic,
                shape: Shape {
                    width: 20.0,
                    height: 20.0,
                },
            },
            Entity {
                id: 4,
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

        // let mut index: usize = 0;

        // environment.set(
        //     String::from("set_thrust"),
        //     Object::Builtin {
        //         function: |arguments| {
        //             index = 2;
        //             // self.simulation.scene.commands.insert(Command.SetThrust(2));
        //             Object::Integer(1)
        //         },
        //     },
        // );

        // let result = evaluator::eval(program, &mut environment);
    }

    pub fn next_simulation_step(&mut self) {
        self.simulation.next_state();
        self.renderer.draw(&self.scene, &self.simulation);
    }
}
