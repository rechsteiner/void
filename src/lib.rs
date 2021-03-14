use wasm_bindgen::prelude::*;
mod renderer;

extern crate wasm_bindgen;
use crate::renderer::Renderer;

#[wasm_bindgen]
pub struct Game {
    renderer: Renderer,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Game {
        Game {
            renderer: Renderer::new(),
        }
    }

    pub fn change_program(&self, input: String) {}
}
