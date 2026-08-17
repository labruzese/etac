use core::fmt;
use std::fmt::Display;

use etac_intern::{EtacInterner, Symbol};
use logos::Logos;

use crate::{internal_error::InternalLexerError, internal_error_unknown_token};

macro_rules! count_idents {
    () => { 0usize };
    ($h:ident $($t:ident)*) => { 1usize + count_idents!($($t)*) };
}

macro_rules! tokens {
    // `Token::Foo(..)` or `Token::Foo`
    (@match_tok $id:ident($data:tt)) => { Token::$id(..) };
    (@match_tok $id:ident)           => { Token::$id };

    // `Token::Foo(v)` or `Token::Foo`
    (@bind_tok $id:ident($v:ident: $data:tt)) => { Token::$id($v) };
    (@bind_tok $id:ident)                     => { Token::$id };

    // Display for Token [ex: Token::Identifier(Symbol) => "Identifier(foo)"]
    (@tok_display $s:ident, $f:ident, $id:ident, $v:ident: Symbol) => {
        write!($f, "{}({})", stringify!($id), $s.res.resolve($v).escape_default())
    };
    (@tok_display $s:ident, $f:ident, $id:ident, $v:ident: char) => {
        write!($f, "{}({})", stringify!($id), $v.escape_default())
    };
    (@tok_display $s:ident, $f:ident, $id:ident, $v:ident: $other:tt) => {
        write!($f, "{}({})", stringify!($id), $v)
    };
    (@tok_display $s:ident, $f:ident, $id:ident) => {
        write!($f, "{}", stringify!($id))
    };

    (
        Logos { $(# $meta:tt)* }
        $(
            $(# $attr:tt)+
            $id:ident $(($data:tt))?
        ),* $(,)?
    ) => {
        tokens! {
            @build
            meta: [ $(# $meta)* ]
            ids:  [ $( $id $(($data))? ),* ]
            disp: [ ]
            todo: [ $( { $(# $attr)+ } $id $(($data))? , )* ]
            variants: [ ]
            arms: [ ]
        }
    };

    (@build
        meta: [ $($meta:tt)* ]
        ids:  [ $($ids:tt)* ]
        disp: [ $($disp:tt)* ]
        todo: [ { $($attrs:tt)* } $id:ident $(($data:tt))? , $($rest:tt)* ]
        variants: [ $($variants:tt)* ]
        arms: [ $($arms:tt)* ]
    ) => {
        tokens! {
            @attrs
            scan: [ $($attrs)* ]
            kept: [ ]
            name: [ ]
            toks: [ ]
            cur:  [ $id $(($data))? ]
            outer: {
                meta: [ $($meta)* ]
                ids:  [ $($ids)* ]
                disp: [ $($disp)* ]
                todo: [ $($rest)* ]
                variants: [ $($variants)* ]
                arms: [ $($arms)* ]
            }
        }
    };

    // Done, an explicit #[name] was given.
    (@attrs
        scan: [ ] kept: [ $($kept:tt)* ] name: [ $n:literal ] toks: [ $($toks:tt)* ]
        cur: [ $id:ident $(($data:tt))? ]
        outer: {
            meta: [ $($meta:tt)* ] ids: [ $($ids:tt)* ] disp: [ $($disp:tt)* ]
            todo: [ $($todo:tt)* ] variants: [ $($variants:tt)* ] arms: [ $($arms:tt)* ]
        }
    ) => {
        tokens! {
            @build
            meta: [ $($meta)* ]
            ids:  [ $($ids)* ]
            disp: [ $($disp)* $id : $n , ]
            todo: [ $($todo)* ]
            variants: [ $($variants)* $($kept)* $id $(($data))? , ]
            arms: [ $($arms)* ($n) => { $crate::Token::$id }; ]
        }
    };

    // Done, no #[name], exactly one #[token] -> that literal is the name.
    (@attrs
        scan: [ ] kept: [ $($kept:tt)* ] name: [ ] toks: [ $t:literal ]
        cur: [ $id:ident $(($data:tt))? ]
        outer: {
            meta: [ $($meta:tt)* ] ids: [ $($ids:tt)* ] disp: [ $($disp:tt)* ]
            todo: [ $($todo:tt)* ] variants: [ $($variants:tt)* ] arms: [ $($arms:tt)* ]
        }
    ) => {
        tokens! {
            @build
            meta: [ $($meta)* ]
            ids:  [ $($ids)* ]
            disp: [ $($disp)* $id : $t , ]
            todo: [ $($todo)* ]
            variants: [ $($variants)* $($kept)* $id $(($data))? , ]
            arms: [ $($arms)* ($t) => { $crate::Token::$id }; ]
        }
    };

    // Done, but ambiguous: zero or 2+ #[token]s and no #[name].
    (@attrs
        scan: [ ] kept: [ $($kept:tt)* ] name: [ ] toks: [ $($toks:tt)* ]
        cur: [ $id:ident $(($data:tt))? ]
        outer: { $($outer:tt)* }
    ) => {
        compile_error!(concat!(
            "token variant `", stringify!($id),
            "` needs either exactly one #[token(..)] or an explicit #[name(\"..\")]"
        ));
    };

    // #[name("..")]
    (@attrs
        scan: [ #[name($n:literal)] $($rest:tt)* ]
        kept: [ $($kept:tt)* ] name: [ $($old:tt)* ] toks: [ $($toks:tt)* ]
        $($tail:tt)*
    ) => {
        tokens! {
            @attrs scan: [ $($rest)* ] kept: [ $($kept)* ] name: [ $n ] toks: [ $($toks)* ]
            $($tail)*
        }
    };

    // #[token("..", optional callback)]
    (@attrs
        scan: [ #[token($lit:literal $($targs:tt)*)] $($rest:tt)* ]
        kept: [ $($kept:tt)* ] name: [ $($name:tt)* ] toks: [ $($toks:tt)* ]
        $($tail:tt)*
    ) => {
        tokens! {
            @attrs
            scan: [ $($rest)* ]
            kept: [ $($kept)* #[token($lit $($targs)*)] ]
            name: [ $($name)* ]
            toks: [ $($toks)* $lit ]
            $($tail)*
        }
    };

    // everything else (#[regex(..)], doc comments, ...) passes through.
    (@attrs
        scan: [ # $other:tt $($rest:tt)* ]
        kept: [ $($kept:tt)* ]
        $($tail:tt)*
    ) => {
        tokens! { @attrs scan: [ $($rest)* ] kept: [ $($kept)* # $other ] $($tail)* }
    };

    // everything is accumulated, emit.
    (@build
        meta: [ $($meta:tt)* ]
        ids:  [ $( $id:ident $(($data:tt))? ),* $(,)? ]
        disp: [ $( $did:ident : $dlit:literal , )* ]
        todo: [ ]
        variants: [ $($variants:tt)* ]
        arms: [ $($arms:tt)* ]
    ) => {
        $($meta)*
        pub enum Token {
            $($variants)*
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[repr(u8)]
        pub enum TokenKind {
            $($id,)*
        }

        impl TokenKind {
            pub const COUNT: usize = count_idents!($($id)*);
        }

        impl fmt::Display for TokenKind {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                // `f.write_str(<expr>)` rather than `write!(f, <macro call>)`:
                // write!'s first argument must be a literal format string.
                f.write_str(match self {
                    $( TokenKind::$did => $dlit, )*
                })
            }
        }

        impl Token {
            #[inline]
            pub fn kind(&self) -> TokenKind {
                match self {
                    $( tokens!(@match_tok $id $(($data))?) => TokenKind::$id, )*
                }
            }
        }

        pub struct TokenDisplay<R: etac_intern::Resolver> {
            tok: Token,
            res: R,
        }

        impl<R: etac_intern::Resolver> TokenDisplay<R> {
            pub fn new(tok: Token, res: R) -> Self {
                TokenDisplay { tok, res }
            }
        }

        impl<R: etac_intern::Resolver> Display for TokenDisplay<R> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match &self.tok {
                    $(
                        tokens!(@bind_tok $id $((v: $data))?)
                            => tokens!(@tok_display self, f, $id $(, v: $data)?),
                    )*
                }
            }
        }
    };
}

tokens! {
    Logos {
        #[derive(Debug, Clone, PartialEq, Logos)]
        #[logos(skip r"[ \t\n\f\r]+")]
        #[logos(skip(r"//[^\n]*", allow_greedy = true))]
        #[logos(extras = (u32, &'s mut EtacInterner))]
        #[logos(error(InternalLexerError, internal_error_unknown_token))]
    }

    #[name("eof")]      EOF,
    #[name("error")]    Error,

    // Keywords
    #[token("use")]     KwUse,
    #[token("length")]  KwLength,
    #[token("while")]   KwWhile,
    #[token("if")]      KwIf,
    #[token("else")]    KwElse,
    #[token("return")]  KwReturn,
    #[token("int")]     KwInt,
    #[token("bool")]    KwBool,

    #[token(";")]       SemiColon,
    #[token("_")]       Discard,
    #[token(":")]       Colon,
    #[token("=")]       Assign,
    #[token(",")]       Comma,

    #[token("(")]       LParen,
    #[token(")")]       RParen,
    #[token("[")]       LBracket,
    #[token("]")]       RBracket,
    #[token("{")]       BlockOpen,
    #[token("}")]       BlockClose,

    #[token("*")]       Star,
    #[token("*>>")]     HMul,
    #[token("/")]       FSlash,
    #[token("%")]       Percent,
    #[token("!")]       Bang,
    #[token("-")]       Minus,
    #[token("+")]       Plus,

    #[token("==")]      EqEq,
    #[token("!=")]      Neq,
    #[token(">")]       Gt,
    #[token(">=")]      Ge,
    #[token("<")]       Lt,
    #[token("<=")]      Le,

    #[token("&")]       Land,
    #[token("|")]       Lor,

    #[token("true", |_| true)]
    #[token("false", |_| false)]
    #[name("a bool literal")]
    BoolLiteral(bool),

    #[regex(r"'([^'\\]|\\(.|x\{[0-9A-Fa-f]{1,6}\}))*'", crate::strings::parse_char)]
    #[name("a char literal")]
    CharLiteral(char),

    #[regex(r#""([^"\\]|\\(.|x\{[0-9A-Fa-f]{1,6}\}))*""#, |lex| {
        crate::strings::parse_str(lex).map(|s| lex.extras.1.get_or_intern(&s))
    })]
    #[name("a string literal")]
    StrLiteral(Symbol),

    #[regex(r"[1-9][0-9]*|0", crate::parse_int)]
    #[name("an int literal")]
    IntLiteral(u64),

    #[regex(r"[a-zA-Z][a-zA-Z0-9_']*", |lex| lex.extras.1.get_or_intern(lex.slice()))]
    #[name("an identifier")]
    Identifier(Symbol),
}
