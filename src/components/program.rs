use crate::interpreter::ast::Program as ParsedProgram;
use crate::interpreter::lexer::Lexer;
use crate::interpreter::object::{Command, Environment};
use crate::interpreter::parser::{Parser, ParserError};

pub struct Program {
    pub program: Result<ParsedProgram, Vec<ParserError>>,
    pub environment: Environment,
    pub commands: Vec<Command>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            program: Err(vec![]),
            environment: Environment::new(),
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
