// Enum Expr version
// use crate::token::Token;

// #[allow(dead_code)]
// enum Expr {
//     Literal {
//         value: Token,
//     },
//     Grouping {
//         expression: Box<Expr>,
//     },
//     Unary {
//         operator: Box<Expr>,
//         right: Box<Expr>,
//     },
//     Binary {
//         left: Box<Expr>,
//         right: Box<Expr>,
//     },
//     Operator(Token),
// }

use crate::token::Token;
use std::fmt::Display;

///////////////////
//  Expressions  //
///////////////////

pub trait Expr {}

impl Expr for Literal {}

impl<R> Expr for Unary<R> where R: Expr {}

impl<L, R> Expr for Binary<L, R>
where
    L: Expr,
    R: Expr,
{
}

impl<E> Expr for Grouping<E> where E: Expr {}

//////////////////////////////
//  Displaying expressions  //
//////////////////////////////

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.value.lexeme.as_ref().unwrap_or(&"".to_string())
        )
    }
}

impl<R> Display for Unary<R>
where
    R: Expr + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.operator.lexeme {
            Some(lexeme) => write!(f, "({} {})", lexeme, self.right),
            None => write!(f, "({})", self.right),
        }
    }
}

impl<L, R> Display for Binary<L, R>
where
    L: Expr + Display,
    R: Expr + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({} {} {})",
            self.operator.lexeme.as_ref().unwrap_or(&"".to_string()),
            self.left,
            self.right
        )
    }
}

impl<E> Display for Grouping<E>
where
    E: Expr + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(group {})", self.expression)
    }
}

////////////////
//  Literals  //
////////////////

#[allow(dead_code)]
// TODO restrict value to TokenTypes that are valid literals: Number, LoxString, True, False, Nil
#[derive(Debug)]
pub struct Literal {
    pub value: Token,
}

/////////////////
//  Groupings  //
/////////////////

#[allow(dead_code)]
#[derive(Debug)]
pub struct Grouping<G: Expr> {
    pub expression: G,
}

/////////////
//  Unary  //
/////////////

#[allow(dead_code)]
#[derive(Debug)]
// TODO restrict operator to only TokenTypes that are prefix operators: Minus, Bang
pub struct Unary<R: Expr> {
    pub operator: Token,
    pub right: R,
}

//////////////
//  Binary  //
//////////////

#[allow(dead_code)]
#[derive(Debug)]
// TODO restrict operator to only TokenTypes that are infix operators: EqualEqual, BangEqual, Less, LessEqual, Greater, GreaterEqual, Plus, Minus, Star, Slash
pub struct Binary<L: Expr, R: Expr> {
    pub left: L,
    pub operator: Token,
    pub right: R,
}

/////////////////
//  Operators  //
/////////////////

#[allow(dead_code)]
#[derive(Debug)]
pub struct Operator {
    pub operator: Token,
}
