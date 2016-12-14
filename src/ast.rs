use token::Token;

#[derive(Debug, Clone)]
pub enum Node<'a> {
    LetStatement {
        token: Token<'a>,
        name: Box<Node<'a>>,
        value: Box<Node<'a>>,
    },
    ReturnStatement {
        token: Token<'a>,
        value: Box<Node<'a>>,
    },
    Identifier {
        token: Token<'a>,
        value: &'a str,
    },
}


impl<'a> Node<'a> {
    pub fn get_token_literal(&self) -> Token {
        match *self {
            Node::LetStatement { token: ref t, .. } => t.clone(),
            Node::ReturnStatement { token: ref t, .. } => t.clone(),
            _ => panic!("Expected a valid token"),
        }
    }
}

#[derive(Default)]
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
