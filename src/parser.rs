// use std::mem::replace;
//
// use ast::*;
// use lexer::Lexer;
// use token::Token;
//
// #[derive(Debug)]
// pub struct Parser<'a> {
//     lexer: Lexer<'a>,
//     cur_token: Token<'a>,
//     peek_token: Token<'a>,
// }
//
// #[derive(Debug, Clone, PartialEq ,PartialOrd)]
// // Numbers must be > 0, but have no meaning other than ordering
// pub enum Precedence {
//     Lowest = 1,
//     Equals = 2,
//     LessGreater = 3,
//     Sum = 4,
//     Product = 5,
//     Prefix = 6,
//     Call = 7,
// }
//
// impl<'a> Parser<'a> {
//     pub fn new(mut lexer: Lexer<'a>) -> Parser<'a> {
//         let cur_token = lexer.next_token().unwrap();
//         let peek_token = lexer.next_token().unwrap();
//         Parser {
//             lexer: lexer,
//             cur_token: cur_token,
//             peek_token: peek_token,
//         }
//     }
//
//     pub fn get_cur_token(&self) -> Token<'a> {
//         self.cur_token.clone()
//     }
//
//     pub fn next_token(&mut self) {
//         self.cur_token = replace(&mut self.peek_token, self.lexer.next_token().unwrap());
//     }
//
//     pub fn parse_program(&mut self) -> Program {
//         let mut program = Program::new();
//
//         loop {
//             if let Token::EOF = self.cur_token {
//                 break;
//             }
//             let stmt = self.parse_statement();
//             program.statements.push(stmt.clone());
//
//             self.next_token();
//         }
//
//         program
//     }
//
//     pub fn parse_statement(&mut self) -> Node<'a> {
//         match self.cur_token {
//             Token::LET => self.parse_let_statement(),
//             Token::RETURN => self.parse_return_statement(),
//             _ => self.parse_expression_statement(),
//         }
//     }
//
//     pub fn parse_expression_statement(&mut self) -> Node<'a> {
//         let _init_token = self.cur_token.clone();
//
//         let expr = self.parse_expression(Precedence::Lowest);
//
//         if self.peek_token == Token::SEMICOLON {
//             self.next_token();
//         }
//
//         expr
//     }
//
//     pub fn parse_expression(&mut self, precedence: Precedence) -> Node<'a> {
//         let tok = self.cur_token.clone();
//
//         let mut left_expr = self.prefix_parse(tok.clone())
//                                 .expect(&format!("Failed to parse prefix for: {:#?}", tok));
//
//         loop {
//             if Token::SEMICOLON == self.peek_token || precedence >= self.peek_precedence() {
//                 break;
//             }
//
//             let peek_tok = self.peek_token.clone();
//             left_expr = self.infix_parse(peek_tok, left_expr);
//
//             self.next_token();
//         }
//
//         left_expr
//     }
//
//     fn peek_precedence(&self) -> Precedence {
//         self.peek_token.get_precedence()
//     }
//
//     fn cur_precedence(&self) -> Precedence {
//         self.cur_token.get_precedence()
//     }
//
//     pub fn parse_integer_literal(&mut self) -> Node<'a> {
//         match self.cur_token {
//             Token::INT(i) => {
//                 Node::IntegerLiteral {
//                     token: self.cur_token.clone(),
//                     value: i,
//                 }
//             }
//             _ => panic!(),
//         }
//     }
//
//     pub fn parse_return_statement(&mut self) -> Node<'a> {
//         let init_token = self.cur_token.clone();
//
//         self.next_token();
//         println!("parse_return_statement {:?}", self.cur_token);
//         let value = self.parse_expression(Precedence::Lowest);
//
//
//         Node::ReturnStatement {
//             token: init_token,
//             value: Box::new(value),
//         }
//     }
//
//     pub fn parse_let_statement(&mut self) -> Node<'a> {
//         let init_token = self.cur_token.clone();
//
//         let ident = if let Token::IDENT(name) = self.peek_token {
//             self.next_token();
//             Node::Identifier {
//                 token: self.cur_token.clone(),
//                 value: name,
//             }
//         } else {
//             panic!("Expected indentifier");
//         };
//
//         assert!(Token::ASSIGN == self.peek_token,
//                 format!("{:#?}", self.peek_token));
//
//         self.next_token();
//
//         println!("parse let statement {:?}", self.cur_token);
//         let value = self.parse_expression(Precedence::Lowest);
//
//         if let Token::SEMICOLON = self.peek_token {
//             self.next_token();
//         };
//
//         Node::LetStatement {
//             token: init_token,
//             name: Box::new(ident),
//             value: Box::new(value),
//         }
//
//     }
//
//
//     fn prefix_parse(&mut self, tok: Token<'a>) -> Option<Node<'a>> {
//         match tok {
//             Token::IDENT(value) => {
//                 Some(Node::Identifier {
//                     token: tok,
//                     value: value,
//                 })
//             }
//             Token::INT(i) => {
//                 Some(Node::IntegerLiteral {
//                     token: tok,
//                     value: i,
//                 })
//             }
//             Token::MINUS => Some(self.parse_prefix_expression(tok)),
//             Token::BANG => Some(self.parse_prefix_expression(tok)),
//             _ => None,
//         }
//     }
//
//     fn parse_prefix_expression(&mut self, tok: Token<'a>) -> Node<'a> {
//         self.next_token();
//         println!("parse_prefix_expression {:?}", tok);
//         let right = self.parse_expression(Precedence::Prefix);
//         Node::PrefixExpression {
//             token: tok.clone(),
//             operator: Parser::operator_from_tok(tok),
//             right: Box::new(right),
//         }
//     }
//
//     fn operator_from_tok(tok: Token<'a>) -> &'static str {
//         match tok {
//             Token::PLUS => "PLUS",
//             Token::MINUS => "MINUS",
//             Token::GT => "GT",
//             Token::LT => "LT",
//             Token::BANG => "BANG",
//             Token::ASTERISK => "ASTERISK",
//             Token::SLASH => "SLASH",
//             _ => panic!("Unexpected operator"),
//         }
//     }
//
//     fn infix_parse(&mut self, tok: Token<'a>, expr: Node<'a>) -> Node<'a> {
//         match tok {
//             t => self.parse_infix_expression(t, expr),
//         }
//     }
//
//     fn parse_infix_expression(&mut self, tok: Token<'a>, expr: Node<'a>) -> Node<'a> {
//         let precedence = self.cur_precedence();
//         self.next_token();
//
//         let right = self.parse_expression(precedence);
//         Node::InfixExpression {
//             token: tok.clone(),
//             operator: Parser::operator_from_tok(tok),
//             left: Box::new(expr),
//             right: Box::new(right),
//         }
//     }
//     // func (p *Parser) ParseProgram() *ast.Program {
//     // program := &ast.Program{}
//     // program.Nodes = []ast.Node{}
//     //
//     // for p.curToken.Type != token.EOF {
//     //     stmt := p.parseNode()
//     //     if stmt != nil {
//     //         program.Nodes = append(program.Nodes, stmt)
//     //     }
//     //     p.nextToken()
//     // }
//     //
//     // return program
//     // }
// }
//
// // #[cfg(test)]
// // mod tests {
// //     use super::*;
// //
// //     // #[test]
// //     fn test_parser() {
// //         let input = "let negative_five = -5; return !negative_five; let y = 4 + 4;";
// //
// //         let mut lexer = Lexer::new(input);
// //
// //         let mut parser = Parser::new(lexer);
// //
// //         println!("{:?}", parser.parse_program());
// //
// //     }
// // }
