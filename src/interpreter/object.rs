use crate::interpreter::ast::BlockStatement;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

#[derive(PartialEq, Debug, Clone)]
pub enum Command {
    SetThrust { force: f64 },
    SetTorque { force: f64 },
}

pub type CommandFn = fn(Vec<Object>) -> Result<Command, String>;

#[derive(PartialEq, Debug, Clone)]
pub enum Object {
    Integer(isize),
    Float(f64),
    Boolean(bool),
    Return(Box<Object>),
    Error(String),
    Function {
        parameters: Vec<String>,
        body: BlockStatement,
        environment: Environment,
    },
    Command {
        function: CommandFn,
    },
    Null,
}

impl Object {
    pub fn name(&self) -> String {
        match self {
            Object::Integer(_) => String::from("integer"),
            Object::Float(_) => String::from("float"),
            Object::Boolean(_) => String::from("boolean"),
            Object::Return(_) => String::from("return"),
            Object::Error(_) => String::from("error"),
            Object::Function { .. } => String::from("function"),
            Object::Command { .. } => String::from("command"),
            Object::Null => String::from("null"),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Environment {
    store: HashMap<String, Object>,
    outer: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            store: HashMap::new(),
            outer: None,
        }
    }

    pub fn extend(environment: Environment) -> Environment {
        Environment {
            store: HashMap::new(),
            outer: Some(Box::new(environment)),
        }
    }

    pub fn get(&self, key: &str) -> Option<Object> {
        match self.store.get(key) {
            Some(value) => Some(value.clone()),
            None => match &self.outer {
                Some(outer) => outer.get(key).clone(),
                None => None,
            },
        }
    }

    pub fn set(&mut self, key: String, value: Object) {
        self.store.insert(key, value);
    }

    pub fn get_variables(&self) -> HashMap<String, ProgramVariable> {
        self.store
            .iter()
            .filter_map(|(key, value)| match value {
                Object::Integer(int) => Some((key.clone(), ProgramVariable::Integer(*int))),
                Object::Boolean(bool) => Some((key.clone(), ProgramVariable::Boolean(*bool))),
                Object::Float(float) => Some((key.clone(), ProgramVariable::Float(*float))),
                _ => None,
            })
            .collect()
    }

    pub fn clear(&mut self) {
        self.store.clear();
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ProgramVariable {
    Integer(isize),
    Float(f64),
    Boolean(bool),
}

// Formatting

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Integer(value) => write!(f, "{}", value),
            Object::Float(value) => write!(f, "{}", value),
            Object::Boolean(value) => write!(f, "{}", value),
            Object::Return(expression) => write!(f, "{}", *expression),
            Object::Error(error) => write!(f, "Error: {}", error),
            Object::Function {
                parameters, body, ..
            } => {
                write!(f, "({}) {}", parameters.join(","), body)
            }
            Object::Command { .. } => {
                write!(f, "command function")
            }
            Object::Null => write!(f, "null"),
        }
    }
}
