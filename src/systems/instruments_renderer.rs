use crate::components::program::Program;
use crate::interpreter::object::ProgramVariable;
use crate::resources::canvas::Canvas;
use crate::systems::System;
use crate::world::World;
use wasm_bindgen::JsValue;

pub struct InstrumentsRenderer {}

impl InstrumentsRenderer {
    pub fn new() -> InstrumentsRenderer {
        InstrumentsRenderer {}
    }
}

impl System for InstrumentsRenderer {
    fn update(&mut self, world: &mut World) {
        let canvas = world.get_resource::<Canvas>().unwrap();
        canvas.clear_rect(0.0, 0.0, canvas.width(), canvas.height());
        canvas.set_font("12px monospace");
        canvas.set_fill_style(&JsValue::from(format!("{}", "white")));

        for program in world.query::<&Program>() {
            let mut y = 50.0;
            let variable_name_x = 50.0;
            let variable_value_x = 120.0;
            let spacing = 30.0;

            let variables = program.environment.get_variables();
            let mut vec: Vec<_> = variables.into_iter().collect();
            vec.sort_by(|x, y| x.0.cmp(&y.0));

            for (key, variable) in vec {
                canvas.fill_text(&key, variable_name_x, y);
                match variable {
                    ProgramVariable::Integer(value) => {
                        canvas.fill_text(&value.to_string(), variable_value_x, y)
                    }
                    ProgramVariable::Float(value) => {
                        canvas.fill_text(&value.to_string(), variable_value_x, y)
                    }
                    ProgramVariable::Boolean(true) => canvas.fill_text("true", variable_value_x, y),
                    ProgramVariable::Boolean(false) => {
                        canvas.fill_text("false", variable_value_x, y)
                    }
                }

                y += spacing;
            }
        }
    }
}
