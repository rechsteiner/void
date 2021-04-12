use scene::{ColorRGBA, Entity, PhysicsMode, Point, RigidBody, Shape, Transform};
use simulation::Simulation;
use wasm_bindgen::prelude::*;
use web_sys::console;
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
                    vertices: vec![
                        Point { x: -10.0, y: -15.0 },
                        Point { x: 10.0, y: -15.0 },
                        Point { x: 15.0, y: 10.0 },
                        Point { x: -15.0, y: 10.0 },
                    ],
                    color: ColorRGBA {
                        r: 255,
                        g: 0,
                        b: 0,
                        a: 1.0,
                    },
                },
            },
            Entity {
                id: 1,
                rigidbody: RigidBody {
                    transform: Transform {
                        position: Point { x: 200.0, y: 160.0 },
                        rotation: 0.0,
                    },
                    mass: 0.5,
                },
                physics_mode: PhysicsMode::Dynamic,
                shape: Shape {
                    vertices: vec![
                        Point { x: -10.0, y: -10.0 },
                        Point { x: 10.0, y: -10.0 },
                        Point { x: 10.0, y: 10.0 },
                        Point { x: -10.0, y: 10.0 },
                    ],
                    color: ColorRGBA {
                        r: 0,
                        g: 255,
                        b: 0,
                        a: 1.0,
                    },
                },
            },
            // --- Static colliders ---
            Entity {
                id: 998,
                rigidbody: RigidBody {
                    transform: Transform {
                        position: Point { x: 200.0, y: 395.0 },
                        rotation: 0.0,
                    },
                    mass: 1.0,
                },
                physics_mode: PhysicsMode::Static,
                shape: Shape {
                    vertices: vec![
                        Point { x: -200.0, y: -5.0 },
                        Point { x: 200.0, y: -5.0 },
                        Point { x: 200.0, y: 5.0 },
                        Point { x: -200.0, y: 5.0 },
                    ],
                    color: ColorRGBA {
                        r: 0,
                        g: 0,
                        b: 0,
                        a: 1.0,
                    },
                },
            },
            Entity {
                id: 999,
                rigidbody: RigidBody {
                    transform: Transform {
                        position: Point { x: 80.0, y: 300.0 },
                        rotation: 0.6,
                    },
                    mass: 1.0,
                },
                physics_mode: PhysicsMode::Static,
                shape: Shape {
                    vertices: vec![
                        Point { x: -80.0, y: -5.0 },
                        Point { x: 80.0, y: -5.0 },
                        Point { x: 80.0, y: 5.0 },
                        Point { x: -80.0, y: 5.0 },
                    ],
                    color: ColorRGBA {
                        r: 0,
                        g: 0,
                        b: 0,
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

        // unsafe {
        //     console::log_1(
        //         &format!(
        //             "alt_vel: {}",
        //             self.simulation.get_entity_velocity(0).y as isize
        //         )
        //         .into(),
        //     );
        // }

        let _ = evaluator.eval(program, &mut environment);
        self.simulation.commands = evaluator.commands;
    }

    pub fn next_simulation_step(&mut self) {
        self.simulation.next_state();
        self.renderer.draw(&self.scene, &self.simulation);
    }
}

/*

let target_alt = 200;
let target_long = 200;
let torque_strength = 20000;

if (altitude < target_alt) {
  set_thrust(-2200);
} else {
  set_thrust(-1200);
}

if (altitude < (target_alt / 2)) {
  set_thrust(-3000);
}

if (longitude > target_long) {
  if (angle < -5) {
    set_torque(torque_strength);
  } else {
    set_torque(-torque_strength);
  }
} else {
  if (angle < 5) {
    set_torque(torque_strength);
  } else {
    set_torque(-torque_strength);
  }
}


let target_long = 100;
let target_alt = 300;
let torque_strength = 14000;

let future_alt = altitude - (alt_vel * 2);

if (future_alt < target_alt) {
    set_thrust(-3000)
}

let future_pos = longitude + (long_vel * 4);
let future_sep = target_long - future_pos;

if (future_pos > target_long) {
  if (angle < -2) {
    set_torque(torque_strength);
  } else {
    set_torque(-torque_strength);
  }
} else {
  if (angle < 2) {
    set_torque(torque_strength);
  } else {
    set_torque(-torque_strength);
  }
}

*/
