use simulation::Vec2;
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
use crate::simulation::SimulationState;

#[wasm_bindgen]
pub struct Game {
    forces: Vec2,
    renderer: Renderer,
    simulation_state: SimulationState,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Game {
        Game {
            forces: Vec2 { x: 0., y: 0. },
            renderer: Renderer::new(),
            simulation_state: SimulationState::new(),
        }
    }

    pub fn change_program(&mut self, input: String) {
        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut environment = Environment::new();
        environment.set(
            "position_x".to_string(),
            Object::Integer(self.simulation_state.position.x as isize),
        );
        environment.set(
            "position_y".to_string(),
            Object::Integer(self.simulation_state.position.y as isize),
        );
        let result = evaluator::eval(program, &mut environment);

        match result {
            Object::Integer(integer) => {
                self.forces = Vec2 {
                    x: 0.,
                    y: integer as f64,
                };
            }
            _ => (),
        }
    }

    pub fn next_simulation_step(&mut self) {
        self.simulation_state.next_state(&self.forces);
        self.renderer.draw(&self.simulation_state);
    }
}
