#![allow(dead_code, non_upper_case_globals)]

// extern crate regex;

// use self::regex::Regex;

use std::iter::*;
use std::str::*;

use token::Token::*;
use token::Token;

#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    input_iter: Peekable<Chars<'a>>,
    position: usize, // current position in input (points to current char)
    read_position: usize, // current reading position in input (after current char)
}


impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer {
            input: input,
            input_iter: input.chars().peekable(),
            position: 0,
            read_position: 1,
        }
    }

    pub fn next_token<'b>(&mut self) -> Option<Token<'b>>
        where 'a: 'b
    {
        self.skip_whitespace();
        if let Some(tok) = self.read_char() {
            match tok {
                '=' => {
                    if let Some('=') = self.peek_char() {
                        self.read_char();
                        Some(EQ)
                    } else {
                        Some(ASSIGN)
                    }
                }
                ';' => Some(SEMICOLON),
                '(' => Some(LPAREN),
                ')' => Some(RPAREN),
                ',' => Some(COMMA),
                '+' => Some(PLUS),
                '{' => Some(LBRACE),
                '}' => Some(RBRACE),
                '>' => Some(GT),
                '<' => Some(LT),
                '-' => Some(MINUS),
                '!' => {
                    if let Some('=') = self.peek_char() {
                        self.read_char();
                        Some(NOT_EQ)
                    } else {
                        Some(BANG)
                    }
                }
                '*' => Some(ASTERISK),
                '/' => Some(SLASH),
                c if c.is_alphabetic() => {
                    let (ix, end_ix) = self.read_identifier();
                    let ident = &self.input[ix..end_ix];
                    return Some(determine_ident(ident));
                }
                c if c.is_digit(10) => {
                    return Some(INT(self.read_number()));
                }
                _ => Some(ILLEGAL),
            }
        } else {
            None
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek_char() {
            if c.is_whitespace() {
                self.read_char();
            } else {
                break;
            }
        }
    }

    fn peek_char(&mut self) -> Option<char> {
        self.input_iter.peek().map(|c| *c)
    }

    fn read_char(&mut self) -> Option<char> {
        self.position = self.read_position;
        self.read_position += 1;
        self.input_iter.next()
    }

    fn read_identifier(&mut self) -> (usize, usize) {
        let position = self.position - 1;
        let mut end_position = position + 1;
        while let Some(c) = self.peek_char() {
            if Lexer::valid_identifier(c) {
                end_position += 1;
                self.read_char();
            } else {
                break;
            }
        }
        (position, end_position)
    }

    fn valid_identifier(ch: char) -> bool {
        ch.is_alphabetic() || ch == '_'
    }

    fn read_number(&mut self) -> u64 {
        let position = self.position;

        while let Some(c) = self.peek_char() {
            if c.is_digit(10) {
                self.read_char();
            } else {
                break;
            }
        }

        self.input[position - 1..self.position]
            .parse::<u64>()
            .expect(&format!("Failed to parse number {}",
                             &self.input[position - 1..self.position]))
    }
}

fn determine_ident(ident: &str) -> Token {
    if ident == "let" {
        LET
    } else if ident == "fn" {
        FUNCTION
    } else if ident == "while" {
        WHILE
    } else if ident == "for" {
        FOR
    } else if ident == "loop" {
        LOOP
    } else if ident == "true" {
        TRUE
    } else if ident == "false" {
        FALSE
    } else if ident == "if" {
        IF
    } else if ident == "else" {
        ELSE
    } else if ident == "return" {
        RETURN
    } else {
        IDENT(ident)
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;
    fn next(&mut self) -> Option<Token<'a>> {
        self.next_token()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use token::Token::*;


    // #[test]
    // fn test_lex() {
    //     let input = "let five = 5 + 2;";
    //     let output = vec![Token::LET,
    //                       Token::IDENT("five"),
    //                       Token::ASSIGN,
    //                       Token::INT(5),
    //                       Token::PLUS,
    //                       Token::INT(2),
    //                       Token::SEMICOLON];
    //
    //
    //     let mut lexer = Lexer::new(input);
    //     let mut tokens = Vec::new();
    //
    //     loop {
    //         let tok = lexer.next_token();
    //         match tok {
    //             Some(ILLEGAL) => panic!("Illegal token: {:#?}", tok),
    //             None | Some(EOF) => break,
    //             Some(t) => tokens.push(t),
    //         }
    //     }
    //
    //     assert_eq!(tokens, output);
    // }

    // #[test]
    // fn test_interpreter() {
    //     let input = "let five = 5;
    //     let ten = 10;
    //
    //     let add = fn(x, y) {
    //       x + y;
    //     };
    //
    //     let result = add(five, ten);
    //     !-/*5;
    //     5 < 10 > 5;
    //
    //     if 5 < 10 {
    //         return true;
    //     } else if 5 == 5 {
    //         return false;
    //     } else if 5 != hello {
    //       return true;
    //     } else {
    //       return false;
    //     }";
    //
    //     let mut lexer = Lexer::new(input);
    //     loop {
    //         let tok = lexer.next_token();
    //         match tok {
    //             ILLEGAL => panic!("Illegal token: {:#?}", tok),
    //             EOF => break,
    //             _ => continue,
    //         }
    //     }
    // }

    #[test]
    fn test_lexer() {
        let input = "let five = 5;
                    let ten = 10;

                    let add = fn(x, y) {
                    x + y;
                    };

                    let result = add(five, ten);
                    !-/*5;
                    5 < 10 > 5;

                    if (5 < 10) {
                    return true;
                    } else {
                    return false;
                    }

                    10 == 10;
                    10 != 9;
                    ";

        let mut lexer = Lexer::new(input);

        let expected = vec![
 Token::LET,
Token::IDENT("five"),
Token::ASSIGN,
Token::INT(5),
Token::SEMICOLON,
Token::LET,
Token::IDENT("ten"),
Token::ASSIGN,
Token::INT(10),
Token::SEMICOLON,
Token::LET,
Token::IDENT("add"),
Token::ASSIGN,
Token::FUNCTION,
Token::LPAREN,
Token::IDENT("x"),
Token::COMMA,
Token::IDENT("y"),
Token::RPAREN,
Token::LBRACE,
Token::IDENT("x"),
Token::PLUS,
Token::IDENT("y"),
Token::SEMICOLON,
Token::RBRACE,
Token::SEMICOLON,
Token::LET,
Token::IDENT("result"),
Token::ASSIGN,
Token::IDENT("add"),
Token::LPAREN,
Token::IDENT("five"),
Token::COMMA,
Token::IDENT("ten"),
Token::RPAREN,
Token::SEMICOLON,
Token::BANG,
Token::MINUS,
Token::SLASH,
Token::ASTERISK,
Token::INT(5),
Token::SEMICOLON,
Token::INT(5),
Token::LT,
Token::INT(10),
Token::GT,
Token::INT(5),
Token::SEMICOLON,
Token::IF,
Token::LPAREN,
Token::INT(5),
Token::LT,
Token::INT(10),
Token::RPAREN,
Token::LBRACE,
Token::RETURN,
Token::TRUE,
Token::SEMICOLON,
Token::RBRACE,
Token::ELSE,
Token::LBRACE,
Token::RETURN,
Token::FALSE,
Token::SEMICOLON,
Token::RBRACE,
Token::INT(10),
Token::EQ,
Token::INT(10),
Token::SEMICOLON,
Token::INT(10),
Token::NOT_EQ,
Token::INT(9),
Token::SEMICOLON,
// Token::EOF,

];

        let mut tokens = Vec::new();

        loop {
            let tok = lexer.next_token();

            match tok {
                Some(ILLEGAL) => panic!("Illegal token: {:#?}", tok),
                Some(t) => {
                    tokens.push(t);
                }
                None => break,
            }
        }

        for (actual, expected) in tokens.into_iter().zip(expected.into_iter()) {
            if actual != expected {
                assert_eq!(actual, expected);
            }
        }
    }
}
