mod lox;
mod scanner;
mod token;
mod token_type;

use lox::Lox;
use std::cmp::Ordering;
use std::env::args;

fn main() {
    println!("Hello, Lox!");

    let mut lox = Lox::new();

    match args().len().cmp(&2) {
        Ordering::Greater => println!("Usage: rlox [script]"),
        Ordering::Equal => lox.run_file(args().nth(1).unwrap()),
        Ordering::Less => lox.run_prompt(),
    }
}
