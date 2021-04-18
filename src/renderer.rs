use wasm_bindgen::{JsCast, JsValue};

use crate::{
    scene::{Point, Scene},
    simulation::Simulation,
};

struct Viewport {
    position: Point,
    target_position: Point,
    zoom: f32,
    target_zoom: f32,
}

pub struct Renderer {
    context: web_sys::CanvasRenderingContext2d,
    canvas: web_sys::HtmlCanvasElement,
    viewport: Viewport,
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

        let viewport = Viewport {
            position: Point { x: -400.0, y: 0.0 },
            zoom: 1.0,
            target_position: Point { x: -400.0, y: 0.0 },
            target_zoom: 1.0,
        };

        Renderer {
            context,
            canvas,
            viewport,
        }
    }

    pub fn move_viewport_target(&mut self, delta_x: f32, delta_y: f32, delta_zoom: f32) {
        self.viewport.target_position.x += delta_x;
        self.viewport.target_position.y += delta_y;
        self.viewport.target_zoom += delta_zoom;
    }

    pub fn move_viewport_toward_target(&mut self) {
        let smoothness_factor = 12.0; // Higher number gives more smooth motion

        let curr_pos = &mut self.viewport.position;
        let targ_pos = &mut self.viewport.target_position;

        let curr_zoom = &mut self.viewport.zoom;
        let targ_zoom = &mut self.viewport.target_zoom;

        // Move slightly toward the target (supposed to run on each frame)
        curr_pos.x += (targ_pos.x - curr_pos.x) / smoothness_factor;
        curr_pos.y += (targ_pos.y - curr_pos.y) / smoothness_factor;

        // Same with zoom
        *curr_zoom += (*targ_zoom - *curr_zoom) / smoothness_factor;
    }

    pub fn draw(&self, scene: &Scene, simulation: &Simulation) {
        self.context.clear_rect(
            0.0,
            0.0,
            self.canvas.width() as f64,
            self.canvas.height() as f64,
        );

        self.context.set_line_width(2.0);

        for entity in scene.entities.iter() {
            let transform = simulation.get_entity_transform(entity.id);

            // Move the sheet
            self.context
                .translate(
                    ((transform.position.x) * self.viewport.zoom - self.viewport.position.x) as f64,
                    ((transform.position.y) * self.viewport.zoom - self.viewport.position.y) as f64,
                )
                .unwrap();

            // Rotate the sheet
            self.context.rotate(transform.rotation as f64).unwrap();

            // Pick the right crayon
            self.context
                .set_stroke_style(&JsValue::from(format!("{}", &entity.shape.color))); // TODO: Might not be idiomatic

            // Draw by numbers
            self.context.begin_path();
            for vertex in &entity.shape.vertices {
                self.context.line_to(
                    (vertex.x * self.viewport.zoom) as f64,
                    (vertex.y * self.viewport.zoom) as f64,
                );
            }

            // Close the shape
            self.context.line_to(
                (entity.shape.vertices[0].x * self.viewport.zoom) as f64,
                (entity.shape.vertices[0].y * self.viewport.zoom) as f64,
            );

            self.context.stroke();

            // Debugging: dot at center of object
            self.context.stroke_rect(-1.0, -1.0, 2.0, 2.0);

            self.context.close_path();

            self.context.reset_transform().unwrap();
        }
    }
}
