use std::{env, fs};
use crate::lexer::Lexer;
use crate::token::TokenType::Eof;

mod token;
mod lexer;

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1])
        .expect("Should have been able to read the file");

    let mut lexer = Lexer::init(contents.chars());
    let mut token = lexer.next();

    while token.token_type != Eof {
        println!("{:?}", token);
        token = lexer.next();
    }
    println!("{:?}", token)
}
