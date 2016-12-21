use std::mem::replace;

use ast::*;
use lexer::Lexer;
use token::Token;

#[derive(Debug)]
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    cur_token: Token<'a>,
    peek_token: Token<'a>,
}

#[derive(Debug, Clone)]
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
    pub fn new(mut lexer: Lexer<'a>) -> Parser<'a> {
        let cur_token = lexer.next_token();
        let peek_token = lexer.next_token();
        Parser {
            lexer: lexer,
            cur_token: cur_token,
            peek_token: peek_token,
        }
    }

    pub fn get_cur_token(&self) -> Token<'a> {
        self.cur_token.clone()
    }

    pub fn next_token(&mut self) {
        self.cur_token = replace(&mut self.peek_token, self.lexer.next_token());
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program::new();

        loop {
            if let Token::EOF = self.cur_token {
                break;
            }
            let stmt = self.parse_statement();
            program.statements.push(stmt.clone());

            self.next_token();
        }

        program
    }

    pub fn parse_statement(&mut self) -> Node<'a> {
        match self.cur_token {
            Token::LET => self.parse_let_statement(),
            Token::RETURN => self.parse_return_statement(),
            _ => panic!(),
        }
    }

    pub fn parse_expression_statement(&mut self) -> Node<'a> {
        let _init_token = self.cur_token.clone();

        let _expr = self.parse_expression(Precedence::Lowest);
        while self.peek_token == Token::SEMICOLON {
            self.next_token();
        }

        unimplemented!()
    }

    pub fn parse_expression(&mut self, _precedent: Precedence) -> Node<'a> {
        return self.prefix_parse(self.cur_token.clone());
    }

    pub fn parse_integer_literal(&mut self) -> Node<'a> {
        match self.cur_token {
            Token::INT(i) => {
                Node::IntegerLiteral {
                    token: self.cur_token.clone(),
                    value: i,
                }
            }
            _ => panic!(),
        }
    }

    pub fn parse_return_statement(&mut self) -> Node<'a> {
        let init_token = self.cur_token.clone();

        while self.cur_token != Token::SEMICOLON {
            self.next_token();
        }

        Node::ReturnStatement {
            token: init_token,
            value: unimplemented!(),
        }
    }

    pub fn parse_let_statement(&mut self) -> Node<'a> {
        let init_token = self.cur_token.clone();

        let ident = if let Token::IDENT(name) = self.peek_token {
            self.next_token();
            Node::Identifier {
                token: self.cur_token.clone(),
                value: name,
            }
        } else {
            panic!("Expected indentifier");
        };

        if Token::EQ != self.peek_token {
            panic!("Expected equals sign");
        }

        // TODO: Parse expression and provide that as 'value' field to LetStatement
        while self.cur_token != Token::SEMICOLON {
            self.next_token();
        }

        Node::LetStatement {
            token: init_token,
            name: Box::new(ident),
            value: unimplemented!(),
        }

    }


    fn prefix_parse(&self, tok: Token<'a>) -> Node<'a> {
        match tok {
            Token::IDENT(value) => {
                Node::Identifier {
                    token: tok,
                    value: value,
                }
            }
            _ => unimplemented!(),
        }
    }

    fn infix_parse(&self, tok: Token<'a>, expr: Node<'a>) -> Node<'a> {
        unimplemented!()
    }
    // func (p *Parser) ParseProgram() *ast.Program {
    // program := &ast.Program{}
    // program.Nodes = []ast.Node{}
    //
    // for p.curToken.Type != token.EOF {
    //     stmt := p.parseNode()
    //     if stmt != nil {
    //         program.Nodes = append(program.Nodes, stmt)
    //     }
    //     p.nextToken()
    // }
    //
    // return program
    // }
}
