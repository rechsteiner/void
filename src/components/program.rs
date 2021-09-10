use crate::interpreter::lexer::Lexer;
use crate::interpreter::object::Command;
use crate::interpreter::parser::Parser;

pub struct Program {
    pub program: crate::interpreter::ast::Program,
    pub commands: Vec<Command>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            program: crate::interpreter::ast::Program::new(),
            commands: vec![],
        }
    }

    pub fn update(&mut self, input: String) {
        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        self.program = program;
    }
}
