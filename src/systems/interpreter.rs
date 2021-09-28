use rapier2d::na::Vector2;
use web_sys::console;

use crate::components::gravity::GravitySource;
use crate::components::program::Program;
use crate::components::rigid_body::RigidBody;
use crate::helpers::math::{angle_between_positions, delta_angle};
use crate::interpreter::evaluator::Evaluator;
use crate::interpreter::object::{Command, Object};
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

        for (program, rigid_body) in world.query_mut::<(&mut Program, &RigidBody)>() {
            let mut evaluator = Evaluator::new();

            let closest_gravity_source = get_closest_gravity_source(rigid_body, &gravity_sources);

            program.environment.clear();
            program.environment.set(
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
                                throttle: (value as f64),
                            }),
                            Object::Float(value) => {
                                Result::Ok(Command::SetThrust { throttle: value })
                            }
                            _ => Result::Err(format!(
                                "argument not supported, got {}",
                                arguments[0].name()
                            )),
                        }
                    },
                },
            );

            program.environment.set(
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
            program
                .environment
                .set(String::from("TIME"), Object::Integer(mission_time as isize));

            // --- ALTITUDE ---
            program.environment.set(
                String::from("ALTITUDE"),
                Object::Float(closest_gravity_source.distance as f64),
            );

            // --- ANGLE ---
            program.environment.set(
                String::from("ANGLE"),
                Object::Float((closest_gravity_source.relative_angle * 57.2958) as f64), // multiply to convert radians to deg
            );

            // --- ANG_VEL ---
            program.environment.set(
                String::from("ANG_VEL"),
                Object::Float((rigid_body.angular_velocity * 57.2958) as f64), // multiply to convert radians to deg
            );

            let result = evaluator.eval(&program.program, &mut program.environment);
            if let Object::Error(err) = result {
                console::log_1(&format!("{:?}", err).into())
            }
            program.commands = evaluator.commands;
        }
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
