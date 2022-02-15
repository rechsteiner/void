use wasm_bindgen::JsValue;

use crate::components::rigid_body::RigidBody;
use crate::components::shape::Shape;
use crate::components::text::Text;
use crate::resources::canvas::Canvas;
use crate::resources::viewport::Viewport;
use crate::systems::System;
use crate::world::World;

pub struct RenderSystem {}

impl RenderSystem {
    pub fn new() -> RenderSystem {
        RenderSystem {}
    }
}

impl System for RenderSystem {
    fn update(&mut self, world: &mut World) {
        let viewport = world.get_resource::<Viewport>().unwrap();
        let zoom = viewport.zoom;
        let position = viewport.position;
        let canvas = world.get_resource::<Canvas>().unwrap();
        let screen_height = canvas.height();
        let screen_width = canvas.width();

        canvas.clear_rect(0.0, 0.0, screen_width, screen_height);

        // Let line width be adaptive to zoom (but min 2.0)
        canvas.set_line_width(f64::max((zoom as f64) * 2.0, 2.0));

        for text in world.query::<&Text>() {
            canvas.set_font(&text.font);
            canvas.set_fill_style(&JsValue::from(format!("{}", text.color)));
            canvas.fill_text(
                &text.content,
                text.position.x as f64,
                text.position.y as f64,
            );
        }

        for (rigid_body, shape) in world.query::<(&RigidBody, &Shape)>() {
            let transform = &rigid_body.transform;
            // Move the sheet
            // Magical math to get zoom with focal point in center of screen
            canvas.translate(
                (((transform.position.x - position.x) * zoom) + (screen_width as f32 / 2.0)) as f64,
                (((transform.position.y - position.y) * zoom) + (screen_height as f32 / 2.0))
                    as f64,
            );

            // Rotate the sheet
            canvas.rotate(transform.rotation as f64);

            // Pick the right crayon
            canvas.set_stroke_style(&JsValue::from(format!("{}", shape.color))); // TODO: Might not be idiomatic

            // Draw by numbers
            canvas.begin_path();
            for vertex in &shape.vertices {
                canvas.line_to((vertex.x * zoom) as f64, (vertex.y * zoom) as f64);
            }

            // Close the shape
            canvas.line_to(
                (shape.vertices[0].x * zoom) as f64,
                (shape.vertices[0].y * zoom) as f64,
            );

            canvas.close_path();
            canvas.stroke();

            // Debugging: dot at center of object
            canvas.stroke_rect(-1.0, -1.0, 2.0, 2.0);
            canvas.close_path();
            canvas.reset_transform();
        }
    }
}
