use crate::interpreter::ast::BlockStatement;
use crate::interpreter::ast::Expression;
use crate::interpreter::ast::Operator;
use crate::interpreter::ast::Program;
use crate::interpreter::ast::Statement;
use crate::interpreter::object::Command;
use crate::interpreter::object::Environment;
use crate::interpreter::object::Object;

pub struct Evaluator {
    pub commands: Vec<Command>,
}

impl Evaluator {
    pub fn new() -> Evaluator {
        Evaluator { commands: vec![] }
    }
    pub fn eval(&mut self, program: &Program, environment: &mut Environment) -> Object {
        // Reset commands each time eval is called so we don't keep commands
        // from previous executions.
        self.commands = vec![];
        let mut result = Object::Null;
        for statement in &program.statements {
            result = self.eval_statement(statement, environment);
            match result {
                Object::Return(return_value) => return *return_value,
                Object::Error(_) => return result,
                _ => (),
            }
        }
        result
    }
    fn eval_statement(&mut self, statement: &Statement, environment: &mut Environment) -> Object {
        match statement {
            Statement::Return { expression } => {
                let return_value = self.eval_expression(expression, environment);
                if let Object::Error(_) = return_value {
                    return return_value;
                }
                Object::Return(Box::new(return_value))
            }
            Statement::Expression { expression } => self.eval_expression(expression, environment),
            Statement::Let {
                identifier,
                expression,
            } => {
                let object = self.eval_expression(expression, environment);
                if let Object::Error(_) = object {
                    return object;
                }
                environment.set(identifier.clone(), object.clone());
                object
            }
        }
    }
    fn eval_expression(
        &mut self,
        expression: &Expression,
        environment: &mut Environment,
    ) -> Object {
        match expression {
            Expression::Int(value) => Object::Integer(*value),
            Expression::Float(value) => Object::Float(*value),
            Expression::Boolean(value) => Object::Boolean(*value),
            Expression::Prefix { operator, right } => {
                let object = self.eval_expression(right, environment);
                if let Object::Error(_) = object {
                    return object;
                }
                self.eval_prefix_expression(*operator, object)
            }
            Expression::Infix {
                operator,
                left,
                right,
            } => {
                let left = self.eval_expression(left, environment);
                if let Object::Error(_) = left {
                    return left;
                }
                let right = self.eval_expression(right, environment);
                if let Object::Error(_) = right {
                    return right;
                }
                self.eval_infix_expression(*operator, left, right)
            }
            Expression::If {
                condition,
                consequence,
                alternative,
            } => {
                let condition = self.eval_expression(condition, environment);
                if let Object::Error(_) = condition {
                    return condition;
                }
                self.eval_if_expression(condition, consequence, alternative, environment)
            }
            Expression::Identifier(name) => self.eval_identifier(name.clone(), environment),
            Expression::Function { parameters, body } => Object::Function {
                parameters: parameters.clone(),
                body: body.clone(),
                environment: environment.clone(),
            },
            Expression::Call {
                function,
                arguments,
            } => {
                let function = self.eval_expression(function, environment);
                let arguments = self.eval_expressions(arguments, environment);
                // TODO: Validate arguments
                match function {
                    Object::Error(_) => function,
                    Object::Function {
                        parameters,
                        body,
                        environment,
                    } => {
                        let mut extended_environment =
                            self.extend_function_environment(environment, parameters, arguments);
                        let evaluated = self.eval_block_statement(&body, &mut extended_environment);
                        match evaluated {
                            Object::Return(return_value) => *return_value,
                            _ => evaluated,
                        }
                    }
                    Object::Command { function } => match function(arguments) {
                        Ok(command) => {
                            self.commands.push(command);
                            Object::Null
                        }
                        Err(error) => Object::Error(error),
                    },
                    // TODO: Error handling
                    _ => Object::Null,
                }
            }
        }
    }
    fn extend_function_environment(
        &mut self,
        environment: Environment,
        parameters: Vec<String>,
        arguments: Vec<Object>,
    ) -> Environment {
        let mut env = Environment::extend(environment);
        for (index, argument) in arguments.iter().enumerate() {
            let param = parameters[index].clone();
            env.set(param, argument.clone());
        }
        env
    }
    fn eval_expressions(
        &mut self,
        arguments: &Vec<Expression>,
        environment: &mut Environment,
    ) -> Vec<Object> {
        let mut result: Vec<Object> = vec![];
        for argument in arguments {
            let evaluated = self.eval_expression(argument, environment);
            if let Object::Error(_) = evaluated {
                return vec![evaluated];
            }
            result.push(evaluated);
        }
        result
    }
    fn eval_identifier(&mut self, name: String, environment: &mut Environment) -> Object {
        match environment.get(&name) {
            Some(value) => value.clone(),
            None => Object::Error(format!("identifier not found: {}", name)),
        }
    }
    fn eval_prefix_expression(&mut self, operator: Operator, object: Object) -> Object {
        match operator {
            Operator::Not => self.eval_not_operator_expression(object),
            Operator::Minus => self.eval_minus_prefix_operator(object),
            _ => Object::Error(format!("unknown operator: {}{}", operator, object.name())),
        }
    }
    fn eval_not_operator_expression(&mut self, object: Object) -> Object {
        match object {
            Object::Boolean(true) => Object::Boolean(false),
            Object::Boolean(false) => Object::Boolean(true),
            Object::Null => Object::Boolean(true),
            _ => Object::Boolean(false),
        }
    }
    fn eval_minus_prefix_operator(&mut self, object: Object) -> Object {
        match object {
            Object::Integer(value) => Object::Integer(-value),
            Object::Float(value) => Object::Float(-value),
            _ => Object::Error(format!("unknown operator: -{}", object.name())),
        }
    }
    fn eval_infix_expression(&mut self, operator: Operator, left: Object, right: Object) -> Object {
        // TODO: Figure out why we can't format these directly
        let left_string = left.name();
        let right_string = right.name();
        match (left, right) {
            (Object::Integer(left), Object::Integer(right)) => {
                self.eval_integer_infix_expression(operator, left, right)
            }
            (Object::Boolean(left), Object::Boolean(right)) => {
                self.eval_boolean_infix_expression(operator, left, right)
            }
            (Object::Float(left), Object::Float(right)) => {
                self.eval_float_infix_expression(operator, left, right)
            }
            _ => Object::Error(format!(
                "type mismatch: {} {} {}",
                left_string, operator, right_string
            )),
        }
    }

    fn eval_integer_infix_expression(
        &mut self,
        operator: Operator,
        left: isize,
        right: isize,
    ) -> Object {
        match operator {
            Operator::Plus => Object::Integer(left + right),
            Operator::Minus => Object::Integer(left - right),
            Operator::Divide => Object::Integer(left / right),
            Operator::Multiply => Object::Integer(left * right),
            Operator::LessThan => Object::Boolean(left < right),
            Operator::GreaterThan => Object::Boolean(left > right),
            Operator::Equal => Object::Boolean(left == right),
            Operator::NotEqual => Object::Boolean(left != right),
            _ => Object::Error(format!("unknown operator: integer {} integer", operator)),
        }
    }

    fn eval_float_infix_expression(&mut self, operator: Operator, left: f64, right: f64) -> Object {
        match operator {
            Operator::Plus => Object::Float(left + right),
            Operator::Minus => Object::Float(left - right),
            Operator::Divide => Object::Float(left / right),
            Operator::Multiply => Object::Float(left * right),
            Operator::LessThan => Object::Boolean(left < right),
            Operator::GreaterThan => Object::Boolean(left > right),
            Operator::Equal => Object::Boolean(left == right),
            Operator::NotEqual => Object::Boolean(left != right),
            _ => Object::Error(format!("unknown operator: float {} float", operator)),
        }
    }

    fn eval_boolean_infix_expression(
        &mut self,
        operator: Operator,
        left: bool,
        right: bool,
    ) -> Object {
        match operator {
            Operator::Equal => Object::Boolean(left == right),
            Operator::NotEqual => Object::Boolean(left != right),
            _ => Object::Error(format!("unknown operator: boolean {} boolean", operator)),
        }
    }
    fn eval_block_statement(
        &mut self,
        statement: &BlockStatement,
        environment: &mut Environment,
    ) -> Object {
        let mut object = Object::Null;
        for statement in &statement.statements {
            object = self.eval_statement(statement, environment);
            match object {
                Object::Return(_) | Object::Error(_) => {
                    return object;
                }
                _ => (),
            }
        }
        return object;
    }
    fn eval_if_expression(
        &mut self,
        condition: Object,
        consequence: &BlockStatement,
        alternative: &Option<BlockStatement>,
        environment: &mut Environment,
    ) -> Object {
        if self.is_truthy(condition) {
            self.eval_block_statement(consequence, environment)
        } else if let Some(alternative) = alternative {
            self.eval_block_statement(alternative, environment)
        } else {
            Object::Null
        }
    }
    fn is_truthy(&mut self, condition: Object) -> bool {
        match condition {
            Object::Null => false,
            Object::Boolean(true) => true,
            Object::Boolean(false) => false,
            _ => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::lexer::Lexer;
    use crate::interpreter::parser::Parser;

    #[test]
    fn test_eval_integer_expression() {
        let tests = vec![
            ("5", Object::Integer(5)),
            ("10", Object::Integer(10)),
            ("-5", Object::Integer(-5)),
            ("-10", Object::Integer(-10)),
            ("5 + 5 + 5 + 5 - 10", Object::Integer(10)),
            ("2 * 2 * 2 * 2 * 2", Object::Integer(32)),
            ("-50 + 100 + -50", Object::Integer(0)),
            ("5 * 2 + 10", Object::Integer(20)),
            ("5 + 2 * 10", Object::Integer(25)),
            ("20 + 2 * -10", Object::Integer(0)),
            ("50 / 2 * 2 + 10", Object::Integer(60)),
            ("2 * (5 + 10)", Object::Integer(30)),
            ("3 * 3 * 3 + 10", Object::Integer(37)),
            ("3 * (3 * 3) + 10", Object::Integer(37)),
            ("(5 + 10 * 2 + 15 / 3) * 2 + -10", Object::Integer(50)),
        ];

        for (input, expected_output) in tests {
            let object = test_eval(input);
            assert_eq!(object, expected_output);
        }
    }

    #[test]
    fn test_eval_float_expression() {
        let tests = vec![
            ("5.0", Object::Float(5.0)),
            ("10.0", Object::Float(10.0)),
            ("-5.0", Object::Float(-5.0)),
            ("-10.0", Object::Float(-10.0)),
            ("5.0 + 5.0 + 5.0 + 5.0 - 10.0", Object::Float(10.0)),
            ("2.0 * 2.0 * 2.0 * 2.0 * 2.0", Object::Float(32.0)),
            ("-50.0 + 100.0 + -50.0", Object::Float(0.0)),
            ("5.0 * 2.0 + 10.0", Object::Float(20.0)),
            ("5.0 + 2.0 * 10.0", Object::Float(25.0)),
            ("20.0 + 2.0 * -10.0", Object::Float(0.0)),
            ("50.0 / 2.0 * 2.0 + 10.0", Object::Float(60.0)),
            ("2.0 * (5.0 + 10.0)", Object::Float(30.0)),
            ("3.0 * 3.0 * 3.0 + 10.0", Object::Float(37.0)),
            ("3.0 * (3.0 * 3.0) + 10.0", Object::Float(37.0)),
            (
                "(5.0 + 10.0 * 2.0 + 15.0 / 3.0) * 2.0 + -10.0",
                Object::Float(50.0),
            ),
        ];

        for (input, expected_output) in tests {
            let object = test_eval(input);
            assert_eq!(object, expected_output);
        }
    }

    #[test]
    fn test_eval_boolean_expression() {
        let tests = vec![
            ("TRUE", Object::Boolean(true)),
            ("FALSE", Object::Boolean(false)),
            ("1 < 2", Object::Boolean(true)),
            ("1 > 2", Object::Boolean(false)),
            ("1 < 1", Object::Boolean(false)),
            ("1 > 1", Object::Boolean(false)),
            ("1 == 1", Object::Boolean(true)),
            ("1 != 1", Object::Boolean(false)),
            ("1 == 2", Object::Boolean(false)),
            ("1 != 2", Object::Boolean(true)),
            ("TRUE == TRUE", Object::Boolean(true)),
            ("FALSE == FALSE", Object::Boolean(true)),
            ("TRUE == FALSE", Object::Boolean(false)),
            ("TRUE != FALSE", Object::Boolean(true)),
            ("(1 < 2) == TRUE", Object::Boolean(true)),
            ("(1 < 2) == FALSE", Object::Boolean(false)),
            ("(1 > 2) == TRUE", Object::Boolean(false)),
            ("(1 > 2) == FALSE", Object::Boolean(true)),
        ];

        for (input, expected_output) in tests {
            let object = test_eval(input);
            assert_eq!(object, expected_output);
        }
    }

    #[test]
    fn test_not_operator() {
        let tests = vec![
            ("!TRUE", Object::Boolean(false)),
            ("!FALSE", Object::Boolean(true)),
            ("!5", Object::Boolean(false)),
            ("!!TRUE", Object::Boolean(true)),
            ("!!FALSE", Object::Boolean(false)),
            ("!!5", Object::Boolean(true)),
        ];

        for (input, expected_output) in tests {
            let object = test_eval(input);
            assert_eq!(object, expected_output);
        }
    }

    #[test]
    fn test_if_else_expressions() {
        let tests = vec![
            ("IF TRUE DO 10 END", Object::Integer(10)),
            ("IF FALSE DO 10 END", Object::Null),
            ("IF 1 DO 10 END", Object::Integer(10)),
            ("IF 1 < 2 DO 10 END", Object::Integer(10)),
            ("IF 1 > 2 DO 10 ELSE 20 END", Object::Integer(20)),
            ("IF 1 < 2 DO 10 ELSE 20 END", Object::Integer(10)),
        ];

        for (input, expected_output) in tests {
            let object = test_eval(input);
            assert_eq!(object, expected_output);
        }
    }

    #[test]
    fn test_return_statements() {
        let tests = vec![
            ("RETURN 10", Object::Integer(10)),
            (
                "
            IF 10 > 1 DO
                IF 10 > 1 DO
                    RETURN 10
                END
                RETURN 1
            END
            ",
                Object::Integer(10),
            ),
        ];

        for (input, expected_output) in tests {
            let object = test_eval(input);
            assert_eq!(object, expected_output);
        }
    }

    #[test]
    fn test_error_handling() {
        let tests = vec![
            (
                "5 + TRUE",
                Object::Error(String::from("type mismatch: integer + boolean")),
            ),
            (
                "
                5 + TRUE
                5
                ",
                Object::Error(String::from("type mismatch: integer + boolean")),
            ),
            (
                "-TRUE",
                Object::Error(String::from("unknown operator: -boolean")),
            ),
            (
                "TRUE + FALSE",
                Object::Error(String::from("unknown operator: boolean + boolean")),
            ),
            (
                "5
                TRUE + FALSE
                5",
                Object::Error(String::from("unknown operator: boolean + boolean")),
            ),
            (
                "IF 10 > 1 DO TRUE + FALSE END",
                Object::Error(String::from("unknown operator: boolean + boolean")),
            ),
            (
                "
                IF 10 > 1 DO
                    IF 10 > 1 DO
                        RETURN TRUE + FALSE
                    END
                    RETURN 1
                END
                ",
                Object::Error(String::from("unknown operator: boolean + boolean")),
            ),
            (
                "foobar",
                Object::Error(String::from("identifier not found: foobar")),
            ),
        ];

        for (input, expected_output) in tests {
            let object = test_eval(input);
            assert_eq!(object, expected_output);
        }
    }

    #[test]
    fn test_let_statements() {
        let tests = vec![
            (
                "
            LET A = 5
            LET B = A
            B
            ",
                Object::Integer(5),
            ),
            (
                "
                LET A = 5
                LET B = A
                LET C = A + B + 5
                C
                ",
                Object::Integer(15),
            ),
        ];

        for (input, expected_output) in tests {
            let object = test_eval(input);
            assert_eq!(object, expected_output);
        }
    }

    #[test]
    fn test_function_object() {
        let input = "FUNC X DO X + 2 END";
        let object = test_eval(input);

        assert_eq!(
            object,
            Object::Function {
                parameters: vec![String::from("X")],
                body: BlockStatement {
                    statements: vec![Statement::Expression {
                        expression: Expression::Infix {
                            operator: Operator::Plus,
                            left: Box::new(Expression::Identifier(String::from("X"))),
                            right: Box::new(Expression::Int(2)),
                        }
                    }]
                },
                environment: Environment::new()
            }
        )
    }

    #[test]
    fn test_function_application() {
        let tests = vec![
            (
                "
                LET IDENTITY = FUNC X DO X END
                IDENTITY(5)
                ",
                Object::Integer(5),
            ),
            (
                "
                LET IDENTITY = FUNC X DO RETURN X END
                IDENTITY(5)
                ",
                Object::Integer(5),
            ),
            (
                "
                LET DOUBLE = FUNC X DO X * 2 END
                DOUBLE(5)
                ",
                Object::Integer(10),
            ),
            (
                "
                LET ADD = FUNC X Y DO X + Y END
                ADD(5, 5)
                ",
                Object::Integer(10),
            ),
            (
                "
                LET ADD = FUNC X DO RETURN X END
                ADD(ADD(20))
                ",
                Object::Integer(20),
            ),
            ("FUNC X DO X END(5)", Object::Integer(5)),
        ];

        for (input, expected_output) in tests {
            let object = test_eval(input);
            assert_eq!(object, expected_output);
        }
    }

    #[test]
    fn test_closures() {
        let input = "
        LET NEW_ADDER = FUNC X DO
            FUNC Y DO X + Y END
        END

        LET ADD_TWO = NEW_ADDER(2)
        ADD_TWO(2)
        ";
        let object = test_eval(input);
        assert_eq!(object, Object::Integer(4));
    }

    #[test]
    fn test_command_functions() {
        let tests = vec![
            (
                "SET_THRUST(10)",
                vec![Command::SetThrust { throttle: 10.0 }],
                Object::Null,
            ),
            (
                "
                LET A = 10
                SET_THRUST(A)
                LET B = 20
                SET_THRUST(B)
                ",
                vec![
                    Command::SetThrust { throttle: 10.0 },
                    Command::SetThrust { throttle: 20.0 },
                ],
                Object::Null,
            ),
            (
                "SET_THRUST(TRUE)",
                vec![],
                Object::Error(String::from("argument not supported, got boolean")),
            ),
            (
                "SET_THRUST(0, 1)",
                vec![],
                Object::Error(String::from("wrong number of arguments. got=2, want=1")),
            ),
        ];

        for (input, expected_commands, expected_output) in tests {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program().unwrap();
            let mut environment = Environment::new();
            let mut evaluator = Evaluator::new();

            environment.set(
                String::from("SET_THRUST"),
                Object::Command {
                    function: |arguments| {
                        if arguments.len() != 1 {
                            return Result::Err(format!(
                                "wrong number of arguments. got={}, want=1",
                                arguments.len()
                            ));
                        }
                        match arguments[0].clone() {
                            Object::Integer(value) => Result::Ok(Command::SetThrust {
                                throttle: value as f64,
                            }),
                            _ => Result::Err(format!(
                                "argument not supported, got {}",
                                arguments[0].name()
                            )),
                        }
                    },
                },
            );
            let object = evaluator.eval(&program, &mut environment);
            assert_eq!(object, expected_output);
            assert_eq!(evaluator.commands, expected_commands);
        }
    }

    fn test_eval(input: &'static str) -> Object {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().unwrap();
        let mut environment = Environment::new();
        let mut evaluator = Evaluator::new();
        return evaluator.eval(&program, &mut environment);
    }
}
