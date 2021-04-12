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
                .set_stroke_style(&JsValue::from(format!("{}", &entity.shape.color)));

            self.context.begin_path();
            for vertex in &entity.shape.vertices {
                self.context.line_to(vertex.x as f64, vertex.y as f64);
            }

            // Close the shape
            self.context.line_to(
                entity.shape.vertices[0].x as f64,
                entity.shape.vertices[0].y as f64,
            );
            self.context.stroke();

            // Debugging: dot at center of object
            self.context.stroke_rect(-1.0, -1.0, 2.0, 2.0);

            self.context.close_path();

            self.context.reset_transform().unwrap();
        }
    }
}
