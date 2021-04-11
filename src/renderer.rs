use std::f64;
use wasm_bindgen::{JsCast, JsValue};

use crate::{scene::Scene, simulation::Simulation};

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

        Renderer { context }
    }

    pub fn draw(&self, scene: &Scene, simulation: &Simulation) {
        self.context.clear_rect(0.0, 0.0, 400.0, 400.0);

        for entity in scene.entities.iter() {
            let transform = simulation.get_entity_transform(entity.id);

            self.context
                .translate((transform.position.x) as f64, (transform.position.y) as f64)
                .unwrap();
            self.context.rotate(transform.rotation as f64).unwrap();

            self.context
                .set_stroke_style(&JsValue::from(&entity.shape.color));

            self.context.begin_path();

            self.context.stroke_rect(
                -(entity.shape.width / 2.0) as f64,
                -(entity.shape.height / 2.0) as f64,
                (entity.shape.width) as f64,
                (entity.shape.height) as f64,
            );

            self.context.close_path();
            self.context.reset_transform().unwrap();
        }
    }
}
