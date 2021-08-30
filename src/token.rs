use crate::token_type::TokenType;

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}
