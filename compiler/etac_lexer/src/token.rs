use core::fmt;
use std::fmt::Display;

use etac_intern::{EtacInterner, Symbol};
use logos::Logos;

use crate::{internal_error::InternalLexerError, internal_error_unknown_token};

pub type SourceToken = (u32, RawToken, u32);

#[derive(Debug, Clone, PartialEq, Logos)]
#[logos(skip r"[ \t\n\f\r]+")]
#[logos(skip(r"//[^\n]*", allow_greedy = true))]
#[logos(extras = (u32, &'s mut EtacInterner))]
#[logos(error(InternalLexerError, internal_error_unknown_token))]
pub enum RawToken {
    // Keywords
    #[token("use")]
    KeywordUse,
    #[token("length")]
    KeywordLength,
    #[token("while")]
    KeywordWhile,
    #[token("if")]
    KeywordIf,
    #[token("else")]
    KeywordElse,
    #[token("return")]
    KeywordReturn,
    #[token("int")]
    KeywordInt,
    #[token("bool")]
    KeywordBool,

    // Punctuation
    #[token(";")]
    SemiColon,
    #[token("_")]
    Discard,
    #[token(":")]
    OfType,
    #[token("=")]
    Assign,
    #[token(",")]
    Comma,

    #[token("true", |_| true)]
    #[token("false", |_| false)]
    BoolLiteral(bool),

    #[regex(r"'([^'\\]|\\(.|x\{[0-9A-Fa-f]{1,6}\}))*'", crate::strings::parse_char)]
    CharLiteral(char),

    #[regex(r#""([^"\\]|\\(.|x\{[0-9A-Fa-f]{1,6}\}))*""#, |lex| { 
        crate::strings::parse_str(lex).map(|s| lex.extras.1.get_or_intern(&s))
    })]
    StrLiteral(Symbol),

    #[regex(r"[a-zA-Z][a-zA-Z0-9_']*", |lex| lex.extras.1.get_or_intern(lex.slice()))]
    Identifier(Symbol),

    #[regex(r"[1-9][0-9]*|0", crate::parse_int)]
    Integer(u64),

    // Brackets and braces
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token("{")]
    BlockOpen,
    #[token("}")]
    BlockClose,

    // Arithmetic operators
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

    // Relational operators
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

    // Logical operators
    #[token("&")]
    Land,
    #[token("|")]
    Lor,
}

pub struct TokenDisplay<R: etac_intern::Resolver> {
    tok: RawToken,
    resolver: R,
}

impl<R: etac_intern::Resolver> TokenDisplay<R> {
    pub fn new(tok: RawToken, resolver: R) -> Self {
        TokenDisplay { tok, resolver }
    }
}

impl<R: etac_intern::Resolver> Display for TokenDisplay<R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.tok {
            RawToken::StrLiteral(sym) => write!(f, "\"{}\"", self.resolver.resolve(sym).escape_default().collect::<String>().replace("\\u{", "\\x{")),
            RawToken::Identifier(sym) => write!(f, "{}", self.resolver.resolve(sym)),
            RawToken::KeywordUse => write!(f, "use"),
            RawToken::KeywordLength => write!(f, "length"),
            RawToken::KeywordWhile => write!(f, "while"),
            RawToken::KeywordIf => write!(f, "if"),
            RawToken::KeywordElse => write!(f, "else"),
            RawToken::KeywordReturn => write!(f, "return"),
            RawToken::KeywordInt => write!(f, "int"),
            RawToken::KeywordBool => write!(f, "bool"),
            RawToken::SemiColon => write!(f, ";"),
            RawToken::Discard => write!(f, "_"),
            RawToken::OfType => write!(f, ":"),
            RawToken::Assign => write!(f, "="),
            RawToken::Comma => write!(f, ","),
            RawToken::BoolLiteral(b) => write!(f, "{b}"),
            RawToken::CharLiteral(c) => write!(f, "'{}'", c.escape_default().collect::<String>().replace("\\u{", "\\x{")),
            RawToken::Integer(n) => write!(f, "{n}"),
            RawToken::LParen => write!(f, "("),
            RawToken::RParen => write!(f, ")"),
            RawToken::LBracket => write!(f, "["),
            RawToken::RBracket => write!(f, "]"),
            RawToken::BlockOpen => write!(f, "{{"),
            RawToken::BlockClose => write!(f, "}}"),
            RawToken::OperatorMul => write!(f, "*"),
            RawToken::OperatorHighMul => write!(f, "*>>"),
            RawToken::OperatorDiv => write!(f, "/"),
            RawToken::OperatorMod => write!(f, "%"),
            RawToken::OperatorNot => write!(f, "!"),
            RawToken::Minus => write!(f, "-"),
            RawToken::OperatorAdd => write!(f, "+"),
            RawToken::RelOpEq => write!(f, "=="),
            RawToken::RelOpNeq => write!(f, "!="),
            RawToken::RelOpGr => write!(f, ">"),
            RawToken::RelOpGe => write!(f, ">="),
            RawToken::RelOpLt => write!(f, "<"),
            RawToken::RelOpLe => write!(f, "<="),
            RawToken::Land => write!(f, "&"),
            RawToken::Lor => write!(f, "|"),

        }
    }
}
