mod lox;
mod scanner;
mod token;
mod token_type;

use lox::Lox;
use std::env::args;

fn main() {
    println!("Hello, Lox!");

    let mut lox = Lox::new();

    if args().len() > 2 {
        println!("Usage: rlox [script]");
    } else if args().len() == 2 {
        lox.run_file(args().nth(1).unwrap());
    } else {
        lox.run_prompt();
    }
}
