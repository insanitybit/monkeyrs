// use std::mem::replace;

use ast::*;
use lexer::Lexer;
use token::Token;
use std::iter::Peekable;

#[derive(Debug)]
pub struct Parser<'a> {
    token_iter: Peekable<Lexer<'a>>,
    cur_token: Option<Token<'a>>,
}

#[derive(Debug, Clone, PartialEq ,PartialOrd)]
// Numbers must be > 0, but have no meaning other than ordering
pub enum Precedence {
    Lowest = 1,
    Equals = 2,
    LessGreater = 3,
    Sum = 4,
    Product = 5,
    Prefix = 6,
    Call = 7,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Parser<'a> {
        let mut p = lexer.peekable();
        let cur_token = p.next();
        Parser {
            token_iter: p,
            cur_token: cur_token,
        }
    }

    pub fn get_cur_token(&self) -> Option<Token<'a>> {
        self.cur_token
    }

    pub fn peek_token(&mut self) -> Option<Token<'a>> {
        self.token_iter.peek().cloned()
    }

    pub fn next_token(&mut self) -> Option<Token<'a>> {
        self.cur_token = self.peek_token();
        self.token_iter.next()
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program::new();

        loop {
            if let None = self.get_cur_token() {
                break;
            }
            let stmt = self.parse_statement();
            if let Some(st) = stmt {
                program.statements.push(st);
            }


            self.next_token();
        }

        program
    }

    pub fn parse_statement(&mut self) -> Option<Node<'a>> {
        match self.get_cur_token() {
            Some(Token::LET) => Some(self.parse_let_statement()),
            Some(Token::RETURN) => Some(self.parse_return_statement()),
            Some(Token::LBRACE) => Some(self.parse_block_statement()),
            _ => self.parse_expression_statement(),
        }
    }

    pub fn parse_expression_statement(&mut self) -> Option<Node<'a>> {
        let expr = self.parse_expression(Precedence::Lowest);

        if let Some(&Token::SEMICOLON) = self.token_iter.peek() {
            self.next_token();
        }

        expr
    }

    pub fn parse_function_literal(&mut self) -> Option<Node<'a>> {
        let tok = self.next_token();

        if let Some(Token::LPAREN) = self.peek_token() {
            self.next_token();
        } else {
            return None;
        }

        let params = self.parse_function_parameters();

        if let Some(Token::LBRACE) = self.peek_token() {
            self.next_token();
        } else {
            return None;
        }

        let fn_body = self.parse_block_statement();

        Some(Node::FunctionLiteral {
            token: tok.unwrap(),
            parameters: params,
            body: Box::new(fn_body),
        })
    }

    pub fn parse_function_parameters(&mut self) -> Vec<Node<'a>> {
        let mut identifiers = Vec::new();

        if Some(Token::RPAREN) == self.peek_token() {
            self.next_token();
            return identifiers;
        }

        self.next_token();

        match self.cur_token {
            Some(Token::IDENT(name)) => {
                identifiers.push(Node::Identifier {
                    token: self.cur_token.expect("cur_token was none"),
                    value: name,
                });
            }
            _ => panic!("Expected identifier for function parameter!"),
        }

        loop {
            if let Some(Token::COMMA) = self.peek_token() {
                self.next_token();
                self.next_token();

                match self.cur_token {
                    Some(Token::IDENT(name)) => {
                        identifiers.push(Node::Identifier {
                            token: self.cur_token.expect("cur_token was none"),
                            value: name,
                        });
                    }
                    _ => panic!("Expected identifier for function parameter!"),
                }
            } else {
                break;
            }
        }

        match self.peek_token() {
            Some(Token::RPAREN) => {
                self.next_token();
                identifiers
            }
            _ => panic!("Unexpected token after function params"),
        }
    }

    pub fn parse_expression(&mut self, precedence: Precedence) -> Option<Node<'a>> {
        let tok = self.get_cur_token().expect("parse_expression get_cur_token");
        let mut left_expr = match self.prefix_parse(tok) {
            Some(le) => le,
            None => return None,
        };

        loop {
            let peek_tok = self.peek_token();
            let peek_prec = self.peek_precedence();

            if Some(Token::SEMICOLON) != peek_tok && precedence >= peek_prec {
                break;
            }

            self.next_token();

            left_expr = match self.infix_parse(peek_tok.expect("peek_tok"), left_expr.clone()) {
                Some(le) => {
                    self.next_token();
                    le
                }
                None => return Some(left_expr),
            };
        }

        Some(left_expr)
    }

    fn peek_precedence(&mut self) -> Precedence {
        self.token_iter.peek().map(Token::get_precedence).unwrap_or(Precedence::Lowest)
    }

    fn cur_precedence(&self) -> Precedence {
        self.cur_token.unwrap().get_precedence()
    }

    pub fn parse_integer_literal(&mut self) -> Node<'a> {
        match self.cur_token {
            Some(Token::INT(i)) => {
                Node::IntegerLiteral {
                    token: self.cur_token.unwrap(),
                    value: i,
                }
            }
            _ => panic!(),
        }
    }

    pub fn parse_return_statement(&mut self) -> Node<'a> {
        let init_token = self.get_cur_token().take().unwrap();

        self.next_token();

        let value = self.parse_expression(Precedence::Lowest);


        Node::ReturnStatement {
            token: init_token,
            value: value.map(Box::new),
        }
    }

    pub fn parse_let_statement(&mut self) -> Node<'a> {
        let init_token = self.cur_token.clone();

        let ident = if let Some(Token::IDENT(name)) = self.peek_token() {
            self.next_token();
            Node::Identifier {
                token: self.cur_token.expect("Failed to get cur_token in parse_let_statement"),
                value: name,
            }
        } else {
            panic!("Expected identifier");
        };

        assert!(Some(Token::ASSIGN) == self.peek_token(),
                format!("{:#?}", self.peek_token()));
        self.next_token();
        self.next_token();

        let value = self.parse_expression(Precedence::Lowest);

        if let Some(Token::SEMICOLON) = self.peek_token() {
            self.next_token();
            self.next_token();
        };

        Node::LetStatement {
            token: init_token.expect("init token is None"),
            name: Box::new(ident),
            value: Box::new(value.expect("let statement with empty values")),
        }

    }


    fn prefix_parse(&mut self, tok: Token<'a>) -> Option<Node<'a>> {
        match tok {
            Token::IDENT(value) => {
                Some(Node::Identifier {
                    token: tok,
                    value: value,
                })
            }
            Token::INT(i) => {
                Some(Node::IntegerLiteral {
                    token: tok,
                    value: i,
                })
            }
            Token::TRUE => {
                Some(Node::Boolean {
                    token: tok,
                    value: true,
                })
            }
            Token::FALSE => {
                Some(Node::Boolean {
                    token: tok,
                    value: false,
                })
            }
            Token::MINUS => Some(self.parse_prefix_expression(tok)),
            Token::BANG => Some(self.parse_prefix_expression(tok)),
            Token::FUNCTION => self.parse_function_literal(),
            // Token::PLUS => Some(self.parse_infix_expression(tok, expr: Node<'a>)),
            _ => None,
        }
    }

    fn parse_prefix_expression(&mut self, tok: Token<'a>) -> Node<'a> {
        self.next_token();
        let right = self.parse_expression(Precedence::Prefix);
        Node::PrefixExpression {
            token: tok.clone(),
            operator: Parser::operator_from_tok(tok),
            right: right.map(Box::new),
        }
    }

    fn operator_from_tok(tok: Token<'a>) -> &'static str {
        match tok {
            Token::PLUS => "PLUS",
            Token::MINUS => "MINUS",
            Token::GT => "GT",
            Token::LT => "LT",
            Token::BANG => "BANG",
            Token::ASTERISK => "ASTERISK",
            Token::SLASH => "SLASH",
            _ => panic!("Unexpected operator"),
        }
    }

    fn infix_parse(&mut self, tok: Token<'a>, expr: Node<'a>) -> Option<Node<'a>> {
        match tok {
            tok @ Token::PLUS |
            tok @ Token::MINUS |
            tok @ Token::SLASH |
            tok @ Token::ASTERISK |
            tok @ Token::EQ |
            tok @ Token::NOT_EQ |
            tok @ Token::LT |
            tok @ Token::GT => Some(self.parse_infix_expression(tok, expr)),
            Token::LPAREN => Some(self.parse_call_expression(expr)),
            _ => None,
        }
    }

    fn parse_infix_expression(&mut self, tok: Token<'a>, expr: Node<'a>) -> Node<'a> {
        let precedence = self.cur_precedence();
        self.next_token();

        let right = self.parse_expression(precedence);
        Node::InfixExpression {
            token: tok,
            operator: Parser::operator_from_tok(tok),
            left: Box::new(expr),
            right: right.map(Box::new),
        }
    }

    pub fn parse_call_expression(&mut self, expr: Node<'a>) -> Node<'a> {
        Node::CallExpression {
            token: self.cur_token.unwrap(),
            fn_name: Box::new(expr),
            parameters: self.parse_call_arguments(),
        }
    }

    fn parse_call_arguments(&mut self) -> Vec<Node<'a>> {
        let mut arguments = Vec::new();

        if let Some(Token::RPAREN) = self.peek_token() {
            self.next_token();
            return arguments;
        }

        self.next_token();

        let arg = self.parse_expression(Precedence::Lowest);

        arguments.push(arg.unwrap());

        loop {
            if let Some(Token::COMMA) = self.peek_token() {
                self.next_token();
                self.next_token();

                let arg = self.parse_expression(Precedence::Lowest);
                arguments.push(arg.unwrap());
            } else {
                break;
            }
        }

        self.next_token();
        arguments

    }

    fn parse_block_statement(&mut self) -> Node<'a> {

        self.next_token();

        let mut statements = Vec::new();

        while let Some(statement) = self.parse_statement() {
            statements.push(Box::new(statement));
            self.next_token();
        }

        Node::BlockStatement {
            token: Token::LBRACE,
            statements: statements,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let input = "let negative_five = -5; return !negative_five; let y = 4 + 4;";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let expected = Program {
            statements: vec![Node::LetStatement {
                                 token: Token::LET,
                                 name: Box::new(Node::Identifier {
                                     token: Token::IDENT("negative_five"),
                                     value: "negative_five",
                                 }),
                                 value: Box::new(Node::PrefixExpression {
                                     token: Token::MINUS,
                                     operator: "MINUS",
                                     right: Some(Box::new(Node::IntegerLiteral {
                                         token: Token::INT(5),
                                         value: 5,
                                     })),
                                 }),
                             },
                             Node::ReturnStatement {
                                 token: Token::RETURN,
                                 value: Some(Box::new(Node::PrefixExpression {
                                     token: Token::BANG,
                                     operator: "BANG",
                                     right: Some(Box::new(Node::Identifier {
                                         token: Token::IDENT("negative_five"),
                                         value: "negative_five",
                                     })),
                                 })),
                             },
                             Node::LetStatement {
                                 token: Token::LET,
                                 name: Box::new(Node::Identifier {
                                     token: Token::IDENT("y"),
                                     value: "y",
                                 }),
                                 value: Box::new(Node::InfixExpression {
                                     token: Token::PLUS,
                                     operator: "PLUS",
                                     left: Box::new(Node::IntegerLiteral {
                                         token: Token::INT(4),
                                         value: 4,
                                     }),
                                     right: Some(Box::new(Node::IntegerLiteral {
                                         token: Token::INT(4),
                                         value: 4,
                                     })),
                                 }),
                             }],
        };

        assert_eq!(parser.parse_program(), expected, "AST differs");

    }

    #[test]
    fn test_block_statement() {
        let input = "{ let a = 4; let b = 5; };";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let expected = Program {
            statements: vec![
                Node::BlockStatement {
                    token: Token::LBRACE,
                    statements: vec![
                        Box::new(Node::LetStatement {
                            token: Token::LET,
                            name: Box::new(Node::Identifier {
                                token: Token::IDENT(
                                    "a"
                                ),
                                value: "a"
                            }),
                            value: Box::new(Node::IntegerLiteral {
                                token: Token::INT(
                                    4
                                ),
                                value: 4
                            })
                        }),
                        Box::new(Node::LetStatement {
                            token: Token::LET,
                            name: Box::new(Node::Identifier {
                                token: Token::IDENT(
                                    "b"
                                ),
                                value: "b"
                            }),
                            value: Box::new(Node::IntegerLiteral {
                                token: Token::INT(
                                    5
                                ),
                                value: 5
                            })
                        })
                    ]
                },
            ],
        };

        assert_eq!(parser.parse_program(), expected, "AST differs");
    }

    #[test]
    fn test_fn_literal_statement() {
        let input = "fn foo(bar, baz) {let x = 5; return x;}";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let expected = Program {
            statements: vec![Node::FunctionLiteral {
                                 token: Token::IDENT("foo"),
                                 parameters: vec![Node::Identifier {
                                                      token: Token::IDENT("bar"),
                                                      value: "bar",
                                                  },
                                                  Node::Identifier {
                                                      token: Token::IDENT("baz"),
                                                      value: "baz",
                                                  }],
                                 body: Box::new(Node::BlockStatement {
                                     token: Token::LBRACE,
                                     statements: vec![Box::new(Node::LetStatement {
                                                          token: Token::LET,
                                                          name: Box::new(Node::Identifier {
                                                              token: Token::IDENT("x"),
                                                              value: "x",
                                                          }),
                                                          value: Box::new(Node::IntegerLiteral {
                                                              token: Token::INT(5),
                                                              value: 5,
                                                          }),
                                                      }),
                                                      Box::new(Node::ReturnStatement {
                                                          token: Token::RETURN,
                                                          value: Some(Box::new(Node::Identifier {
                                                              token: Token::IDENT("x"),
                                                              value: "x",
                                                          })),
                                                      })],
                                 }),
                             }],
        };

        assert_eq!(parser.parse_program(), expected);
    }

    #[test]
    fn test_fn_call() {
        let input = "foo(bar, baz);";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let expected = Program {
            statements: vec![Node::CallExpression {
                                 token: Token::LPAREN,
                                 fn_name: Box::new(Node::Identifier {
                                     token: Token::IDENT("foo"),
                                     value: "foo",
                                 }),
                                 parameters: vec![Node::Identifier {
                                                      token: Token::IDENT("bar"),
                                                      value: "bar",
                                                  },
                                                  Node::Identifier {
                                                      token: Token::IDENT("baz"),
                                                      value: "baz",
                                                  }],
                             }],
        };


        assert_eq!(parser.parse_program(), expected);
    }

    #[test]
    fn test_double_infix_expression() {
        let input = "let a = 5 + 4 * 3 + 2;";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let expected = Program {
            statements: vec![
                Node::LetStatement {
                    token: Token::LET,
                    name: Box::new(Node::Identifier {
                        token: Token::IDENT(
                            "a"
                        ),
                        value: "a"
                    }),
                    value: Box::new(Node::InfixExpression {
                        token: Token::PLUS,
                        operator: "PLUS",
                        left: Box::new(Node::IntegerLiteral {
                            token: Token::INT(5),
                            value: 5
                        }),
                        right: Some(Box::new(Node::InfixExpression {
                            token: Token::ASTERISK,
                            operator: "ASTERISK",
                            left: Box::new(Node::IntegerLiteral {
                                token: Token::INT(4),
                                value: 4
                            }),
                            right: Some(Box::new(Node::IntegerLiteral {
                                token: Token::INT(3),
                                value: 3
                            }))
                        }))
                    }),
                },
            ],
        };

        assert_eq!(parser.parse_program(), expected);
    }

    #[test]
    fn test_bool() {
        let input = "true";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let expected = Program {
            statements: vec![
                Node::Boolean {
                    token: Token::TRUE,
                    value: true,
                }
            ]
        };


        assert_eq!(parser.parse_program(), expected);
    }
}
