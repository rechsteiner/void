mod components;
mod entities;
mod interpreter;
mod query;
mod scene;
mod scenes;
mod systems;
mod world;

extern crate console_error_panic_hook;
extern crate wasm_bindgen;

use components::program::Program;
use components::viewport::Viewport;
use scene::Scene;
use scenes::scene_1;
use systems::interpreter::InterpreterSystem;
use systems::renderer::RenderSystem;
use systems::simulation::SimulationSystem;
use systems::System;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Game {
    scene: Scene,
    systems: Vec<Box<dyn System>>,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Game {
        console_error_panic_hook::set_once();
        let scene = scene_1::generate_scene();
        Game {
            scene: scene,
            systems: vec![
                Box::new(InterpreterSystem::new()),
                Box::new(SimulationSystem::new()),
                Box::new(RenderSystem::new()),
            ],
        }
    }

    pub fn change_program(&mut self, input: String) {
        let mut programs = self.scene.world.query_mut::<Program>();
        let mut program = programs.get_mut(0).unwrap();
        program.input = input;
    }

    pub fn tick(&mut self) {
        let mut viewports = self.scene.world.query_mut::<Viewport>();
        let viewport = viewports.get_mut(0).unwrap();
        viewport.move_toward_target();
        for system in self.systems.iter_mut() {
            system.update(&mut self.scene.world);
        }
    }

    pub fn move_render_viewport(&mut self, delta_x: f32, delta_y: f32, delta_zoom: f32) {
        let mut viewports = self.scene.world.query_mut::<Viewport>();
        let viewport = viewports.get_mut(0).unwrap();
        viewport.move_target(delta_x, delta_y, delta_zoom);
    }
}
