use simulation::Simulation;
use wasm_bindgen::prelude::*;
mod interpreter;
mod renderer;
mod scene;
mod scenes;
mod setup_environment;
mod simulation;

extern crate wasm_bindgen;
use crate::interpreter::evaluator::Evaluator;
use crate::interpreter::lexer::Lexer;
use crate::interpreter::object::Environment;
use crate::interpreter::parser::Parser;
use crate::renderer::Renderer;
use crate::scene::Scene;
use crate::scenes::scene_1;

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

        setup_environment::set_up(&self, &mut environment);

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
