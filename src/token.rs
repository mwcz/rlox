use std::fmt::{self, Display, Formatter};

use crate::token_type::TokenType;

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Token {{ {}, lexeme: {}, line: {} }}",
            self.token_type, self.lexeme, self.line
        )
    }
}
