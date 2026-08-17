use etac_errors::dcx::DiagCtx;
use etac_lexer::token::Token;
use etac_lexer::token::TokenKind;
use etac_lexer::Tok;
use etac_source::Span;

macro_rules! Tok {
    (EOF) => { etac_lexer::token::TokenKind::EOF };
    (Error) => { etac_lexer::token::TokenKind::Error };

    // Keywords
    (use) => { etac_lexer::token::TokenKind::KwUse };
    (length) => { etac_lexer::token::TokenKind::KwLength };
    (while) => { etac_lexer::token::TokenKind::KwWhile };
    (if) => { etac_lexer::token::TokenKind::KwIf };
    (else) => { etac_lexer::token::TokenKind::KwElse };
    (return) => { etac_lexer::token::TokenKind::KwReturn };
    (int) => { etac_lexer::token::TokenKind::KwInt };
    (bool) => { etac_lexer::token::TokenKind::KwBool };

    (;) => { etac_lexer::token::TokenKind::SemiColon };
    (_) => { etac_lexer::token::TokenKind::Discard };
    (:) => { etac_lexer::token::TokenKind::Colon };
    (=) => { etac_lexer::token::TokenKind::Assign };
    (,) => { etac_lexer::token::TokenKind::Comma };

    ('(') => { etac_lexer::token::TokenKind::LParen };
    (')') => { etac_lexer::token::TokenKind::RParen };
    ('[') => { etac_lexer::token::TokenKind::LBracket };
    (']') => { etac_lexer::token::TokenKind::RBracket };
    ('{') => { etac_lexer::token::TokenKind::BlockOpen };
    ('}') => { etac_lexer::token::TokenKind::BlockClose };

    (*) => { etac_lexer::token::TokenKind::Star };
    (*>>) => { etac_lexer::token::TokenKind::HMul };
    (/) => { etac_lexer::token::TokenKind::FSlash };
    (%) => { etac_lexer::token::TokenKind::Percent };
    (!) => { etac_lexer::token::TokenKind::Bang };
    (-) => { etac_lexer::token::TokenKind::Minus };
    (+) => { etac_lexer::token::TokenKind::Plus };

    (==) => { etac_lexer::token::TokenKind::EqEq };
    (!=) => { etac_lexer::token::TokenKind::Neq };
    (>) => { etac_lexer::token::TokenKind::Gt };
    (>=) => { etac_lexer::token::TokenKind::Ge };
    (<) => { etac_lexer::token::TokenKind::Lt };
    (<=) => { etac_lexer::token::TokenKind::Le };

    (&) => { etac_lexer::token::TokenKind::Land};
    (|) => { etac_lexer::token::TokenKind::Lor };
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TokenSet(u64);
impl TokenSet {
    const _TOKEN_COUNT_IS_SMALL_ENOUGH: () = assert!(TokenKind::COUNT <= 64);
    pub const EMPTY: Self = Self(0);

    pub const fn new(ks: &[TokenKind]) -> Self {
        let (mut b, mut i) = (0u64, 0);
        while i < ks.len() { 
            b |= 1 << (ks[i] as u8); 
            i += 1; 
        }
        Self(b)
    }

    pub const fn union(self, o: Self) -> Self { 
        Self(self.0 | o.0) 
    }

    #[inline] 
    pub const fn contains(self, k: TokenKind) -> bool { 
        self.0 & (1 << (k as u8)) != 0 
    }
}

macro_rules! ts {
    (@acc [$($out:expr),*])                         => { const { TokenSet::new(&[$($out),*]) } };
    (@acc [$($out:expr),*] $l:tt , $($rest:tt)*)    => { ts!(@acc [$($out,)* Tok![$l]] $($rest)*) };
    (@acc [$($out:expr),*] $l:tt)                   => { ts!(@acc [$($out,)* Tok![$l]])           };
    (@acc [$($out:expr),*] $p:path , $($rest:tt)*)  => { ts!(@acc [$($out,)* $p      ] $($rest)*) };
    (@acc [$($out:expr),*] $p:path)                 => { ts!(@acc [$($out,)* $p      ])           };
    ($($tt:tt)*) => { ts!(@acc [] $($tt)*) };
}

const STMT_FIRST: TokenSet = ts![if, while, return, '{', '(', TokenKind::Identifier, _];
const TYPE_FIRST: TokenSet = ts![int, bool];
const EXPR_FIRST: TokenSet = ts![TokenKind::Identifier, length, '(', '{', -, !].union(LIT_FIRST);
const ITEM_FIRST: TokenSet = ts![TokenKind::Identifier, use];
const LIT_FIRST: TokenSet  = ts![TokenKind::IntLiteral, TokenKind::CharLiteral, TokenKind::StrLiteral, TokenKind::BoolLiteral, '[' /* arr lit*/];

#[derive(Clone, Copy)] pub struct Marker(u32);

pub struct Parser<'t, 'dcx> {
    kinds: &'t [TokenKind],
    data:  &'t [(Span, Token)],
    pos: u32,
    prev_end: u32,          // == LALRPOP's @R
    dcx: &'dcx DiagCtx,
    recovery: TokenSet,     // current anchor set
    last_err_pos: u32,      // cascade suppression
    depth: u16,             // stack-overflow guard
}

impl Parser {
    #[inline] fn cur(&self) -> TokenKind { 
        self.kinds[self.pos] 
    }
    #[inline] fn nth(&self, n: u32) -> TokenKind { 
        self.kinds[n] 
    }
    #[inline] fn at(&self, k: TokenKind) -> bool { 
        self.cur() == k 
    }
    #[inline] fn at_any(&self, ts: TokenSet) -> bool { 
        ts.contains(self.cur()) 
    }
    #[inline] fn nth_at(&self, n: u32, k: TokenKind) -> bool { 
        k == self.nth(n) 
    }
    #[inline] fn bump(&mut self, k: TokenKind) { 
        debug_assert!(self.at(k)); 
        self.pos += 1 
    }
    #[inline] fn bump_any(&mut self) -> (Span, Token) { 
        self.pos += 1; 
        self.data[self.pos - 1] 
    }
    #[inline] fn eat(&mut self, k: TokenKind) -> bool { 
        if self.at(k) { 
            self.bump(k); 
            true 
        } else { 
            false 
        }
    }
    #[inline] fn expect(&mut self, k: TokenKind, what: Expected) -> bool { 
        
    }
    fn start(&self) -> Marker { 
        Marker(self.cur_span().start()) 
    }
    fn span(&self, m: Marker) -> Span { 
        Span::new(m.0, self.prev_end) 
    }
}
