use crate::interpreter::lexer::Lexer;
use crate::interpreter::object::{Command, Environment, RuntimeError};
use crate::interpreter::parser::{Parser, ParserError};

pub struct Program {
    pub program: crate::interpreter::ast::Program,
    pub environment: Environment,
    pub commands: Vec<Command>,
    pub parser_errors: Vec<ParserError>,
    pub runtime_errors: Vec<RuntimeError>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            program: crate::interpreter::ast::Program::new(),
            environment: Environment::new(),
            commands: vec![],
            parser_errors: vec![],
            runtime_errors: vec![],
        }
    }

    pub fn update(&mut self, input: String) {
        let lexer = Lexer::new(&input);
        let mut parser = Parser::new(lexer);
        let (program, errors) = parser.parse_program();
        self.program = program;
        self.runtime_errors = vec![];
        self.parser_errors = errors;
    }
}
