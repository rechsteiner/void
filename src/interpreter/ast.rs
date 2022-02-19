use crate::interpreter::token::Token;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Let {
        identifier: String,
        expression: Expression,
    },
    Return {
        expression: Expression,
    },
    // Expression statements is a statement that consists solely of one
    // expression. It used to handle cases where we write expression in the
    // top-level code. E.g the second line here:
    //
    // let x = 5;
    // x + 10;
    Expression {
        expression: Expression,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub struct BlockStatement {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Identifier(String),
    Int(isize),
    Float(f64),
    Boolean(bool),
    // Need to use a Box here to avoid an infinitely large size on the
    // Expression type. Using Box means we just store a pointer to the
    // associated expression.
    Prefix {
        operator: Operator,
        right: Box<Expression>,
    },
    Infix {
        operator: Operator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    If {
        condition: Box<Expression>,
        consequence: BlockStatement,
        alternative: Option<BlockStatement>,
    },
    Function {
        parameters: Vec<String>,
        body: BlockStatement,
    },
    Call {
        function: Box<Expression>,
        arguments: Vec<Expression>,
    },
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Operator {
    Plus,
    Minus,
    Not,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
}

impl Operator {
    pub fn from(token: &Token) -> Option<Operator> {
        match token {
            Token::Bang => Some(Operator::Not),
            Token::Plus => Some(Operator::Plus),
            Token::Minus => Some(Operator::Minus),
            Token::Slash => Some(Operator::Divide),
            Token::Asterisk => Some(Operator::Multiply),
            Token::Equal => Some(Operator::Equal),
            Token::NotEqual => Some(Operator::NotEqual),
            Token::LessThan => Some(Operator::LessThan),
            Token::GreaterThan => Some(Operator::GreaterThan),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Program {
        Program { statements: vec![] }
    }
}

// Formatting

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for stmt in &self.statements {
            write!(f, "{}", stmt)?;
        }
        Ok(())
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Identifier(name) => write!(f, "{}", name),
            Expression::Int(literal) => write!(f, "{}", literal),
            Expression::Float(literal) => write!(f, "{}", literal),
            Expression::Boolean(boolean) => write!(f, "{}", boolean),
            Expression::Prefix { operator, right } => write!(f, "({}{})", operator, right),
            Expression::Infix {
                operator,
                left,
                right,
            } => write!(f, "({} {} {})", left, operator, right),
            Expression::If {
                condition,
                consequence,
                alternative,
            } => {
                write!(f, "if {} {}", condition, consequence)?;

                if let Some(alternative) = alternative {
                    write!(f, " else {}", alternative)?;
                }

                Ok(())
            }
            Expression::Function { parameters, body } => {
                write!(f, "({}) {}", parameters.join(","), body)
            }
            Expression::Call {
                function,
                arguments,
            } => {
                write!(
                    f,
                    "{}({})",
                    function,
                    arguments
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
        }
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Let {
                identifier,
                expression,
            } => write!(f, "let {} = {};", identifier, expression),
            Statement::Return { expression } => write!(f, "return {};", expression),
            Statement::Expression { expression } => write!(f, "{}", expression),
        }
    }
}

impl fmt::Display for BlockStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for stmt in &self.statements {
            write!(f, "{{ {} }}", stmt)?;
        }
        Ok(())
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operator::Plus => write!(f, "+"),
            Operator::Minus => write!(f, "-"),
            Operator::Not => write!(f, "!"),
            Operator::Multiply => write!(f, "*"),
            Operator::Divide => write!(f, "/"),
            Operator::Equal => write!(f, "=="),
            Operator::NotEqual => write!(f, "!="),
            Operator::LessThan => write!(f, "<"),
            Operator::GreaterThan => write!(f, ">"),
        }
    }
}
