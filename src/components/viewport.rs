use super::shape::Point;

pub struct Viewport {
    pub position: Point,
    pub target_position: Point,
    pub zoom: f32,
    pub target_zoom: f32,
}

impl Viewport {
    // Indirect way for user to move viewport
    pub fn move_target(&mut self, delta_x: f32, delta_y: f32, delta_zoom: f32) {
        // Input values are scaled by zoom value, for more natural and accurate movement
        self.target_position.x += delta_x / self.zoom;
        self.target_position.y += delta_y / self.zoom;

        if self.target_zoom + delta_zoom > 0.1 {
            self.target_zoom += delta_zoom * self.target_zoom;
        } else {
            self.target_zoom = 0.1;
        }
    }

    // Should run on every frame – moves the viewport toward the target gradually
    pub fn move_toward_target(&mut self) {
        let smoothness_factor = 12.0; // Higher number gives more smooth motion

        let curr_pos = &mut self.position;
        let targ_pos = &mut self.target_position;

        let curr_zoom = &mut self.zoom;
        let targ_zoom = &mut self.target_zoom;

        // Move slightly toward the target (supposed to run on each frame)
        curr_pos.x += (targ_pos.x - curr_pos.x) / smoothness_factor;
        curr_pos.y += (targ_pos.y - curr_pos.y) / smoothness_factor;

        // Same with zoom
        *curr_zoom += (*targ_zoom - *curr_zoom) / smoothness_factor;
    }
}
