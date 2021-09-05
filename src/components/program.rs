use crate::interpreter::object::Command;

pub struct Program {
    pub input: String,
    pub commands: Vec<Command>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            input: String::new(),
            commands: vec![],
        }
    }
}
