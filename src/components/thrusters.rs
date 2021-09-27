use rapier2d::na::Vector2;

// The overall thruster system of a spacecraft

pub struct Thrusters {
    pub fuel: f64,
    pub fuel_max: f64,
    throttle: f64,
    thrusters: Vec<Thruster>,
}

impl Thrusters {
    pub fn new(fuel: f64, fuel_max: f64, thrusters: Vec<Thruster>) -> Self {
        Thrusters {
            fuel,
            fuel_max,
            throttle: 0.0,
            thrusters,
        }
    }

    /// Set throttle level – must be between 0.0 and 1.0
    pub fn set_throttle(&mut self, throttle_level: f64) {
        self.throttle = throttle_level.clamp(0.0, 1.0);
    }

    pub fn get_throttle(&self) -> f64 {
        self.throttle
    }

    pub fn get_total_thrust(&self) -> f64 {
        self.thrusters.iter().fold(0.0, |acc, thruster| {
            thruster.get_thrust(self.throttle) + acc
        })
    }

    pub fn get_current_fuel_consumption(&self) -> f64 {
        self.thrusters.iter().fold(0.0, |acc, thruster| {
            thruster.fuel_consumption_per_force * thruster.get_thrust(self.throttle) + acc
        })
    }

    pub fn get_thrusters(&self) -> &Vec<Thruster> {
        &self.thrusters
    }
}

// A single thruster

pub struct Thruster {
    pub max_thrust_force: f64,
    pub fuel_consumption_per_force: f64,
    pub rotation: f64,
    pub position: Vector2<f64>,
}

impl Thruster {
    /// Get thrust strength at given throttle level. Throttle scales from 0-1, and maps from 0 to thruster `max_thrust_force`.
    pub fn get_thrust(&self, throttle_level: f64) -> f64 {
        self.max_thrust_force * throttle_level
    }
}
