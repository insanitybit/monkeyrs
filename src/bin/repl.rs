extern crate monkeyrs;

use std::io;
use std::io::prelude::*;

use monkeyrs::parser;
use monkeyrs::lexer;
use monkeyrs::evaluator;

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let lexer = lexer::Lexer::new(&line);

        let mut parser = parser::Parser::new(lexer);

        let program = parser.parse_program();

        let evaluated = evaluator::eval_program(program);

        println!("{}", evaluated.unwrap().inspect());
    }

}
