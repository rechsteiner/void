use hashbrown::HashMap;
use rapier2d::na::Vector2;
use std::f64::consts::PI;

use crate::components::gravity::GravitySource;
use crate::components::program::Program;
use crate::components::rigid_body::RigidBody;
use crate::interpreter::evaluator::Evaluator;
use crate::interpreter::object::{Command, Environment, Object};
use crate::resources::program_environment::{
    ProgramEnvironment, ProgramVariable, ProgramVariableValue,
};
use crate::systems::System;
use crate::world::World;

pub struct InterpreterSystem {}

impl InterpreterSystem {
    pub fn new() -> Self {
        InterpreterSystem {}
    }
}

impl System for InterpreterSystem {
    fn update(&mut self, world: &mut World) {
        // These are later used by rigid bodies to determine where
        // the closest gravity source is, i.e. what direction is "down"
        let gravity_sources: Vec<Vector2<f32>> = world
            .query_mut::<(&GravitySource, &RigidBody)>()
            .iter()
            .map(|(_, rb)| rb.transform.position)
            .collect();

        // Set current frame mission time
        let mission_time = world.start_timestamp.elapsed().as_millis();

        // Storing all program variables here so we can later give it
        // to the World without angering the borrow checker
        let mut current_program_variables: HashMap<String, ProgramVariableValue> = HashMap::new();

        for (program, rigid_body) in world.query_mut::<(&mut Program, &RigidBody)>() {
            let mut environment = Environment::new();
            let mut evaluator = Evaluator::new();

            let closest_gravity_source = get_closest_gravity_source(rigid_body, &gravity_sources);

            environment.set(
                String::from("SET_THRUST"),
                Object::Command {
                    function: |arguments| {
                        if arguments.len() != 1 {
                            return Result::Err(format!(
                                "wrong number of arguments. got={}, want=1",
                                arguments.len()
                            ));
                        }
                        match arguments[0].clone() {
                            Object::Integer(value) => Result::Ok(Command::SetThrust {
                                force: value as f64,
                            }),
                            Object::Float(value) => Result::Ok(Command::SetThrust { force: value }),
                            _ => Result::Err(format!(
                                "argument not supported, got {}",
                                arguments[0].name()
                            )),
                        }
                    },
                },
            );

            environment.set(
                String::from("SET_TORQUE"),
                Object::Command {
                    function: |arguments| {
                        if arguments.len() != 1 {
                            return Result::Err(format!(
                                "wrong number of arguments. got={}, want=1",
                                arguments.len()
                            ));
                        }
                        match arguments[0].clone() {
                            Object::Integer(value) => Result::Ok(Command::SetTorque {
                                force: value as f64,
                            }),
                            Object::Float(value) => Result::Ok(Command::SetTorque { force: value }),
                            _ => Result::Err(format!(
                                "argument not supported, got {}",
                                arguments[0].name()
                            )),
                        }
                    },
                },
            );

            // --- TIME ---
            environment.set(String::from("TIME"), Object::Integer(mission_time as isize));
            current_program_variables.insert(
                String::from("TIME"),
                ProgramVariableValue::Integer(mission_time as isize),
            );

            // --- ALTITUDE ---
            environment.set(
                String::from("ALTITUDE"),
                Object::Integer(closest_gravity_source.distance as isize),
            );
            current_program_variables.insert(
                String::from("ALTITUDE"),
                ProgramVariableValue::Integer(closest_gravity_source.distance as isize),
            );

            // --- ANGLE ---
            environment.set(
                String::from("ANGLE"),
                Object::Integer((closest_gravity_source.relative_angle * 57.2958) as isize), // multiply to convert radians to deg
            );
            current_program_variables.insert(
                String::from("ANGLE"),
                ProgramVariableValue::Integer(
                    (closest_gravity_source.relative_angle * 57.2958) as isize,
                ), // multiply to convert radians to deg
            );

            // --- ANG_VEL ---
            environment.set(
                String::from("ANG_VEL"),
                Object::Integer((rigid_body.angular_velocity * 57.2958) as isize), // multiply to convert radians to deg
            );
            current_program_variables.insert(
                String::from("ANG_VEL"),
                ProgramVariableValue::Integer((rigid_body.angular_velocity * 57.2958) as isize), // multiply to convert radians to deg
            );

            let _ = evaluator.eval(&program.program, &mut environment);

            program.commands = evaluator.commands;
        }

        // Write program variables to World so they can be shown in UI
        // TODO: This should probably be done somewhere else
        // And read directly from the already registered Environment variables
        let world_program_environment = world
            .get_resource_mut::<ProgramEnvironment>()
            .unwrap()
            .clear();

        for (name, value_object) in current_program_variables.iter() {
            world_program_environment.add_variable(ProgramVariable {
                name: String::from(name.clone()),
                value: value_object.clone(),
            });
        }
    }
}

// Helpers (will possibly be moved to a helper library)

fn angle_between_positions(a: Vector2<f32>, b: Vector2<f32>) -> f32 {
    let b_relative_to_a = Vector2::new(b.x - a.x, b.y - a.y);
    let up = Vector2::new(0.0, -1.0);

    let angle_from_up = Vector2::angle(&up, &b_relative_to_a);

    if b_relative_to_a.x > 0.0 {
        return angle_from_up;
    } else {
        return -angle_from_up;
    }
}

fn delta_angle(from: f32, to: f32) -> f32 {
    let total = 2.0 * PI as f32;

    let a = if from.is_sign_positive() {
        from
    } else {
        total - from.abs() // a = 348 (-12)
    };
    let b = if to.is_sign_positive() {
        to // b = 2
    } else {
        total - to.abs()
    };

    let x = if b > a {
        // 2 < 345
        total - (b - a)
    } else {
        a - b // x = (348 - 2) = 346
    };

    if x > (PI as f32) {
        // x > 360
        return total - x; // 360 - 346 = 14
    } else {
        return -x;
    }
}

struct ClosestGravitySourceParameters {
    distance: f32,
    relative_angle: f32,
}

fn get_closest_gravity_source(
    rigid_body: &RigidBody,
    gravity_sources: &Vec<Vector2<f32>>,
) -> ClosestGravitySourceParameters {
    let mut closest_gravity_position = None;
    let mut closest_gravity_distance = f32::INFINITY;

    for gravity_source_position in gravity_sources {
        let dist = (rigid_body.transform.position - gravity_source_position).magnitude();
        if dist < closest_gravity_distance {
            closest_gravity_position = Some(gravity_source_position);
            closest_gravity_distance = dist;
        }
    }

    let angle_to_closest_gravity_source = angle_between_positions(
        rigid_body.transform.position,
        *closest_gravity_position.unwrap(),
    );

    let relative_angle = delta_angle(
        rigid_body.transform.rotation,
        angle_to_closest_gravity_source,
    );

    ClosestGravitySourceParameters {
        distance: closest_gravity_distance,
        relative_angle,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delta_angle_neg160_to_160() {
        let from_angle: f32 = -160.0;
        let to_angle: f32 = 160.0;
        assert_eq!(
            delta_angle(from_angle.to_radians(), to_angle.to_radians())
                .to_degrees()
                .round(),
            -40.0
        );
    }

    #[test]
    fn delta_angle_20_to_neg10() {
        let from_angle: f32 = 20.0;
        let to_angle: f32 = -10.0;
        assert_eq!(
            delta_angle(from_angle.to_radians(), to_angle.to_radians())
                .to_degrees()
                .round(),
            -30.0
        );
    }

    #[test]
    fn delta_angle_110_to_160() {
        let from_angle: f32 = 110.0;
        let to_angle: f32 = 160.0;
        assert_eq!(
            delta_angle(from_angle.to_radians(), to_angle.to_radians())
                .to_degrees()
                .round(),
            50.0
        );
    }

    #[test]
    fn delta_angle_neg12_t0_2() {
        let from_angle: f32 = -12.0;
        let to_angle: f32 = 2.0;
        assert_eq!(
            delta_angle(from_angle.to_radians(), to_angle.to_radians())
                .to_degrees()
                .round(),
            14.0
        );
    }
}
