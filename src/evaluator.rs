use lexer;
use parser;
use object::*;
use ast::*;

pub fn eval<'a>(node: &Node<'a>) -> Option<Box<Object>> {
    match *node {
        Node::IntegerLiteral{ref value, ..} => {
            Some(Box::new(Integer{value: *value}))
        },
        ref node @ Node::Expression {..} => eval(node),
        _ => None
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
}
