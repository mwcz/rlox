use crate::token_type::TokenType;

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<String>, // TODO should be Object
    pub line: usize,
}

impl Token {
    pub fn to_string(self) -> String {
        format!["{} {:?} {:?}", "TODO_TOKEN_TYPE", self.lexeme, self.literal]
    }
}
