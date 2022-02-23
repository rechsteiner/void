use crate::components::program::Program;
use crate::components::shape::ColorRGBA;
use crate::interpreter::object::ProgramVariable;
use crate::resources::canvas::Canvas;
use crate::systems::System;
use crate::world::World;

pub struct InstrumentsRenderer {}

impl InstrumentsRenderer {
    pub fn new() -> InstrumentsRenderer {
        InstrumentsRenderer {}
    }
}

impl System for InstrumentsRenderer {
    fn update(&mut self, world: &mut World) {
        let canvas = world.get_resource::<Canvas>().unwrap();
        let font = "12px monospace";
        let color = ColorRGBA {
            r: 0,
            g: 0,
            b: 0,
            a: 1.0,
        };
        canvas.clear_rect(0.0, 0.0, canvas.width(), canvas.height());

        for program in world.query::<&Program>() {
            let mut y = 50.0;
            let variable_name_x = 50.0;
            let variable_value_x = 150.0;
            let spacing = 30.0;

            let variables = program.environment.get_variables();
            let mut vec: Vec<_> = variables.into_iter().collect();
            vec.sort_by(|x, y| x.0.cmp(&y.0));

            for (key, variable) in vec {
                canvas.draw_text(variable_name_x, y, font, &key, &color);

                match variable {
                    ProgramVariable::Integer(value) => {
                        canvas.draw_text(variable_value_x, y, font, &value.to_string(), &color);
                    }
                    ProgramVariable::Float(value) => {
                        canvas.draw_text(variable_value_x, y, font, &value.to_string(), &color);
                    }
                    ProgramVariable::Boolean(true) => {
                        canvas.draw_text(variable_value_x, y, font, "true", &color)
                    }
                    ProgramVariable::Boolean(false) => {
                        canvas.draw_text(variable_value_x, y, font, "false", &color);
                    }
                }

                y += spacing;
            }
        }
    }
}
