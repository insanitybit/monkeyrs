use lexer;
use parser;
use object::*;
use ast::*;

pub fn eval<'a>(node: &Node<'a>) -> Option<Box<Object>> {
    match *node {
        Node::IntegerLiteral { ref value, .. } => Some(Box::new(Integer { value: *value })),
        Node::Boolean { ref value, .. } => Some(Box::new(Boolean { value: *value })),
        Node::Expression { ref value, .. } => eval(value),
        Node::PrefixExpression { right: Some(ref right), operator, .. } => {
            eval_prefix_expressions(operator, right)
        }
        _ => None,
    }
}

pub fn eval_prefix_expressions<'a>(operator: &str, expression: &Node<'a>) -> Option<Box<Object>> {
    match operator {
        "BANG" => eval_bang(expression),
        "MINUS" => eval_minus(expression),
        _    => None
    }
}

pub fn eval_bang<'a>(expr: &Node<'a>) -> Option<Box<Object>> {
    match *expr {
        Node::Boolean {ref value, ..} => Some(Box::new(Boolean{value: !value})),
        _ => None
    }
}

pub fn eval_minus<'a>(expr: &Node<'a>) -> Option<Box<Object>> {
    match *expr {
        Node::IntegerLiteral {ref value, ..} => Some(Box::new(Integer{value: -1 * value})),
        _ => panic!(format!("Can not apply MINUS operator to {:#?}", expr))
    }
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
    fn test_eval_negate() {
        let input = "-5";

        let lexer = lexer::Lexer::new(input);
        let mut parser = parser::Parser::new(lexer);

        let program = parser.parse_program();

        let evaled = eval_program(program);

        assert_eq!(evaled.expect("failed to eval").inspect(), "-5");
    }
}
