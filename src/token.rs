use std::fmt::{self, Display, Formatter};

use crate::token_type::TokenType;

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: Option<String>,
    pub line: usize,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.lexeme {
            Some(lexeme) => {
                write!(
                    f,
                    "Token {{ {}, lexeme: {}, line: {} }}",
                    self.token_type, lexeme, self.line
                )
            }
            None => {
                write!(f, "Token {{ {}, line: {} }}", self.token_type, self.line)
            }
        }
    }
}
