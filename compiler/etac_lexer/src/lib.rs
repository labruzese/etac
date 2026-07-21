//! Lexer
#![allow(clippy::cast_possible_truncation)]

use std::{
    fmt::{self, Display},
    num::ParseIntError,
};

use etac_errors::dcx::{DiagCtx, Diag};
use etac_cache::sources::Span;
use logos::Logos;

mod internal_error;
use internal_error::{InternalLexerError, lexer_error};

pub struct SourceToken<'src> {
    pub tok: RawToken<'src>,
    pub span: Span,
}

pub trait ILexer<'src, 'dcx>: Iterator<Item = Result<SourceToken<'src>, Diag<'dcx>>> {}

type LogosLexer<'src> = logos::Lexer<'src, RawToken<'src>>;

pub struct EtaLexer<'src, 'dcx> {
    inner: logos::SpannedIter<'src, RawToken<'src>>,
    dcx: &'dcx DiagCtx,
}

impl<'src, 'dcx> EtaLexer<'src, 'dcx> {
    pub fn new(base: u32, source: &'src str, diag_context: &'dcx DiagCtx) -> Self {
        Self {
            dcx: diag_context,
            inner: <RawToken as Logos>::lexer_with_extras(source, base).spanned(),
        }
    }
}

impl<'src, 'dcx> ILexer<'src, 'dcx> for EtaLexer<'src, 'dcx> {}
// transformed for lalrpop
impl<'src, 'dcx> Iterator for EtaLexer<'src, 'dcx> {
    type Item = Result<SourceToken<'src>, Diag<'dcx>>;

    fn next(&mut self) -> Option<Self::Item> {
        let (next, local_span) = self.inner.next()?;
        let base = self.inner.extras;
        let span = Span::new(base + local_span.start as u32, base + local_span.end as u32);
        match next {
            Ok(tok) => Some(Ok(SourceToken { tok, span })),
            Err(ie) => {
                let mut d = self.dcx.err(ie.span, ie.message);
                if let Some(l) = ie.plabel { d = d.with_primary_label(l); }
                if let Some(l) = ie.note { d = d.with_note(l); }
                Some(Err(d))
            }
        }
    }
}

mod strings;

#[derive(Debug, Clone, PartialEq, Logos)]
#[logos(lifetime = 's)]
#[logos(skip r"[ \t\n\f\r]+")]
#[logos(skip(r"//[^\n]*", allow_greedy = true))]
#[logos(extras = u32)]
#[logos(error(InternalLexerError, internal_error_unknown_token))]
pub enum RawToken<'s> {
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

    #[regex(r"'([^'\\]|\\(.|x\{[0-9A-Fa-f]{1,6}\}))*'", strings::parse_char)]
    CharLiteral(u32),

    #[regex(r#""([^"\\]|\\(.|x\{[0-9A-Fa-f]{1,6}\}))*""#, strings::parse_str)]
    StrLiteral(String),

    #[regex(r"[a-zA-Z][a-zA-Z0-9_']*", |lex| lex.slice())]
    Identifier(&'s str),

    #[regex(r"-?[1-9][0-9]*|0", parse_int)]
    Integer(i64),

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

// Callbacks

fn parse_int(lex: &mut LogosLexer<'_>) -> Result<i64, InternalLexerError> {
    lex.slice().parse::<i64>().map_err(|err: ParseIntError| lexer_error! {
        span = current_span(lex),
        message = format!("illegal integer literal: {}", err),
        plabel = format!("this number is too {}", 
            if err.to_string().contains("large") { "large" } 
            else if err.to_string().contains("small") { "small" } 
            else { panic!("rust error message is weird") }),
        note = "eta only supports ints in the range [-2^63, 2^63)"
    })
}

impl Display for RawToken<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
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
            RawToken::CharLiteral(c) => write!(
                f,
                "character {}",
                char::from_u32(*c)
                    .expect("illegal char somehow lexed")
                    .escape_default()
                    .collect::<String>()
                    .replace("\\u{", "\\x{")
            ),
            RawToken::StrLiteral(s) => write!(
                f,
                "string {}",
                s.escape_default().collect::<String>().replace("\\u{", "\\x{")
            ),
            RawToken::Identifier(s) => write!(f, "id {s}"),
            RawToken::Integer(n) => write!(f, "integer {n}"),
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

// helpers

fn current_span<'s>(lex: &logos::Lexer<'s, RawToken<'s>>) -> Span {
    Span::new(
        lex.extras + lex.span().start as u32,
        lex.extras + lex.span().end as u32
    )
}

fn internal_error_unknown_token<'s>(lex: &mut logos::Lexer<'s, RawToken<'s>>) -> InternalLexerError {
    lexer_error! {
        span = current_span(lex),
        message = "unknown token",
        plabel = "this token",
    }
}
