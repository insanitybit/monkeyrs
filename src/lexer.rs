#![allow(dead_code, non_upper_case_globals)]

// extern crate regex;

// use self::regex::Regex;

use token::Token::*;
use token::Token;

#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    position: usize, // current position in input (points to current char)
    read_position: usize, // current reading position in input (after current char)
    ch: char, // current char under examination
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer {
            input: input,
            position: 0,
            read_position: 1,
            ch: input.chars().nth(0).unwrap_or('\u{0004}'),
        }
    }

    pub fn next_token<'b>(&mut self) -> Token<'b>
        where 'a: 'b
    {
        self.skip_whitespace();

        if self.read_position == self.input.len() {
            return EOF;
        }

        let tok = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    EQ
                } else {
                    ASSIGN
                }
            }
            ';' => SEMICOLON,
            '(' => LPAREN,
            ')' => RPAREN,
            ',' => COMMA,
            '+' => PLUS,
            '{' => LBRACE,
            '}' => RBRACE,
            '>' => GT,
            '<' => LT,
            '-' => MINUS,
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    NOT_EQ
                } else {
                    BANG
                }
            }
            '*' => ASTERISK,
            '/' => SLASH,
            c if c.is_alphabetic() => {
                let ix = self.read_identifier();
                let ident = &self.input[ix..self.position];
                return determine_ident(ident);
            }
            c if c.is_digit(10) => {
                return INT(self.read_number());
            }
            _ => ILLEGAL,
        };
        self.read_char();
        tok
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }

    fn peek_char(&self) -> char {
        self.input.chars().nth(self.read_position).unwrap()
    }

    fn back_peek_char(&self) -> char {
        self.input.chars().nth(self.read_position - 1).unwrap()
    }

    fn read_char(&mut self) {
        self.ch = self.input.chars().nth(self.read_position).unwrap();
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> usize {
        let position = self.position;
        while self.ch.is_alphabetic() {
            self.read_char();
        }
        position
    }

    fn read_number(&mut self) -> u64 {
        let position = self.position;
        while self.ch.is_digit(10) {
            self.read_char();
        }
        self.input[position..self.position].parse::<u64>().unwrap()
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
        BOOL(true)
    } else if ident == "false" {
        BOOL(false)
    } else if ident == "if" {
        IF
    } else if ident == "else" {
        ELSE
    } else {
        IDENT(ident)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use token::Token::*;

    #[test]
    fn test_interpreter() {
        let input = "let five = 5;
        let ten = 10;

        let add = fn(x, y) {
          x + y;
        };

        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;

        if 5 < 10 {
            return true;
        } else if 5 == 5 {
            return false;
        } else if 5 != hello {
          return true;
        } else {
          return false;
        }";

        let mut lexer = Lexer::new(input);
        loop {
            let tok = lexer.next_token();
            match tok {
                ILLEGAL => panic!("Illegal token: {:#?}", tok),
                EOF => break,
                _ => continue,
            }
        }
    }
}
