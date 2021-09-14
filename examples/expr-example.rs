use rlox::expr::*;
use rlox::token::Token;
use rlox::token_type::TokenType;

fn main() {
    let one = Literal {
        value: Token {
            token_type: TokenType::Number(1.0),
            lexeme: Some("1.0".to_string()),
            line: 0,
        },
    };

    let two = Literal {
        value: Token {
            token_type: TokenType::Number(2.0),
            lexeme: Some("2.0".to_string()),
            line: 0,
        },
    };

    let unary = Unary {
        operator: Token {
            token_type: TokenType::Bang,
            lexeme: Some("-".to_string()),
            line: 0,
        },
        right: one,
    };

    let grouping = Grouping { expression: unary };

    let binary = Binary {
        operator: Token {
            token_type: TokenType::Plus,
            lexeme: Some("+".to_string()),
            line: 0,
        },
        left: grouping,
        right: two,
    };

    println!("{}", binary);
}
