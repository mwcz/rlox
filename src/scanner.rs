use crate::lox::Lox;
use crate::token::Token;
use crate::token_type::TokenType;

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,

    // location
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 0,
        }
    }

    pub fn scan_token(&mut self) {
        if let Some(c) = self.advance() {
            // println!("scanned token: {:?}", c);
            match c {
                '(' => self.add_token(TokenType::LeftParen),
                ')' => self.add_token(TokenType::RightParen),
                '{' => self.add_token(TokenType::LeftBrace),
                '}' => self.add_token(TokenType::RightBrace),
                ',' => self.add_token(TokenType::Comma),
                '.' => self.add_token(TokenType::Dot),
                '-' => self.add_token(TokenType::Minus),
                '+' => self.add_token(TokenType::Plus),
                ';' => self.add_token(TokenType::Semicolon),
                '*' => self.add_token(TokenType::Star),
                '!' => {
                    let token_type = if self.match_next(&'=') {
                        TokenType::BangEqual
                    } else {
                        TokenType::Bang
                    };
                    self.add_token(token_type);
                }
                '=' => {
                    let token_type = if self.match_next(&'=') {
                        TokenType::EqualEqual
                    } else {
                        TokenType::Equal
                    };
                    self.add_token(token_type);
                }
                '<' => {
                    let token_type = if self.match_next(&'=') {
                        TokenType::LessEqual
                    } else {
                        TokenType::Equal
                    };
                    self.add_token(token_type);
                }
                '>' => {
                    let token_type = if self.match_next(&'=') {
                        TokenType::GreaterEqual
                    } else {
                        TokenType::Equal
                    };
                    self.add_token(token_type);
                }
                '/' => {
                    let slash_follows = self.match_next(&'/');
                    if slash_follows {
                        while self.peek() != Some(&'\n') && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        let star_follows = self.match_next(&'*');
                        if star_follows {
                            // nested comments must be matching
                            // beginning of /* ... */ comment
                            println!("COMMENT: starting /*");
                            let mut running_comment_count = 1;
                            while running_comment_count > 0 {
                                println!("COMMENT: loop iter, running = {}", running_comment_count);
                                if self.is_at_end() {
                                    break;
                                }
                                if self.peek() == Some(&'/') && self.peek_next() == Some(&'*') {
                                    running_comment_count += 1;
                                    println!(
                                        "COMMENT: /* found, running = {}",
                                        running_comment_count
                                    );
                                    self.advance();
                                } else if self.peek() == Some(&'*')
                                    && self.peek_next() == Some(&'/')
                                {
                                    running_comment_count -= 1;
                                    self.advance();
                                }
                                self.advance();
                            }
                        } else {
                            // just a slash, aka divide
                            self.add_token(TokenType::Slash);
                        }
                    }
                }
                ' ' => {}
                '\r' => {}
                '\t' => {}
                '\n' => self.line += 1,
                '"' => self.string(),
                '0'..='9' => self.number(),
                'a'..='z' | 'A'..='Z' => self.identifier(),
                _ => {
                    let ch = self.peek().unwrap();
                    Lox::error(&self.line, format!("Unexpected character: {}", ch))
                }
            }
        }
    }

    fn identifier(&mut self) {
        while Scanner::is_alphanumeric(self.peek()) {
            self.advance();
        }

        self.add_token(
            TokenType::Identifier(
                self.source[self.start..self.current]
                    .iter()
                    .collect::<String>(),
            )
            .check_keyword(),
        );
    }

    fn is_alpha(c: Option<&char>) -> bool {
        match c {
            // Some(c) => (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_',
            Some(c) => c.is_ascii_alphabetic() || c == &'_',
            None => false,
        }
    }

    fn is_alphanumeric(c: Option<&char>) -> bool {
        Scanner::is_alpha(c) || Scanner::is_digit(c)
    }

    fn string(&mut self) {
        while self.peek() != Some(&'"') && !self.is_at_end() {
            if self.peek() == Some(&'\n') {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            Lox::error(&self.line, "Unterminated string.".to_string());
        } else {
            self.advance();
            self.add_token(TokenType::LoxString(
                self.source[self.start + 1..self.current - 1]
                    .iter()
                    .collect::<String>(),
            ));
        }
    }

    fn number(&mut self) {
        while Scanner::is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == Some(&'.') && Scanner::is_digit(self.peek_next()) {
            self.advance();
            while Scanner::is_digit(self.peek()) {
                self.advance();
            }
        }

        let parsed_number = self.source[self.start..self.current]
            .iter()
            .collect::<String>() // become a string
            .parse::<f64>() // parse to f64
            .unwrap();

        self.add_token(TokenType::Number(parsed_number));
    }

    fn is_digit(c: Option<&char>) -> bool {
        match c {
            Some(c) => c.is_digit(10),
            None => false,
        }
    }

    fn peek(&self) -> Option<&char> {
        // self.source.chars().nth(self.current)
        self.source.get(self.current)
    }

    fn peek_next(&self) -> Option<&char> {
        // self.source.chars().nth(self.current + 1)
        self.source.get(self.current + 1)
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
            line: self.line,
        });

        &self.tokens
    }

    fn match_next(&mut self, expected: &char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.peek().unwrap() != expected {
            return false;
        }

        self.current += 1;

        true
    }

    fn add_token(&mut self, token_type: TokenType) {
        // println!("add_token: {}", lexeme);
        self.tokens.push(Token {
            token_type,
            lexeme: self.source[self.start..self.current]
                .iter()
                .collect::<String>(),
            line: self.line,
        });
    }

    fn advance(&mut self) -> Option<&char> {
        let c = self.source.get(self.current); // TODO should use peek here, but peek borrows self and prevents advance from incrementing self.current

        self.current += 1;

        c
    }

    pub fn is_at_end(&self) -> bool {
        // TODO revisit source.len() if non-ASCII chars are wanted
        self.current >= self.source.len()
    }
}
