use crate::components::program::Program;
use crate::components::rigid_body::RigidBody;
use crate::interpreter::evaluator::Evaluator;
use crate::interpreter::lexer::Lexer;
use crate::interpreter::object::{Command, Environment, Object};
use crate::interpreter::parser::Parser;
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
        let mut commands: Vec<Vec<Command>> = vec![];
        for (program, rigid_body) in world.query2::<Program, RigidBody>() {
            let lexer = Lexer::new(&program.input);
            let mut parser = Parser::new(lexer);
            let parsed_program = parser.parse_program();
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

            let _ = evaluator.eval(parsed_program, &mut environment);

            commands.push(evaluator.commands);
        }

        // TODO: Add support for querying multiple components mutably so we can
        // just update the program in the above for loop directly.
        for (index, program) in world
            .query_mut::<Program>()
            .unwrap()
            .into_iter()
            .enumerate()
        {
            program.commands = commands[index].clone();
        }
    }
}
