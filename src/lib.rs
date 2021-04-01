use simulation::SimulationState;
use wasm_bindgen::prelude::*;
mod interpreter;
mod renderer;
mod simulation;

extern crate wasm_bindgen;
use crate::interpreter::evaluator;
use crate::interpreter::lexer::Lexer;
use crate::interpreter::object::Environment;
use crate::interpreter::object::Object;
use crate::interpreter::parser::Parser;
use crate::renderer::Renderer;

#[wasm_bindgen]
pub struct Game {
    renderer: Renderer,
    simulation_state: SimulationState,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Game {
        Game {
            renderer: Renderer::new(),
            simulation_state: SimulationState::new(),
        }
    }

    pub fn change_program(&mut self, input: String) {
        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut environment = Environment::new();
        let result = evaluator::eval(program, &mut environment);
    }

    pub fn next_simulation_step(&mut self) {
        self.simulation_state.next_state();
        self.renderer.draw(&self.simulation_state);
    }
}
