extern crate monkeyrs;

use std::io;
use std::io::prelude::*;

use monkeyrs::parser;
use monkeyrs::lexer;

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        println!("{}", line);
        let lexer = lexer::Lexer::new(&line);

        let mut parser = parser::Parser::new(lexer);

        let program = parser.parse_program();

        println!("{:#?}", program);
    }

}
