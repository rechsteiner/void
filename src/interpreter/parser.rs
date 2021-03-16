use crate::interpreter::ast::BlockStatement;
use crate::interpreter::ast::Expression;
use crate::interpreter::ast::Operator;
use crate::interpreter::ast::Program;
use crate::interpreter::ast::Statement;
use crate::interpreter::lexer::Lexer;
use crate::interpreter::token::Token;

type PrefixParseFn<'a> = fn(&mut Parser<'a>) -> Option<Expression>;
type InfixParseFn<'a> = fn(&mut Parser<'a>, Expression) -> Option<Expression>;

#[derive(PartialEq, PartialOrd, Debug)]
enum Precedence {
    Lowest,
    Equals,      // ==
    LessGreater, // > or <
    Sum,         // +
    Product,     // *
    Prefix,      // -X or !X
    Call,        // myFunction(x)
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer) -> Parser {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();

        Parser {
            lexer: lexer,
            current_token: current_token,
            peek_token: peek_token,
            errors: vec![],
        }
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program::new();

        while self.current_token != Token::Eof {
            let statement = self.parse_statement();

            if let Some(statement) = statement {
                program.statements.push(statement);
            }

            self.next_token();
        }

        program
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token {
            Token::Let => self.parse_let_statement(),
            Token::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        match self.peek_token.clone() {
            Token::Identifier(identifier) => {
                self.next_token();

                if !self.expect_peek(Token::Assign) {
                    return None;
                }

                self.next_token();
                if let Some(expression) = self.parse_expression(Precedence::Lowest) {
                    if !self.expect_peek(Token::Semicolon) {
                        return None;
                    }

                    let statement = Statement::Let {
                        identifier: identifier.to_string(),
                        expression: expression,
                    };

                    return Some(statement);
                }

                None
            }
            _ => None,
        }
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        self.next_token();

        if let Some(expression) = self.parse_expression(Precedence::Lowest) {
            let statement = Statement::Return {
                expression: expression,
            };

            if self.peek_token == Token::Semicolon {
                self.next_token();
            }

            return Some(statement);
        }

        None
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        if let Some(expression) = self.parse_expression(Precedence::Lowest) {
            if self.peek_token == Token::Semicolon {
                self.next_token();
            }
            let statement = Statement::Expression {
                expression: expression,
            };
            Some(statement)
        } else {
            None
        }
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        if let Some(prefix_fn) = self.prefix_parse_fn() {
            if let Some(left_expression) = prefix_fn(self) {
                let mut left_expression = left_expression;
                while self.peek_token != Token::Semicolon && precedence < self.peek_precedence() {
                    match self.infix_parse_fn() {
                        Some(infix_fn) => {
                            self.next_token();
                            match infix_fn(self, left_expression.clone()) {
                                Some(right_expression) => {
                                    left_expression = right_expression;
                                }
                                None => return Some(left_expression),
                            }
                        }
                        None => return Some(left_expression),
                    }
                }

                Some(left_expression)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn prefix_parse_fn(&mut self) -> Option<PrefixParseFn<'a>> {
        match &self.current_token {
            Token::Identifier(_) => Some(Parser::parse_identifier),
            Token::Int(_) => Some(Parser::parse_integer),
            Token::Bang | Token::Minus => Some(Parser::parse_prefix_expression),
            Token::True | Token::False => Some(Parser::parse_boolean),
            Token::LeftParen => Some(Parser::parse_grouped_expression),
            Token::If => Some(Parser::parse_if_expression),
            Token::Function => Some(Parser::parse_function_expression),
            _ => {
                let error = format!(
                    "No prefix parse function found for {:?}",
                    self.current_token
                );
                self.errors.push(error);
                None
            }
        }
    }

    fn infix_parse_fn(&mut self) -> Option<InfixParseFn<'a>> {
        match &self.peek_token {
            Token::Plus
            | Token::Minus
            | Token::Slash
            | Token::Asterisk
            | Token::Equal
            | Token::NotEqual
            | Token::LessThan
            | Token::GreaterThan => Some(Parser::parse_infix_expression),
            Token::LeftParen => Some(Parser::parse_call_expression),
            _ => None,
        }
    }

    fn parse_identifier(&mut self) -> Option<Expression> {
        match &self.current_token {
            Token::Identifier(name) => Some(Expression::Identifier(name.clone())),
            _ => None,
        }
    }

    fn parse_integer(&mut self) -> Option<Expression> {
        match &self.current_token {
            Token::Int(value) => match value.parse::<isize>() {
                Ok(literal) => Some(Expression::Int(literal)),
                Err(_) => {
                    let error = format!("Could not parse {} as integer", value);
                    self.errors.push(error);
                    None
                }
            },
            _ => None,
        }
    }

    fn parse_boolean(&mut self) -> Option<Expression> {
        match &self.current_token {
            Token::True => Some(Expression::Boolean(true)),
            Token::False => Some(Expression::Boolean(false)),
            _ => None,
        }
    }

    fn parse_prefix_expression(&mut self) -> Option<Expression> {
        let operator = Operator::from(&self.current_token).unwrap();
        // The current token is either ! or - here, so we move to the next token
        // and parse that as an expression.
        self.next_token();

        match self.parse_expression(Precedence::Prefix) {
            Some(expression) => Some(Expression::Prefix {
                operator: operator,
                right: Box::new(expression),
            }),
            None => None,
        }
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Option<Expression> {
        let operator = Operator::from(&self.current_token).unwrap();
        let precedence = self.current_precedence();
        self.next_token();

        match self.parse_expression(precedence) {
            Some(right) => Some(Expression::Infix {
                operator: operator,
                left: Box::new(left),
                right: Box::new(right),
            }),
            None => None,
        }
    }

    fn parse_grouped_expression(&mut self) -> Option<Expression> {
        self.next_token();

        if let Some(expression) = self.parse_expression(Precedence::Lowest) {
            if self.expect_peek(Token::RightParen) {
                Some(expression)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn parse_if_expression(&mut self) -> Option<Expression> {
        if !self.expect_peek(Token::LeftParen) {
            return None;
        }

        self.next_token();

        if let Some(condition) = self.parse_expression(Precedence::Lowest) {
            if !self.expect_peek(Token::RightParen) {
                return None;
            }
            if !self.expect_peek(Token::LeftBrackets) {
                return None;
            }
            let consequence = self.parse_block_statement();

            if self.peek_token == Token::Else {
                self.next_token();

                if !self.expect_peek(Token::LeftBrackets) {
                    return None;
                }

                let alternative = self.parse_block_statement();

                return Some(Expression::If {
                    condition: Box::new(condition),
                    consequence: consequence,
                    alternative: Some(alternative),
                });
            } else {
                return Some(Expression::If {
                    condition: Box::new(condition),
                    consequence: consequence,
                    alternative: None,
                });
            }
        }

        None
    }

    fn parse_function_expression(&mut self) -> Option<Expression> {
        if !self.expect_peek(Token::LeftParen) {
            return None;
        }

        if let Some(parameters) = self.parse_function_parameters() {
            if !self.expect_peek(Token::LeftBrackets) {
                return None;
            }
            let body = self.parse_block_statement();
            Some(Expression::Function {
                parameters: parameters,
                body: body,
            })
        } else {
            None
        }
    }

    fn parse_function_parameters(&mut self) -> Option<Vec<String>> {
        if self.peek_token == Token::RightParen {
            self.next_token();
            return Some(vec![]);
        }

        self.next_token();

        let mut identifiers: Vec<String> = vec![];

        if let Some(Expression::Identifier(identifier)) = self.parse_identifier() {
            identifiers.push(identifier);

            while self.peek_token == Token::Comma {
                self.next_token();
                self.next_token();

                if let Some(Expression::Identifier(identifier)) = self.parse_identifier() {
                    identifiers.push(identifier);
                }
            }
        }

        if !self.expect_peek(Token::RightParen) {
            return None;
        }

        Some(identifiers)
    }

    fn parse_block_statement(&mut self) -> BlockStatement {
        self.next_token();
        let mut statements: Vec<Statement> = vec![];

        while self.current_token != Token::RightBrackets && self.current_token != Token::Eof {
            if let Some(statement) = self.parse_statement() {
                statements.push(statement);
            }
            self.next_token();
        }

        BlockStatement {
            statements: statements,
        }
    }

    fn parse_call_expression(&mut self, function: Expression) -> Option<Expression> {
        let arguments = self.parse_call_arguments();
        Some(Expression::Call {
            function: Box::new(function),
            arguments: arguments,
        })
    }

    fn parse_call_arguments(&mut self) -> Vec<Expression> {
        let mut arguments = vec![];

        if self.peek_token == Token::RightParen {
            self.next_token();
            return arguments;
        }

        self.next_token();
        if let Some(expression) = self.parse_expression(Precedence::Lowest) {
            arguments.push(expression);

            while self.peek_token == Token::Comma {
                self.next_token();
                self.next_token();

                if let Some(expression) = self.parse_expression(Precedence::Lowest) {
                    arguments.push(expression);
                }
            }

            if !self.expect_peek(Token::RightParen) {
                return vec![];
            }
        }

        arguments
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn expect_peek(&mut self, token: Token) -> bool {
        if self.peek_token == token {
            self.next_token();
            true
        } else {
            self.peek_error(token);
            false
        }
    }

    fn peek_error(&mut self, token: Token) {
        let error = format!(
            "Expected next token to be {:?}, got {:?} instead",
            token, self.peek_token
        );
        self.errors.push(error);
    }

    fn peek_precedence(&self) -> Precedence {
        self.precedence_for_token(self.peek_token.clone())
    }

    fn current_precedence(&self) -> Precedence {
        self.precedence_for_token(self.current_token.clone())
    }

    fn precedence_for_token(&self, token: Token) -> Precedence {
        match token {
            Token::Equal => Precedence::Equals,
            Token::NotEqual => Precedence::Equals,
            Token::LessThan => Precedence::LessGreater,
            Token::GreaterThan => Precedence::LessGreater,
            Token::Plus => Precedence::Sum,
            Token::Minus => Precedence::Sum,
            Token::Slash => Precedence::Product,
            Token::Asterisk => Precedence::Product,
            Token::LeftParen => Precedence::Call,
            _ => Precedence::Lowest,
        }
    }
}

#[test]
fn test_new() {
    let lexer = Lexer::new("let a = 1;");
    let parser = Parser::new(lexer);

    assert_eq!(parser.current_token, Token::Let);
    assert_eq!(parser.peek_token, Token::Identifier(String::from("a")));
}

#[test]
fn test_let_statement() {
    let input = "
    let x = 5;
    let y = 10;
    let foobar = 838383;
    ";

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    assert_eq!(parser.errors, vec![] as Vec<String>);
    assert_eq!(
        program.statements,
        vec![
            Statement::Let {
                identifier: String::from("x"),
                expression: Expression::Int(5),
            },
            Statement::Let {
                identifier: String::from("y"),
                expression: Expression::Int(10),
            },
            Statement::Let {
                identifier: String::from("foobar"),
                expression: Expression::Int(838383),
            }
        ]
    );
}

#[test]
fn test_return_statement() {
    let input = "
    return 5;
    return 10;
    return 993322;
    ";

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    assert_eq!(parser.errors, vec![] as Vec<String>);
    assert_eq!(
        program.statements,
        vec![
            Statement::Return {
                expression: Expression::Int(5),
            },
            Statement::Return {
                expression: Expression::Int(10),
            },
            Statement::Return {
                expression: Expression::Int(993322),
            }
        ]
    );
}

#[test]
fn test_identifier_expression() {
    let input = "foobar";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    assert_eq!(parser.errors, vec![] as Vec<String>);
    assert_eq!(
        program.statements,
        vec![Statement::Expression {
            expression: Expression::Identifier(String::from("foobar"))
        }]
    );
}

#[test]
fn test_integer_expression() {
    let input = "5;";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    assert_eq!(parser.errors, vec![] as Vec<String>);
    assert_eq!(
        program.statements,
        vec![Statement::Expression {
            expression: Expression::Int(5)
        }]
    );
}

#[test]
fn test_prefix_expression() {
    let input = "
    !5;
    -15;
    ";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    assert_eq!(parser.errors, vec![] as Vec<String>);
    assert_eq!(
        program.statements,
        vec![
            Statement::Expression {
                expression: Expression::Prefix {
                    operator: Operator::Not,
                    right: Box::new(Expression::Int(5))
                }
            },
            Statement::Expression {
                expression: Expression::Prefix {
                    operator: Operator::Minus,
                    right: Box::new(Expression::Int(15))
                }
            }
        ]
    );
}

#[test]
fn test_infix_expression() {
    let input = "
    5 + 5;
    5 - 5;
    5 * 5;
    5 / 5;
    5 > 5;
    5 < 5;
    5 == 5;
    5 != 5;
    ";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    let expected_expressions = vec![
        (Operator::Plus, 5, 5),
        (Operator::Minus, 5, 5),
        (Operator::Multiply, 5, 5),
        (Operator::Divide, 5, 5),
        (Operator::GreaterThan, 5, 5),
        (Operator::LessThan, 5, 5),
        (Operator::Equal, 5, 5),
        (Operator::NotEqual, 5, 5),
    ];

    assert_eq!(parser.errors, vec![] as Vec<String>);

    for (index, (operator, left, right)) in expected_expressions.iter().enumerate() {
        assert_eq!(
            program.statements[index],
            Statement::Expression {
                expression: Expression::Infix {
                    operator: *operator,
                    left: Box::new(Expression::Int(*left)),
                    right: Box::new(Expression::Int(*right))
                }
            },
        )
    }
}

#[test]
fn test_boolean_expression() {
    let input = "
    true;
    false;
    ";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    assert_eq!(parser.errors, vec![] as Vec<String>);
    assert_eq!(
        program.statements,
        vec![
            Statement::Expression {
                expression: Expression::Boolean(true)
            },
            Statement::Expression {
                expression: Expression::Boolean(false)
            }
        ]
    );
}

#[test]
fn test_operator_precedence_parsing() {
    let tests = vec![
        ("-a * b", "((-a) * b)"),
        ("!-a", "(!(-a))"),
        ("a + b + c", "((a + b) + c)"),
        ("a + b - c", "((a + b) - c)"),
        ("a * b * c", "((a * b) * c)"),
        ("a * b / c", "((a * b) / c)"),
        ("a + b / c", "(a + (b / c))"),
        ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
        ("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
        ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
        ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
        (
            "3 + 4 * 5 == 3 * 1 + 4 * 5",
            "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
        ),
        ("true", "true"),
        ("false", "false"),
        ("3 > 5 == false", "((3 > 5) == false)"),
        ("3 < 5 == true", "((3 < 5) == true)"),
        ("!(true == true)", "(!(true == true))"),
        ("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
        ("(5 + 5) * 2", "((5 + 5) * 2)"),
        ("2 / (5 + 5)", "(2 / (5 + 5))"),
        ("(5 + 5) * 2 * (5 + 5)", "(((5 + 5) * 2) * (5 + 5))"),
        ("-(5 + 5)", "(-(5 + 5))"),
        ("a + add(b * c) + d", "((a + add((b * c))) + d)"),
        (
            "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
            "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))",
        ),
        (
            "add(a + b + c * d / f + g)",
            "add((((a + b) + ((c * d) / f)) + g))",
        ),
    ];

    for (input, expected_output) in tests {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        assert_eq!(parser.errors, vec![] as Vec<String>);
        assert_eq!(expected_output, program.to_string());
    }
}

#[test]
fn test_if_expression() {
    let input = "if (x < y) { x }";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    assert_eq!(parser.errors, vec![] as Vec<String>);
    assert_eq!(
        program.statements,
        vec![Statement::Expression {
            expression: Expression::If {
                condition: Box::new(Expression::Infix {
                    operator: Operator::LessThan,
                    left: Box::new(Expression::Identifier(String::from("x"))),
                    right: Box::new(Expression::Identifier(String::from("y"))),
                }),
                consequence: BlockStatement {
                    statements: vec![Statement::Expression {
                        expression: Expression::Identifier(String::from("x"))
                    }]
                },
                alternative: None
            }
        },]
    );
}

#[test]
fn test_if_else_expression() {
    let input = "if (x < y) { x } else { y }";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    assert_eq!(parser.errors, vec![] as Vec<String>);
    assert_eq!(
        program.statements,
        vec![Statement::Expression {
            expression: Expression::If {
                condition: Box::new(Expression::Infix {
                    operator: Operator::LessThan,
                    left: Box::new(Expression::Identifier(String::from("x"))),
                    right: Box::new(Expression::Identifier(String::from("y"))),
                }),
                consequence: BlockStatement {
                    statements: vec![Statement::Expression {
                        expression: Expression::Identifier(String::from("x"))
                    }]
                },
                alternative: Some(BlockStatement {
                    statements: vec![Statement::Expression {
                        expression: Expression::Identifier(String::from("y"))
                    }]
                })
            }
        },]
    );
}

#[test]
fn test_function_expression() {
    let input = "func(x, y) { x + y; }";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    assert_eq!(parser.errors, vec![] as Vec<String>);
    assert_eq!(
        program.statements,
        vec![Statement::Expression {
            expression: Expression::Function {
                parameters: vec![String::from("x"), String::from("y"),],
                body: BlockStatement {
                    statements: vec![Statement::Expression {
                        expression: Expression::Infix {
                            operator: Operator::Plus,
                            left: Box::new(Expression::Identifier(String::from("x"))),
                            right: Box::new(Expression::Identifier(String::from("y"))),
                        }
                    }]
                }
            }
        },]
    );
}

#[test]
fn test_function_parameters() {
    let tests = vec![
        ("func() {}", vec![]),
        ("func(x) {}", vec![String::from("x")]),
        (
            "func(x, y, z) {}",
            vec![String::from("x"), String::from("y"), String::from("z")],
        ),
    ];

    for (input, expected_parameters) in tests {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        let statement = program.statements.first().unwrap();

        match statement {
            Statement::Expression {
                expression: Expression::Function { parameters, .. },
            } => {
                assert_eq!(*parameters, expected_parameters);
            }
            _ => assert!(false),
        }
    }
}

#[test]
fn test_call_expression() {
    let input = "add(1, 2 * 3, 4 + 5);";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();

    assert_eq!(parser.errors, vec![] as Vec<String>);
    assert_eq!(
        program.statements,
        vec![Statement::Expression {
            expression: Expression::Call {
                function: Box::new(Expression::Identifier(String::from("add"))),
                arguments: vec![
                    Expression::Int(1),
                    Expression::Infix {
                        operator: Operator::Multiply,
                        left: Box::new(Expression::Int(2)),
                        right: Box::new(Expression::Int(3)),
                    },
                    Expression::Infix {
                        operator: Operator::Plus,
                        left: Box::new(Expression::Int(4)),
                        right: Box::new(Expression::Int(5)),
                    },
                ]
            }
        }]
    );
}
