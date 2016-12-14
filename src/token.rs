#![allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
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
    BOOL(bool),
    RETURN,
    IF,
    ELSE,
}
