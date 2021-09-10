use wasm_bindgen::{JsCast, JsValue};

use crate::components::rigid_body::RigidBody;
use crate::components::shape::Shape;
use crate::components::viewport::Viewport;
use crate::systems::System;
use crate::world::World;

pub struct RenderSystem {
    context: web_sys::CanvasRenderingContext2d,
    canvas: web_sys::HtmlCanvasElement,
}

impl RenderSystem {
    pub fn new() -> RenderSystem {
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

        RenderSystem { context, canvas }
    }
}

impl System for RenderSystem {
    fn update(&mut self, world: &mut World) {
        let screen_height = self.canvas.height() as f32;
        let screen_width = self.canvas.width() as f32;

        self.context
            .clear_rect(0.0, 0.0, screen_width as f64, screen_height as f64);

        let viewport = world.get_resource::<Viewport>().unwrap();

        self.context
            .set_line_width(f64::max((viewport.zoom as f64) * 2.0, 2.0)); // Let line width be adaptive to zoom (but min 2.0)

        for (rigid_body, shape) in world.query::<(&RigidBody, &Shape)>() {
            let transform = &rigid_body.transform;
            // Move the sheet
            // Magical math to get zoom with focal point in center of screen
            self.context
                .translate(
                    (((transform.position.x - viewport.position.x) * viewport.zoom)
                        + (screen_width / 2.0)) as f64,
                    (((transform.position.y - viewport.position.y) * viewport.zoom)
                        + (screen_height / 2.0)) as f64,
                )
                .unwrap();

            // Rotate the sheet
            self.context.rotate(transform.rotation as f64).unwrap();

            // Pick the right crayon
            self.context
                .set_stroke_style(&JsValue::from(format!("{}", shape.color))); // TODO: Might not be idiomatic

            // Draw by numbers
            self.context.begin_path();
            for vertex in &shape.vertices {
                self.context.line_to(
                    (vertex.x * viewport.zoom) as f64,
                    (vertex.y * viewport.zoom) as f64,
                );
            }

            // Close the shape
            self.context.line_to(
                (shape.vertices[0].x * viewport.zoom) as f64,
                (shape.vertices[0].y * viewport.zoom) as f64,
            );

            self.context.stroke();

            // Debugging: dot at center of object
            self.context.stroke_rect(-1.0, -1.0, 2.0, 2.0);

            self.context.close_path();

            self.context.reset_transform().unwrap();
        }
    }
}
