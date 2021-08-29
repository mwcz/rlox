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
                '(' => self.add_token(TokenType::LeftParen, None),
                ')' => self.add_token(TokenType::RightParen, None),
                '{' => self.add_token(TokenType::LeftBrace, None),
                '}' => self.add_token(TokenType::RightBrace, None),
                ',' => self.add_token(TokenType::Comma, None),
                '.' => self.add_token(TokenType::Dot, None),
                '-' => self.add_token(TokenType::Minus, None),
                '+' => self.add_token(TokenType::Plus, None),
                ';' => self.add_token(TokenType::Semicolon, None),
                '*' => self.add_token(TokenType::Star, None),
                '!' => {
                    let token_type = if self.match_next('=') {
                        TokenType::BangEqual
                    } else {
                        TokenType::BangEqual
                    };
                    self.add_token(token_type, None);
                }
                '=' => {
                    let token_type = if self.match_next('=') {
                        TokenType::EqualEqual
                    } else {
                        TokenType::Equal
                    };
                    self.add_token(token_type, None);
                }
                '<' => {
                    let token_type = if self.match_next('=') {
                        TokenType::LessEqual
                    } else {
                        TokenType::Equal
                    };
                    self.add_token(token_type, None);
                }
                '>' => {
                    let token_type = if self.match_next('=') {
                        TokenType::GreaterEqual
                    } else {
                        TokenType::Equal
                    };
                    self.add_token(token_type, None);
                }
                '/' => {
                    let slash_follows = self.match_next('/');
                    if slash_follows {
                        while self.peek() != Some('\n') && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        self.add_token(TokenType::Slash, None);
                    }
                }
                ' ' => {}
                '\r' => {}
                '\t' => {}
                '\n' => self.line += 1,
                '"' => {
                    println!("\" encountered");
                    let next = self.peek();
                    println!("followed by: {}", next.unwrap());
                    while self.peek() != Some('"') && !self.is_at_end() {
                        println!("next is not \" and we're not at the end yet");
                        if self.peek() == Some('\n') {
                            println!("next is \\n");
                            self.line += 1;
                        }
                        self.advance();
                        println!("advance");
                    }
                    println!("next is \" or we're at the end now");
                    if self.is_at_end() {
                        println!("we're at the end, string is unterminated, uh oh");
                        Lox::error(self.line, "Unterminated string.".to_string());
                    } else {
                        println!("found ending \"");
                        self.advance();
                        let value = self.source[self.start + 1..self.current - 1].to_string();
                        println!("string value is: {}", value);
                        self.add_token(TokenType::String, Some(value));
                    }
                }
                _ => Lox::error(self.line, format!["Unexpected character: {}", c]),
            }
        }
    }

    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.current)
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
            literal: None,
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

    fn add_token(&mut self, token_type: TokenType, literal: Option<String>) {
        let lexeme = self.source[self.start..self.current].to_string();
        println!(" ↳ add_token text {}", lexeme);
        self.tokens.push(Token {
            token_type,
            lexeme,
            literal,
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
