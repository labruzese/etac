use logos::Logos;
use std::fmt;
use std::num::ParseIntError;

use crate::{
    error,
    errors::{self, Diagnostic},
};

pub type Lexer<'input> = logos::Lexer<'input, Token>;

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(
    skip r"[ \t\n\f]+", // whitespace
    skip r"//.*\n?", // // comments
    skip r"/\*([^*]|\*[^/])*\*/", // /* comments */ 
    error = errors::Diagnostic,
)]
pub enum Token {
    #[regex("[a-zA-Z][a-zA-Z0-9_’']*", |lex| lex.slice().to_string())]
    Identifier(String),
    #[regex("[1-9][0-9]*", |lex| lex.slice().parse())]
    Integer(i32),

    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,

    #[token("*")]
    OperatorMul,
    #[token("*>>")]
    OperatorHighMul,
    #[token("/")]
    OperatorDiv,
    #[token("%")]
    OperatorMod,

    #[token("!")]
    OperatorNot,
    #[token("-")]
    Minus,
    #[token("+")]
    OperatorAdd,

    #[token("==")]
    RelOpEq,
    #[token("!=")]
    RelOpNeq,
    #[token(">")]
    RelOpGr,
    #[token(">=")]
    RelOpGe,
    #[token("<")]
    RelOpLt,
    #[token("<=")]
    RelOpLe,
    #[token("&")]
    Land,
    #[token("|")]
    Lor,
}

impl From<ParseIntError> for Diagnostic {
    fn from(err: ParseIntError) -> Self {
        error!("Illegal int: {}", err)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Identifier(name) => write!(f, "id {}", name),
            Token::Integer(i) => write!(f, "integer {}", i),
            _ => write!(f, "{self:?}"),
        }
    }
}
