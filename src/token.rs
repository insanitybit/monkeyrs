#![allow(non_camel_case_types)]

// use parser::Precedence;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token<'a> {
    ILLEGAL,
    EOF,

    // Identifiers + literals
    IDENT(&'a str),
    INT(u64),
    STRING(&'a str),

    // Operators
    ASSIGN,
    PLUS,
    MINUS,
    GT,
    LT,
    BANG,
    ASTERISK,
    SLASH,
    EQ,
    NOT_EQ,

    // Delimiters
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // Keywords
    FUNCTION,
    LET,
    WHILE,
    FOR,
    LOOP,
    TRUE,
    FALSE,
    RETURN,
    IF,
    ELSE,
}
// impl<'a> Token<'a> {
//     pub fn get_precedence(&self) -> Precedence {
//         match *self {
//             Token::EQ => Precedence::Equals,
//             Token::NOT_EQ => Precedence::Equals,
//             Token::GT => Precedence::LessGreater,
//             Token::LT => Precedence::LessGreater,
//             Token::PLUS => Precedence::Sum,
//             Token::MINUS => Precedence::Sum,
//             Token::SLASH => Precedence::Product,
//             Token::ASTERISK => Precedence::Product,
//             Token::LPAREN => Precedence::Call,
//             _ => Precedence::Lowest,
//         }
//     }
// }
