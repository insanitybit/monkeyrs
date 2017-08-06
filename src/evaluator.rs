use lexer;
use parser;
use object::*;
use ast::*;

pub fn eval<'a>(node: &Node<'a>) -> Option<Box<Object>> {
    match *node {
        Node::IntegerLiteral { ref value, .. } => Some(Box::new(Integer { value: *value })),
        Node::Boolean { ref value, .. } => Some(Box::new(Boolean { value: *value })),
        Node::Expression { ref value, .. } => eval(value),
        Node::PrefixExpression { right: Some(box ref right), operator, .. } => {
            let right = eval(right)
                .expect(&format!("Could not parse right expression: {:#?}", right));
            eval_prefix_expressions(operator, &*right)
        }
        Node::InfixExpression { left: box ref left, right: Some(box ref right), operator, .. } => {
            let left = eval(left)
                .expect(&format!("Could not parse left expression: {:#?}", left));

            let right = eval(right)
                .expect(&format!("Could not parse right expression: {:#?}", right));

            eval_infix_expression(operator, &*left, &*right)
        }
        Node::BlockStatement {ref statements, ..} => {
            eval_boxed_statements(&statements[..])
        },
        Node::IfExpression { box ref condition, box ref consequence, box ref alternative, .. } => {
            eval_if_condition(condition, consequence, alternative)
        },
        _ => None,
    }
}

pub fn eval_if_condition<'a>(condition: &Node<'a>, consequence: &Node<'a>, alternative: &Node<'a>) -> Option<Box<Object>> {

    let condition = eval(condition).unwrap();

    if let ObjectType::Boolean = condition.get_type() {
       if condition.inspect() == "true" {
           eval(consequence)
       } else {
           eval(alternative)
       }
    } else {
        None
    }
}

pub fn eval_infix_expression(operator: &str, left: &Object, right: &Object) -> Option<Box<Object>> {
    match (left.get_type(), right.get_type()) {
        (ObjectType::Integer64, ObjectType::Integer64) => eval_integer_infix_express(operator, left, right),
        (ObjectType::Boolean, ObjectType::Boolean) => eval_boolean_infix_express(operator, left, right),
        _ => None,
    }
}

pub fn eval_boolean_infix_express(operator: &str, left: &Object, right: &Object) -> Option<Box<Object>> {
    let left = left.inspect().parse::<bool>().unwrap();
    let right = right.inspect().parse::<bool>().unwrap();

    match operator {
        "EQ" => Some(box Boolean { value: left == right }),
        "NOT_EQ" => Some(box Boolean { value: left != right }),
        _ => None,
    }
}

pub fn eval_integer_infix_express(operator: &str, left: &Object, right: &Object) -> Option<Box<Object>> {
    let left = left.inspect().parse::<i64>().unwrap();
    let right = right.inspect().parse::<i64>().unwrap();

    match operator {
        "PLUS" => Some(box Integer { value: left + right }),
        "MINUS" => Some(box Integer { value: left - right }),
        "ASTERISK" => Some(box Integer { value: left * right }),
        "SLASH" => Some(box Integer { value: left / right }),
        "EQ" => Some(box Boolean { value: left == right }),
        "NOT_EQ" => Some(box Boolean { value: left != right }),
        _ => None,
    }
}

pub fn eval_prefix_expressions(operator: &str, expression: &Object) -> Option<Box<Object>> {
    match operator {
        "BANG" => eval_bang(expression),
        "MINUS" => eval_minus(expression),
        _ => None,
    }
}

pub fn eval_bang(expr: &Object) -> Option<Box<Object>> {
    expr.inspect().parse::<bool>().map(|b| Box::new(Boolean { value: !b }) as Box<Object>).ok()
}

pub fn eval_minus(expr: &Object) -> Option<Box<Object>> {
    expr.inspect().parse::<i64>().map(|i| Box::new(Integer { value: -1 * i }) as Box<Object>).ok()
}


pub fn eval_program(program: Program) -> Option<Box<Object>> {
    eval_statements(&program.statements[..])
}

pub fn eval_statements<'a>(statements: &[Node<'a>]) -> Option<Box<Object>> {
    let mut result = None;

    for statement in statements {
        result = eval(statement);
    }

    result
}

pub fn eval_boxed_statements<'a>(statements: &[Box<Node<'a>>]) -> Option<Box<Object>> {
    let mut result = None;

    for statement in statements {
        result = eval(statement);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_integer_literal() {
        let input = "5";

        let lexer = lexer::Lexer::new(input);
        let mut parser = parser::Parser::new(lexer);

        let program = parser.parse_program();

        let evaled = eval_program(program);

        assert_eq!(evaled.unwrap().inspect(), "5");
    }

    #[test]
    fn test_eval_bool_literal() {
        let input = "true";

        let lexer = lexer::Lexer::new(input);
        let mut parser = parser::Parser::new(lexer);

        let program = parser.parse_program();

        let evaled = eval_program(program);

        assert_eq!(evaled.unwrap().inspect(), "true");
    }

    #[test]
    fn test_eval_bang_bool_literal() {
        let input = "!true";

        let lexer = lexer::Lexer::new(input);
        let mut parser = parser::Parser::new(lexer);

        let program = parser.parse_program();
        let evaled = eval_program(program);

        assert_eq!(evaled.expect("failed to eval").inspect(), "false");
    }

    #[test]
    fn test_eval_bang_bang_bool_literal() {
        let input = "!!true";

        let lexer = lexer::Lexer::new(input);
        let mut parser = parser::Parser::new(lexer);

        let program = parser.parse_program();
        let evaled = eval_program(program);

        assert_eq!(evaled.expect("failed to eval").inspect(), "true");
    }

    #[test]
    fn test_eval_negate() {
        let input = "-5";

        let lexer = lexer::Lexer::new(input);
        let mut parser = parser::Parser::new(lexer);

        let program = parser.parse_program();

        let evaled = eval_program(program);

        assert_eq!(evaled.expect("failed to eval").inspect(), "-5");
    }

    #[test]
    fn test_eval_plus() {
        let input = "-5 + 5";

        let lexer = lexer::Lexer::new(input);
        let mut parser = parser::Parser::new(lexer);

        let program = parser.parse_program();

        let evaled = eval_program(program);

        assert_eq!(evaled.expect("failed to eval").inspect(), "0");
    }

    #[test]
    fn test_bool_cmp() {
        let input = "true == true";

        let lexer = lexer::Lexer::new(input);
        let mut parser = parser::Parser::new(lexer);

        let program = parser.parse_program();

        let evaled = eval_program(program);

        assert_eq!(evaled.expect("failed to eval").inspect(), "true");
    }

    #[test]
    fn test_integer_cmp() {
        let input = "3 == 3";

        let lexer = lexer::Lexer::new(input);
        let mut parser = parser::Parser::new(lexer);

        let program = parser.parse_program();

        let evaled = eval_program(program);

        assert_eq!(evaled.expect("failed to eval").inspect(), "true");
    }

    #[test]
    fn test_conditional_1() {
        let input = "if (3 == 3) { 1 } else { -1 } ";

        let lexer = lexer::Lexer::new(input);
        let mut parser = parser::Parser::new(lexer);

        let program = parser.parse_program();

        let evaled = eval_program(program);

        assert_eq!(evaled.expect("failed to eval").inspect(), "1");
    }

    fn test_conditional_2() {
        let input = "if (3 != 3) { 1 } else { -1 } ";

        let lexer = lexer::Lexer::new(input);
        let mut parser = parser::Parser::new(lexer);

        let program = parser.parse_program();

        let evaled = eval_program(program);

        assert_eq!(evaled.expect("failed to eval").inspect(), "-1");
    }
}
