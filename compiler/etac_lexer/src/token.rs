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
            pats: [ ]
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
        pats: [ $($p:tt)+ ]
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
            disp: [ $($disp)* $id : $n : prose , ]
            todo: [ $($todo)* ]
            variants: [ $($variants)* $($kept)* $id $(($data))? , ]
            arms: [ $($arms)* ($($p)*) => { $crate::token::TokenKind::$id }; ]
        }
    };

    // Done, no #[name], exactly one #[token] -> that literal is the name.
    (@attrs
        scan: [ ] kept: [ $($kept:tt)* ] name: [ ] toks: [ $t:literal ]
        pats: [ $($p:tt)+ ]
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
            disp: [ $($disp)* $id : $t : literal , ]
            todo: [ $($todo)* ]
            variants: [ $($variants)* $($kept)* $id $(($data))? , ]
            arms: [ $($arms)* ($($p)*) => { $crate::token::TokenKind::$id }; ]
        }
    };

    // Done, but no #[pat(..)] -> `Tok![..]` would have no spelling for this token.
    (@attrs
        scan: [ ] kept: [ $($kept:tt)* ] name: [ $($name:tt)* ] toks: [ $($toks:tt)* ]
        pats: [ ]
        cur: [ $id:ident $(($data:tt))? ]
        outer: { $($outer:tt)* }
    ) => {
        compile_error!(concat!(
            "token variant `", stringify!($id),
            "` needs a #[pat(..)] giving its spelling inside `Tok![..]`"
        ));
    };

    // Done, but ambiguous: zero or 2+ #[token]s and no #[name].
    (@attrs
        scan: [ ] kept: [ $($kept:tt)* ] name: [ ] toks: [ $($toks:tt)* ]
        pats: [ $($p:tt)* ]
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

    // #[pat(..)] -- the spelling this token answers to inside `Tok![..]`.
    (@attrs
        scan: [ #[pat($($p:tt)*)] $($rest:tt)* ]
        kept: [ $($kept:tt)* ] name: [ $($name:tt)* ] toks: [ $($toks:tt)* ]
        pats: [ $($old:tt)* ]
        $($tail:tt)*
    ) => {
        tokens! {
            @attrs
            scan: [ $($rest)* ]
            kept: [ $($kept)* ]
            name: [ $($name)* ]
            toks: [ $($toks)* ]
            pats: [ $($p)* ]
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

    // Render a token name the way a diagnostic wants to read it: concrete spellings get
    // backticks (`` `,` ``), prose names are already written to slot into a sentence
    // ("an identifier").
    (@disp_str literal $l:literal) => { concat!("`", $l, "`") };
    (@disp_str prose   $l:literal) => { $l };

    // everything is accumulated, emit.
    (@build
        meta: [ $($meta:tt)* ]
        ids:  [ $( $id:ident $(($data:tt))? ),* $(,)? ]
        disp: [ $( $did:ident : $dlit:literal : $dkind:ident , )* ]
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

            /// Every kind, in discriminant order. Lets a bitset of kinds be turned back
            /// into the kinds it holds (see `etac_parse`'s `TokenSet`).
            pub const ALL: [TokenKind; Self::COUNT] = [ $(TokenKind::$id,)* ];
        }

        impl fmt::Display for TokenKind {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                // `f.write_str(<expr>)` rather than `write!(f, <macro call>)`:
                // write!'s first argument must be a literal format string.
                f.write_str(match self {
                    $( TokenKind::$did => tokens!(@disp_str $dkind $dlit), )*
                })
            }
        }

        /// Name a [`TokenKind`] by how it is written in Eta source: `Tok![,]`,
        /// `Tok![while]`, `Tok!['(']`, `Tok![ident]`.
        ///
        /// Generated from the token table above, so a new token becomes available here
        /// the moment it gets a `#[pat(..)]` -- there is no second list to update.
        /// Delimiters are spelled as char literals (`Tok!['(']`) because an unbalanced
        /// `(` cannot appear on its own inside a macro invocation.
        #[macro_export]
        macro_rules! Tok {
            $($arms)*
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
        #[derive(Debug, Clone, Copy, PartialEq, Logos)]
        #[logos(skip r"[ \t\n\f\r]+")]
        #[logos(skip(r"//[^\n]*", allow_greedy = true))]
        #[logos(extras = (u32, &'s mut EtacInterner))]
        #[logos(error(InternalLexerError, internal_error_unknown_token))]
    }

    #[name("end of file")]  #[pat(EOF)]     EOF,
    #[name("error")]        #[pat(Error)]   Error,

    // Keywords
    #[token("use")]     #[pat(use)]     KwUse,
    #[token("length")]  #[pat(length)]  KwLength,
    #[token("while")]   #[pat(while)]   KwWhile,
    #[token("if")]      #[pat(if)]      KwIf,
    #[token("else")]    #[pat(else)]    KwElse,
    #[token("return")]  #[pat(return)]  KwReturn,
    #[token("int")]     #[pat(int)]     KwInt,
    #[token("bool")]    #[pat(bool)]    KwBool,

    #[token(";")]       #[pat(;)]       SemiColon,
    #[token("_")]       #[pat(_)]       Discard,
    #[token(":")]       #[pat(:)]       Colon,
    #[token("=")]       #[pat(=)]       Assign,
    #[token(",")]       #[pat(,)]       Comma,

    #[token("(")]       #[pat('(')]     LParen,
    #[token(")")]       #[pat(')')]     RParen,
    #[token("[")]       #[pat('[')]     LBracket,
    #[token("]")]       #[pat(']')]     RBracket,
    #[token("{")]       #[pat('{')]     BlockOpen,
    #[token("}")]       #[pat('}')]     BlockClose,

    #[token("*")]       #[pat(*)]       Star,
    #[token("*>>")]     #[pat(*>>)]     HMul,
    #[token("/")]       #[pat(/)]       FSlash,
    #[token("%")]       #[pat(%)]       Percent,
    #[token("!")]       #[pat(!)]       Bang,
    #[token("-")]       #[pat(-)]       Minus,
    #[token("+")]       #[pat(+)]       Plus,

    #[token("==")]      #[pat(==)]      EqEq,
    #[token("!=")]      #[pat(!=)]      Neq,
    #[token(">")]       #[pat(>)]       Gt,
    #[token(">=")]      #[pat(>=)]      Ge,
    #[token("<")]       #[pat(<)]       Lt,
    #[token("<=")]      #[pat(<=)]      Le,

    #[token("&")]       #[pat(&)]       Land,
    #[token("|")]       #[pat(|)]       Lor,

    #[token("true", |_| true)]
    #[token("false", |_| false)]
    #[name("a boolean literal")]
    #[pat(bool_lit)]
    BoolLiteral(bool),

    #[regex(r"'([^'\\]|\\(.|x\{[0-9A-Fa-f]{1,6}\}))*'", crate::strings::parse_char)]
    #[name("a character literal")]
    #[pat(char_lit)]
    CharLiteral(char),

    #[regex(r#""([^"\\]|\\(.|x\{[0-9A-Fa-f]{1,6}\}))*""#, |lex| {
        crate::strings::parse_str(lex).map(|s| lex.extras.1.get_or_intern(&s))
    })]
    #[name("a string literal")]
    #[pat(str_lit)]
    StrLiteral(Symbol),

    #[regex(r"[1-9][0-9]*|0", crate::parse_int)]
    #[name("an integer literal")]
    #[pat(int_lit)]
    IntLiteral(u64),

    #[regex(r"[a-zA-Z][a-zA-Z0-9_']*", |lex| lex.extras.1.get_or_intern(lex.slice()))]
    #[name("an identifier")]
    #[pat(ident)]
    Identifier(Symbol),
}
