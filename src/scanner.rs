use crate::lox::Lox;
use crate::token::Token;
use crate::token_type::TokenType;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,

    // location
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 0,
        }
    }

    pub fn scan_token(&mut self) {
        if let Some(c) = self.advance() {
            println!("scanned token: {:?}", c);
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
                    let token_type = if self.match_next('=') {
                        TokenType::BangEqual
                    } else {
                        TokenType::Bang
                    };
                    self.add_token(token_type);
                }
                '=' => {
                    let token_type = if self.match_next('=') {
                        TokenType::EqualEqual
                    } else {
                        TokenType::Equal
                    };
                    self.add_token(token_type);
                }
                '<' => {
                    let token_type = if self.match_next('=') {
                        TokenType::LessEqual
                    } else {
                        TokenType::Equal
                    };
                    self.add_token(token_type);
                }
                '>' => {
                    let token_type = if self.match_next('=') {
                        TokenType::GreaterEqual
                    } else {
                        TokenType::Equal
                    };
                    self.add_token(token_type);
                }
                '/' => {
                    let slash_follows = self.match_next('/');
                    if slash_follows {
                        while self.peek() != Some('\n') && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        self.add_token(TokenType::Slash);
                    }
                }
                ' ' => {}
                '\r' => {}
                '\t' => {}
                '\n' => self.line += 1,
                '"' => self.string(),
                '0'..='9' => {
                    if self.is_digit(Some(c)) {
                        self.number();
                    } else {
                        Lox::error(self.line, format!["Unexpected character: {}", c]);
                    }
                }
                _ => Lox::error(self.line, format!["Unexpected character: {}", c]),
            }
        }
    }

    fn string(&mut self) {
        while self.peek() != Some('"') && !self.is_at_end() {
            if self.peek() == Some('\n') {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            Lox::error(self.line, "Unterminated string.".to_string());
        } else {
            self.advance();
            let value = self.source[self.start + 1..self.current - 1].to_string();
            self.add_token(TokenType::String(value));
        }
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == Some('.') && self.is_digit(self.peek_next()) {
            self.advance();
            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let parsed_number = self.source[self.start..self.current]
            .parse::<f64>()
            .unwrap();

        println!("parsed_number {}", parsed_number);
        self.add_token(TokenType::Number(parsed_number));
    }

    fn is_digit(&self, c: Option<char>) -> bool {
        return match c {
            Some(c) => c >= '0' && c <= '9',
            None => false,
        };
    }

    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.current)
    }

    fn peek_next(&self) -> Option<char> {
        self.source.chars().nth(self.current + 1)
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

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;

        return true;
    }

    fn add_token(&mut self, token_type: TokenType) {
        let lexeme = self.source[self.start..self.current].to_string();
        println!(" ↳ add_token text {}", lexeme);
        self.tokens.push(Token {
            token_type,
            lexeme,
            line: self.line,
        });
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.source.chars().nth(self.current);
        self.current += 1;

        c
    }

    pub fn is_at_end(&self) -> bool {
        // TODO revisit source.len() if non-ASCII chars are wanted
        self.current >= self.source.len()
    }
}
