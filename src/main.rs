#![allow(dead_code)]
#[macro_use]
extern crate lazy_static;

pub mod token;
pub mod lexer;
pub mod ast;
pub mod parser;

fn main() {
    println!("Hello, world!");
}
