use token::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Node<'a> {
    LetStatement {
        token: Token<'a>,
        name: Box<Node<'a>>,
        value: Box<Node<'a>>,
    },
    ReturnStatement {
        token: Token<'a>,
        value: Option<Box<Node<'a>>>,
    },
    Identifier {
        token: Token<'a>,
        value: &'a str,
    },
    Expression {
        token: Token<'a>,
        value: Box<Node<'a>>,
    },
    IntegerLiteral {
        token: Token<'a>,
        value: u64,
    },
    PrefixExpression {
        token: Token<'a>,
        operator: &'a str,
        right: Option<Box<Node<'a>>>,
    },
    InfixExpression {
        token: Token<'a>,
        operator: &'a str,
        left: Box<Node<'a>>,
        right: Option<Box<Node<'a>>>,
    },
    BlockStatement {
        token: Token<'a>,
        statements: Vec<Box<Node<'a>>>,
    },
    FunctionLiteral {
        token: Token<'a>,
        parameters: Vec<Node<'a>>,
        body: Box<Node<'a>>,
    },
}


impl<'a> Node<'a> {
    pub fn get_token_literal(&self) -> Token {
        match *self {
            Node::LetStatement { token: t, .. } => t,
            Node::ReturnStatement { token: t, .. } => t,
            Node::Identifier { token: t, .. } => t,
            Node::Expression { token: t, .. } => t,
            Node::IntegerLiteral { token: t, .. } => t,
            Node::PrefixExpression { token: t, .. } => t,
            Node::InfixExpression { token: t, .. } => t,
            Node::BlockStatement { token: t, .. } => t,
            Node::FunctionLiteral { token: t, .. } => t,
            // _ => panic!("Expected a valid token"),
        }
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct Program<'a> {
    pub statements: Vec<Node<'a>>,
}

impl<'a> Program<'a> {
    pub fn new() -> Program<'a> {
        Program { statements: vec![] }
    }

    fn get_token_literal(&self) -> Token {
        self.statements
            .iter()
            .next()
            .map(|n| n.get_token_literal())
            .expect("Expected a valid token")
    }
}
