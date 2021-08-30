#[allow(dead_code)]
#[derive(Debug)]
/// The types of Lox tokens.
pub enum TokenType {
    // Single-character types
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two-character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier(String),
    LoxString(String),
    Number(f64),

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

impl TokenType {
    /// If this is a TokenType::Identifier, and if the Identifier's text is a keyword, return the
    /// TokenType for that keyword, otherwise return self.
    /// For example, TokenType::Identifier("var".to_string()).to_keyword() returns TokenType::Var
    pub fn to_keyword(self) -> Self {
        match self {
            TokenType::Identifier(ref text) => match text.as_str() {
                "and" => TokenType::And,
                "class" => TokenType::Class,
                "else" => TokenType::Else,
                "false" => TokenType::False,
                "fun" => TokenType::Fun,
                "for" => TokenType::For,
                "if" => TokenType::If,
                "nil" => TokenType::Nil,
                "or" => TokenType::Or,
                "print" => TokenType::Print,
                "return" => TokenType::Return,
                "super" => TokenType::Super,
                "this" => TokenType::This,
                "true" => TokenType::True,
                "var" => TokenType::Var,
                "while" => TokenType::While,
                _ => self,
            },
            _ => self,
        }
    }
}
