use wasm_bindgen::prelude::*;
mod interpreter;
mod renderer;

extern crate wasm_bindgen;
use crate::interpreter::evaluator;
use crate::interpreter::lexer::Lexer;
use crate::interpreter::object::Environment;
use crate::interpreter::object::Object;
use crate::interpreter::parser::Parser;
use crate::renderer::Renderer;

#[wasm_bindgen]
pub struct Game {
    position: f64,
    renderer: Renderer,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Game {
        Game {
            position: 0.,
            renderer: Renderer::new(),
        }
    }

    pub fn change_program(&mut self, input: String) {
        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut environment = Environment::new();
        environment.set("position".to_string(), Object::Integer(20));
        let result = evaluator::eval(program, &mut environment);

        match result {
            Object::Integer(integer) => {
                self.position = integer as f64;
                self.renderer.draw(self.position);
            }
            _ => (),
        }
    }
}
