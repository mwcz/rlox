use crate::scanner::Scanner;
use std::fs;
use std::io;

pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Self { had_error: false }
    }

    pub fn run_file(&mut self, script_path: String) {
        println!("Running file {}", script_path);

        if let Ok(file_contents) = fs::read(&script_path) {
            let code = String::from_utf8_lossy(&file_contents);
            self.run(code.to_string());
        } else {
            println!("Could not read file {}", script_path);
        }
    }

    pub fn run_prompt(&mut self) {
        println!("Running REPL");

        loop {
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(n) => {
                    self.run(input.to_string());
                    self.had_error = false;
                    // println!("{} bytes read", n);
                    // println!("  {}", input);
                    // input.clear();
                }
                Err(why) => println!("error: {}", why),
            }
        }
    }

    fn run(&mut self, source: String) {
        println!("Running source: {}", source);
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();

        for token in tokens {
            println!("{:?}", token);
        }
    }

    pub fn error(line: usize, message: String) {
        Self::report(line, "".to_string(), message);
    }

    fn report(line: usize, weriamz: String, message: String) {
        println!("[line {}] Error{}: {}", line, weriamz, message);
    }
}
