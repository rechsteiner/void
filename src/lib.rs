use std::ops::Index;

use scene::Command;
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
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Game {
        let scene = Scene::new();
        Game {
            renderer: Renderer::new(),
            simulation: Simulation::new(scene),
        }
    }

    pub fn change_program(&mut self, input: String) {
        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut environment = Environment::new();

        // environment.set(
        //     String::from("set_thrust"),
        //     Object::Builtin {
        //         function: |arguments| {
        //             self.simulation.scene.commands.insert(Command.SetThrust(2));
        //         },
        //     },
        // );

        let result = evaluator::eval(program, &mut environment);
    }

    pub fn next_simulation_step(&mut self) {
        self.simulation.next_state();
        self.renderer.draw(&self.simulation.scene);
    }
}
