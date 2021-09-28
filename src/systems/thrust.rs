use crate::components::{program::Program, thrusters::Thrusters};
use crate::interpreter::object::{Command, Object};

use super::System;

/// Reads SET_THRUST commands from `Program` and sets current thrust level in `Thrusters` components.
/// Also reduces fuel according to current thrust level.
pub struct ThrusterSystem {}

impl ThrusterSystem {
    pub fn new() -> Self {
        ThrusterSystem {}
    }
}

impl System for ThrusterSystem {
    fn update(&mut self, world: &mut crate::world::World) {
        for (program, thrusters) in world.query_mut::<(&mut Program, &mut Thrusters)>() {
            // Read Command to set current throttle (0.0 - 1.0)
            for command in program.commands.iter() {
                if let Command::SetThrust { throttle } = command {
                    thrusters.set_throttle(*throttle);
                }
            }

            // TODO: Make `FUEL` variable accessible in Environment
            // Currently it only renders in UI, but isn't registered by interpreter
            program
                .environment
                .set(String::from("FUEL"), Object::Float(thrusters.fuel));

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
