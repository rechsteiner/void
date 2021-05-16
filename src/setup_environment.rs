use crate::{
    interpreter::object::{Command, Environment, Object},
    Game,
};

pub fn set_up(game: &Game, environment: &mut Environment) {
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
        Object::Integer(400 - game.simulation.get_entity_transform(0).position.y as isize), // 400 is height of canvas
    );

    environment.set(
        String::from("LONGITUDE"),
        Object::Integer(game.simulation.get_entity_transform(0).position.x as isize),
    );

    environment.set(
        String::from("ANGLE"),
        Object::Integer((game.simulation.get_entity_transform(0).rotation * 58.122) as isize), // multiply to convert radians to deg
    );

    environment.set(
        String::from("LONG_VEL"),
        Object::Integer((game.simulation.get_entity_velocity(0).x) as isize), // multiply to convert radians to deg
    );

    environment.set(
        String::from("ALT_VEL"),
        Object::Integer((game.simulation.get_entity_velocity(0).y) as isize), // multiply to convert radians to deg
    );

    environment.set(
        String::from("ANG_VEL"),
        Object::Integer((game.simulation.get_entity_radial_velocity(0)) as isize), // multiply to convert radians to deg
    );
}
