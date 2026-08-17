//! A bitset of [`TokenKind`]s

use std::fmt;

use etac_lexer::Tok;
use etac_lexer::token::TokenKind;

const _: () = assert!(TokenKind::COUNT <= 64, "TokenSet is a u64 bitset; add a second word");

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct TokenSet(u64);

impl TokenSet {
    pub const EMPTY: Self = Self(0);
    // pub const UNIVERSAL: Self = Self((1u64 << TokenKind::COUNT).wrapping_sub(1));

    pub const fn new(ks: &[TokenKind]) -> Self {
        let (mut b, mut i) = (0u64, 0);
        while i < ks.len() {
            b |= 1 << (ks[i] as u8);
            i += 1;
        }
        Self(b)
    }

    pub const fn of(k: TokenKind) -> Self {
        Self(1 << (k as u8))
    }

    pub const fn union(self, o: Self) -> Self {
        Self(self.0 | o.0)
    }

    #[inline]
    pub const fn contains(self, k: TokenKind) -> bool {
        self.0 & (1 << (k as u8)) != 0
    }

    pub fn iter(self) -> impl Iterator<Item = TokenKind> {
        TokenKind::ALL.into_iter().filter(move |k| self.contains(*k))
    }
}

/// renders as the tail of an expected message. 
impl fmt::Display for TokenSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // EOF is in a lot of follow sets and never helps a reader.
        let named: Vec<TokenKind> = self.iter().filter(|k| *k != Tok![EOF] && *k != Tok![Error]).collect();
        match named.as_slice() {
            [] => f.write_str("nothing"),
            [one] => write!(f, "{one}"),
            [a, b] => write!(f, "{a} or {b}"),
            [rest @ .., last] => {
                f.write_str("one of ")?;
                for k in rest {
                    write!(f, "{k}, ")?;
                }
                write!(f, "or {last}")
            }
        }
    }
}

/// build a [`TokenSet`] from `Tok![..]` spellings: `ts![if, while, '{', ident]`.
///
/// expands to an inline `const` block, so the bitset is folded at compile time even in
/// non-`const` position. 
///
/// spellings that are more than one Rust token (`*>>`) can't be captured as a `tt`;
/// write those as a `TokenKind` path instead.
macro_rules! ts {
    (@acc [$($out:expr),*])                         => { const { $crate::token_set::TokenSet::new(&[$($out),*]) } };
    (@acc [$($out:expr),*] $l:tt , $($rest:tt)*)    => { ts!(@acc [$($out,)* ::etac_lexer::Tok![$l]] $($rest)*) };
    (@acc [$($out:expr),*] $l:tt)                   => { ts!(@acc [$($out,)* ::etac_lexer::Tok![$l]])           };
    (@acc [$($out:expr),*] $p:path , $($rest:tt)*)  => { ts!(@acc [$($out,)* $p] $($rest)*) };
    (@acc [$($out:expr),*] $p:path)                 => { ts!(@acc [$($out,)* $p])           };
    ($($tt:tt)*)                                    => { ts!(@acc [] $($tt)*) };
}

pub(crate) use ts;
