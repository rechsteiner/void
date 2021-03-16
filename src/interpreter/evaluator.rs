use crate::interpreter::ast::BlockStatement;
use crate::interpreter::ast::Expression;
use crate::interpreter::ast::Operator;
use crate::interpreter::ast::Program;
use crate::interpreter::ast::Statement;
use crate::interpreter::object::Environment;
use crate::interpreter::object::Object;

pub fn eval(program: Program, environment: &mut Environment) -> Object {
    let mut result = Object::Null;
    for statement in program.statements {
        result = eval_statement(statement, environment);

        match result {
            Object::Return(return_value) => return *return_value,
            Object::Error(_) => return result,
            _ => (),
        }
    }
    result
}

fn eval_statement(statement: Statement, environment: &mut Environment) -> Object {
    match statement {
        Statement::Return { expression } => {
            let return_value = eval_expression(expression, environment);
            if let Object::Error(_) = return_value {
                return return_value;
            }
            Object::Return(Box::new(return_value))
        }
        Statement::Expression { expression } => eval_expression(expression, environment),
        Statement::Let {
            identifier,
            expression,
        } => {
            let object = eval_expression(expression, environment);
            if let Object::Error(_) = object {
                return object;
            }
            environment.set(identifier, object.clone());
            object
        }
    }
}

fn eval_expression(expression: Expression, environment: &mut Environment) -> Object {
    match expression {
        Expression::Int(value) => Object::Integer(value),
        Expression::Boolean(value) => Object::Boolean(value),
        Expression::Prefix { operator, right } => {
            let object = eval_expression(*right, environment);
            if let Object::Error(_) = object {
                return object;
            }
            eval_prefix_expression(operator, object)
        }
        Expression::Infix {
            operator,
            left,
            right,
        } => {
            let left = eval_expression(*left, environment);
            if let Object::Error(_) = left {
                return left;
            }
            let right = eval_expression(*right, environment);
            if let Object::Error(_) = right {
                return right;
            }
            eval_infix_expression(operator, left, right)
        }
        Expression::If {
            condition,
            consequence,
            alternative,
        } => {
            let condition = eval_expression(*condition, environment);
            if let Object::Error(_) = condition {
                return condition;
            }
            eval_if_expression(condition, consequence, alternative, environment)
        }
        Expression::Identifier(name) => eval_identifier(name, environment),
        Expression::Function { parameters, body } => Object::Function {
            parameters: parameters,
            body: body,
            environment: environment.clone(),
        },
        Expression::Call {
            function,
            arguments,
        } => {
            let function = eval_expression(*function, environment);
            let arguments = eval_expressions(arguments, environment);

            // TODO: Validate arguments

            match function {
                Object::Error(_) => function,
                Object::Function {
                    parameters,
                    body,
                    environment,
                } => {
                    let mut extended_environment =
                        extend_function_environment(environment, parameters, arguments);
                    let evaluated = eval_block_statement(body, &mut extended_environment);

                    match evaluated {
                        Object::Return(return_value) => *return_value,
                        _ => evaluated,
                    }
                }

                // TODO: Error handling
                _ => Object::Null,
            }
        }
        _ => Object::Null,
    }
}

fn extend_function_environment(
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

fn eval_expressions(arguments: Vec<Expression>, environment: &mut Environment) -> Vec<Object> {
    let mut result: Vec<Object> = vec![];

    for argument in arguments {
        let evaluated = eval_expression(argument, environment);
        if let Object::Error(_) = evaluated {
            return vec![evaluated];
        }
        result.push(evaluated);
    }

    result
}

fn eval_identifier(name: String, environment: &mut Environment) -> Object {
    match environment.get(&name) {
        Some(value) => value.clone(),
        None => Object::Error(format!("identifier not found: {}", name)),
    }
}

fn eval_prefix_expression(operator: Operator, object: Object) -> Object {
    match operator {
        Operator::Not => eval_not_operator_expression(object),
        Operator::Minus => eval_minus_prefix_operator(object),
        _ => Object::Error(format!("unknown operator: {}{}", operator, object.name())),
    }
}

fn eval_not_operator_expression(object: Object) -> Object {
    match object {
        Object::Boolean(true) => Object::Boolean(false),
        Object::Boolean(false) => Object::Boolean(true),
        Object::Null => Object::Boolean(true),
        _ => Object::Boolean(false),
    }
}

fn eval_minus_prefix_operator(object: Object) -> Object {
    match object {
        Object::Integer(value) => Object::Integer(-value),
        _ => Object::Error(format!("unknown operator: -{}", object.name())),
    }
}

fn eval_infix_expression(operator: Operator, left: Object, right: Object) -> Object {
    // TODO: Figure out why we can't format these directly
    let left_string = left.name();
    let right_string = right.name();

    match (left, right) {
        (Object::Integer(left), Object::Integer(right)) => {
            eval_integer_infix_expression(operator, left, right)
        }
        (Object::Boolean(left), Object::Boolean(right)) => {
            eval_boolean_infix_expression(operator, left, right)
        }
        _ => Object::Error(format!(
            "type mismatch: {} {} {}",
            left_string, operator, right_string
        )),
    }
}

fn eval_integer_infix_expression(operator: Operator, left: isize, right: isize) -> Object {
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

fn eval_boolean_infix_expression(operator: Operator, left: bool, right: bool) -> Object {
    match operator {
        Operator::Equal => Object::Boolean(left == right),
        Operator::NotEqual => Object::Boolean(left != right),
        _ => Object::Error(format!("unknown operator: boolean {} boolean", operator)),
    }
}

fn eval_block_statement(statement: BlockStatement, environment: &mut Environment) -> Object {
    let mut object = Object::Null;
    for statement in statement.statements {
        object = eval_statement(statement, environment);

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
    condition: Object,
    consequence: BlockStatement,
    alternative: Option<BlockStatement>,
    environment: &mut Environment,
) -> Object {
    if is_truthy(condition) {
        eval_block_statement(consequence, environment)
    } else if let Some(alternative) = alternative {
        eval_block_statement(alternative, environment)
    } else {
        Object::Null
    }
}

fn is_truthy(condition: Object) -> bool {
    match condition {
        Object::Null => false,
        Object::Boolean(true) => true,
        Object::Boolean(false) => false,
        _ => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

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
    fn test_eval_boolean_expression() {
        let tests = vec![
            ("true", Object::Boolean(true)),
            ("false", Object::Boolean(false)),
            ("1 < 2", Object::Boolean(true)),
            ("1 > 2", Object::Boolean(false)),
            ("1 < 1", Object::Boolean(false)),
            ("1 > 1", Object::Boolean(false)),
            ("1 == 1", Object::Boolean(true)),
            ("1 != 1", Object::Boolean(false)),
            ("1 == 2", Object::Boolean(false)),
            ("1 != 2", Object::Boolean(true)),
            ("true == true", Object::Boolean(true)),
            ("false == false", Object::Boolean(true)),
            ("true == false", Object::Boolean(false)),
            ("true != false", Object::Boolean(true)),
            ("(1 < 2) == true", Object::Boolean(true)),
            ("(1 < 2) == false", Object::Boolean(false)),
            ("(1 > 2) == true", Object::Boolean(false)),
            ("(1 > 2) == false", Object::Boolean(true)),
        ];

        for (input, expected_output) in tests {
            let object = test_eval(input);
            assert_eq!(object, expected_output);
        }
    }

    #[test]
    fn test_not_operator() {
        let tests = vec![
            ("!true", Object::Boolean(false)),
            ("!false", Object::Boolean(true)),
            ("!5", Object::Boolean(false)),
            ("!!true", Object::Boolean(true)),
            ("!!false", Object::Boolean(false)),
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
            ("if (true) { 10 }", Object::Integer(10)),
            ("if (false) { 10 }", Object::Null),
            ("if (1) { 10 }", Object::Integer(10)),
            ("if (1 < 2) { 10 }", Object::Integer(10)),
            ("if (1 > 2) { 10 }", Object::Null),
            ("if (1 > 2) { 10 } else { 20 }", Object::Integer(20)),
            ("if (1 < 2) { 10 } else { 20 }", Object::Integer(10)),
        ];

        for (input, expected_output) in tests {
            let object = test_eval(input);
            assert_eq!(object, expected_output);
        }
    }

    #[test]
    fn test_return_statements() {
        let tests = vec![
            ("return 10;", Object::Integer(10)),
            ("return 10; 9;", Object::Integer(10)),
            ("return 2 * 5; 9", Object::Integer(10)),
            ("9; return 2 * 5; 9;", Object::Integer(10)),
            (
                "
            if (10 > 1) {
                if (10 > 1) {
                    return 10;
                }
                return 1;
            }
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
                "5 + true;",
                Object::Error(String::from("type mismatch: integer + boolean")),
            ),
            (
                "5 + true; 5;",
                Object::Error(String::from("type mismatch: integer + boolean")),
            ),
            (
                "-true",
                Object::Error(String::from("unknown operator: -boolean")),
            ),
            (
                "true + false;",
                Object::Error(String::from("unknown operator: boolean + boolean")),
            ),
            (
                "5; true + false; 5",
                Object::Error(String::from("unknown operator: boolean + boolean")),
            ),
            (
                "if (10 > 1) { true + false; }",
                Object::Error(String::from("unknown operator: boolean + boolean")),
            ),
            (
                "
                if (10 > 1) {
                    if (10 > 1) {
                        return true + false;
                    }
                    return 1;
                }
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
            ("let a = 5; let b = a; b;", Object::Integer(5)),
            (
                "let a = 5; let b = a; let c = a + b + 5; c;",
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
        let input = "func(x) { x + 2; };";
        let object = test_eval(input);

        assert_eq!(
            object,
            Object::Function {
                parameters: vec![String::from("x")],
                body: BlockStatement {
                    statements: vec![Statement::Expression {
                        expression: Expression::Infix {
                            operator: Operator::Plus,
                            left: Box::new(Expression::Identifier(String::from("x"))),
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
                "let identity = func(x) { x; }; identity(5);",
                Object::Integer(5),
            ),
            (
                "let identity = func(x) { return x; }; identity(5);",
                Object::Integer(5),
            ),
            (
                "let double = func(x) { x * 2; }; double(5);",
                Object::Integer(10),
            ),
            (
                "let add = func(x, y) { x + y; }; add(5, 5);",
                Object::Integer(10),
            ),
            (
                "let add = func(x) { return x; }; add(add(20));",
                Object::Integer(20),
            ),
            ("func(x) { x; }(5);", Object::Integer(5)),
        ];

        for (input, expected_output) in tests {
            let object = test_eval(input);
            assert_eq!(object, expected_output);
        }
    }

    #[test]
    fn test_closures() {
        let input = "
        let newAdder = func(x) {
            func(y) { x + y };
        };

        let addTwo = newAdder(2);
        addTwo(2);
        ";
        let object = test_eval(input);
        assert_eq!(object, Object::Integer(4));
    }

    fn test_eval(input: &'static str) -> Object {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut environment = Environment::new();
        return eval(program, &mut environment);
    }
}
