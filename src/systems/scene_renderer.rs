use crate::components::rigid_body::RigidBody;
use crate::components::shape::Shape;
use crate::resources::canvas::{Canvas, Path};
use crate::resources::viewport::Viewport;
use crate::systems::System;
use crate::world::World;

pub struct SceneRenderer {}

impl SceneRenderer {
    pub fn new() -> SceneRenderer {
        SceneRenderer {}
    }
}

impl System for SceneRenderer {
    fn update(&mut self, world: &mut World) {
        let viewport = world.get_resource::<Viewport>().unwrap();
        let zoom = viewport.zoom;
        let position = viewport.position;
        let canvas = world.get_resource::<Canvas>().unwrap();
        let screen_height = canvas.height();
        let screen_width = canvas.width();

        // Let line width be adaptive to zoom (but min 2.0)
        let line_width = f32::max(zoom * 2.0, 2.0);

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

            // Create path for each vertex
            let mut path = Path::new();
            for vertex in &shape.vertices {
                path.line_to(vertex.x * zoom, vertex.y * zoom);
            }

            // Close the shape
            path.line_to(shape.vertices[0].x * zoom, shape.vertices[0].y * zoom);

            // Draw the path
            canvas.draw_path(path, line_width, &shape.color);

            // Debugging: dot at center of object
            canvas.draw_rectangle(
                -line_width / 2.,
                -line_width / 2.,
                line_width,
                line_width,
                &shape.color,
            );

            canvas.reset_transform();
        }
    }
}
