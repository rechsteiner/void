mod components;
mod entities;
mod helpers;
mod interpreter;
mod query;
mod resources;
mod scene;
mod scenes;
mod systems;
mod world;

extern crate console_error_panic_hook;
extern crate wasm_bindgen;

use components::program::Program;
use resources::viewport::Viewport;
use scene::Scene;
use scenes::scene_1;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Game {
    scene: Scene,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Game {
        console_error_panic_hook::set_once();
        let scene = scene_1::generate_scene();
        Game { scene }
    }

    pub fn change_program(&mut self, input: String) {
        let mut programs = self.scene.world.query_mut::<&mut Program>();
        let program = programs.get_mut(0).unwrap();
        program.update(input);
    }

    pub fn tick(&mut self) {
        let viewport = self.scene.world.get_resource_mut::<Viewport>().unwrap();
        viewport.move_toward_target();
        for system in self.scene.systems.iter_mut() {
            system.update(&mut self.scene.world);
        }
    }

    pub fn move_render_viewport(&mut self, delta_x: f32, delta_y: f32, delta_zoom: f32) {
        let viewport = self.scene.world.get_resource_mut::<Viewport>().unwrap();
        viewport.move_target(delta_x, delta_y, delta_zoom);
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
