use crate::components::{program::Program, thrusters::Thrusters};
use crate::interpreter::object::Command;

use super::System;

/// Reads set_thrust commands in `Program` and sets current thrust level in `Thrusters` components
pub struct ThrusterSystem {}

impl ThrusterSystem {
    pub fn new() -> Self {
        ThrusterSystem {}
    }
}

impl System for ThrusterSystem {
    fn update(&mut self, world: &mut crate::world::World) {
        for (program, thrusters) in world.query_mut::<(&Program, &mut Thrusters)>() {
            // Read Command to set current throttle (0.0 - 1.0)
            for command in program.commands.iter() {
                if let Command::SetThrust { throttle } = command {
                    thrusters.set_throttle(*throttle);
                }
            }

            // Consume fuel based on current throttle
            // Simulation system is responsible for actually applying the force
            let remaining_fuel = thrusters.fuel - thrusters.get_current_fuel_consumption();
            if remaining_fuel > 0.0 {
                thrusters.fuel = remaining_fuel;
            } else {
                thrusters.fuel = 0.0;
            }
        }
    }
}
