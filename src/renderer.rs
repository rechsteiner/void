use std::f64;
use wasm_bindgen::JsCast;

use crate::simulation::SimulationState;

pub struct Renderer {
    context: web_sys::CanvasRenderingContext2d,
}

impl Renderer {
    pub fn new() -> Renderer {
        let window = web_sys::window().expect("no global `window` exists");
        let document: web_sys::Document =
            window.document().expect("should have a document on window");
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let context: web_sys::CanvasRenderingContext2d = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        Renderer { context: context }
    }

    pub fn draw(&self, state: &SimulationState) {
        self.context.clear_rect(0.0, 0.0, 400.0, 400.0);

        self.context.begin_path();

        for entity in state.entities() {
            // Draw the outer circle.
            self.context
                .arc(
                    entity.position.x,
                    entity.position.y,
                    10.0,
                    0.0,
                    f64::consts::PI * 2.0,
                )
                .unwrap();

            self.context.fill();
        }
    }
}
