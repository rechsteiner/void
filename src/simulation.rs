pub struct SimulationState {
    pub position: Vec2,
    pub velocity: Vec2,

    air_friction: f64,
    gravity: f64,

    safe_area: f64,
    canvas_width: f64,
    canvas_height: f64,
}

pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl SimulationState {
    pub fn new() -> SimulationState {
        SimulationState {
            position: Vec2 { x: 100., y: 100. },
            velocity: Vec2 { x: 10., y: 0. },

            air_friction: 1.004,
            gravity: 0.2,
            safe_area: 10.0,
            canvas_width: 400.0,
            canvas_height: 400.0,
        }
    }

    pub fn next_state(&mut self, forces: &Vec2) {
        let padded_width = self.canvas_width - self.safe_area;
        let padded_height = self.canvas_height - self.safe_area;

        let vel_x = if self.position.x < padded_width && self.position.x > self.safe_area {
            (self.velocity.x + forces.x * 0.1) / self.air_friction
        } else {
            (-self.velocity.x + forces.x * 0.1) / self.air_friction
        };

        let vel_y = if self.position.y < padded_height && self.position.y > self.safe_area {
            (self.velocity.y + self.gravity + forces.y * 0.1) / self.air_friction
        } else {
            (-self.velocity.y + self.gravity + forces.y * 0.1) / self.air_friction
        };

        // Calculate position
        let pos_x = match self.position.x + vel_x {
            x if x > padded_width => padded_width,
            x if x < self.safe_area => self.safe_area,
            x => x,
        };

        let pos_y = match self.position.y + vel_y {
            y if y > padded_height => padded_height,
            y if y < self.safe_area => self.safe_area,
            y => y,
        };

        // Set own position and velocity
        self.velocity = Vec2 { x: vel_x, y: vel_y };
        self.position = Vec2 {
            x: pos_x + vel_x,
            y: pos_y + vel_y,
        };
    }
}
