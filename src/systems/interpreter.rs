use crate::components::program::Program;
use crate::components::rigid_body::RigidBody;
use crate::interpreter::evaluator::Evaluator;
use crate::interpreter::object::{Command, Environment, Object};
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
        for (program, rigid_body) in world.query_mut::<(&mut Program, &RigidBody)>() {
            let mut environment = Environment::new();
            let mut evaluator = Evaluator::new();

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

            environment.set(
                String::from("ALTITUDE"),
                Object::Integer(400 - rigid_body.transform.position.y as isize), // 400 is height of canvas
            );

            environment.set(
                String::from("LONGITUDE"),
                Object::Integer(rigid_body.transform.position.x as isize),
            );

            environment.set(
                String::from("ANGLE"),
                Object::Integer((rigid_body.transform.rotation * 58.122) as isize), // multiply to convert radians to deg
            );

            environment.set(
                String::from("LONG_VEL"),
                Object::Integer(rigid_body.linear_velocity.x as isize), // multiply to convert radians to deg
            );

            environment.set(
                String::from("ALT_VEL"),
                Object::Integer(rigid_body.linear_velocity.y as isize), // multiply to convert radians to deg
            );

            environment.set(
                String::from("ANG_VEL"),
                Object::Integer(rigid_body.angular_velocity as isize), // multiply to convert radians to deg
            );

            let _ = evaluator.eval(&program.program, &mut environment);

            program.commands = evaluator.commands;
        }
    }
}
