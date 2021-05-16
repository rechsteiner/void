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

mod scenes;
pub use crate::scenes::scene_1;

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
        let scene = scene_1::generate_scene();
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
            String::from("SET_THRUST"),
            Object::Command {
                function: |arguments| {
                    if arguments.len() != 1 {
                        return Result::Err(format!(
                            "wrong number of arguments. got={}, want=1",
                            arguments.len()
                        ));
                    }
                    match arguments[0].clone() {
                        Object::Integer(value) => Result::Ok(Command::SetThrust {
                            force: value as f64,
                        }),
                        Object::Float(value) => Result::Ok(Command::SetThrust { force: value }),
                        _ => Result::Err(format!(
                            "argument not supported, got {}",
                            arguments[0].name()
                        )),
                    }
                },
            },
        );

        environment.set(
            String::from("SET_TORQUE"),
            Object::Command {
                function: |arguments| {
                    if arguments.len() != 1 {
                        return Result::Err(format!(
                            "wrong number of arguments. got={}, want=1",
                            arguments.len()
                        ));
                    }
                    match arguments[0].clone() {
                        Object::Integer(value) => Result::Ok(Command::SetTorque {
                            force: value as f64,
                        }),
                        Object::Float(value) => Result::Ok(Command::SetTorque { force: value }),
                        _ => Result::Err(format!(
                            "argument not supported, got {}",
                            arguments[0].name()
                        )),
                    }
                },
            },
        );

        environment.set(
            String::from("ALTITUDE"),
            Object::Integer(400 - self.simulation.get_entity_transform(0).position.y as isize), // 400 is height of canvas
        );

        environment.set(
            String::from("LONGITUDE"),
            Object::Integer(self.simulation.get_entity_transform(0).position.x as isize),
        );

        environment.set(
            String::from("ANGLE"),
            Object::Integer((self.simulation.get_entity_transform(0).rotation * 58.122) as isize), // multiply to convert radians to deg
        );

        environment.set(
            String::from("LONG_VEL"),
            Object::Integer((self.simulation.get_entity_velocity(0).x) as isize), // multiply to convert radians to deg
        );

        environment.set(
            String::from("ALT_VEL"),
            Object::Integer((self.simulation.get_entity_velocity(0).y) as isize), // multiply to convert radians to deg
        );

        environment.set(
            String::from("ANG_VEL"),
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
