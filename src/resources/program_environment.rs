use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ProgramEnvironment {
    variables: Vec<ProgramVariable>,
}

impl ProgramEnvironment {
    pub fn new() -> Self {
        ProgramEnvironment { variables: vec![] }
    }

    pub fn clear(&mut self) -> &mut Self {
        self.variables = vec![];
        self
    }

    pub fn add_variable(&mut self, program_variable: ProgramVariable) -> &mut Self {
        self.variables.push(program_variable);
        self
    }

    pub fn get_variables(&self) -> &Vec<ProgramVariable> {
        &self.variables
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ProgramVariable {
    pub name: String,
    pub value: ProgramVariableValue,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ProgramVariableValue {
    Integer(isize),
    Boolean(bool),
    String(String),
    Function,
}
