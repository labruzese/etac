//! Parser
//!
//! Everything in here is grammar-independent.

use std::fmt;

use etac_ast::ast::Error;
use etac_errors::dcx::{DiagBuilder, DiagCtx};
use etac_errors::guarantee::ErrorGuaranteed;
use etac_lexer::Tok;
use etac_lexer::Tokens;
use etac_lexer::token::{Token, TokenKind};
use etac_source::Span;

use crate::token_set::TokenSet;

/// Different kinds of things the parser might expect. Named constructs provide better error
/// messages but if there's only a few tokens that you could have expected you can also expect a
/// certain token or set.
#[derive(Clone, Copy)]
pub enum Expected {
    Tok(TokenKind),
    Set(TokenSet),
    Expr,
    Type,
    Stmt,
    Definition,
    InterfaceItem,
    Ident,
    Param,
    GlobalInit,
}

impl fmt::Display for Expected {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expected::Tok(k) => write!(f, "{k}"),
            Expected::Set(s) => write!(f, "{s}"),
            Expected::Expr => f.write_str("an expression"),
            Expected::Type => f.write_str("a type"),
            Expected::Stmt => f.write_str("a statement"),
            Expected::Definition => f.write_str("a method or global definition"),
            Expected::InterfaceItem => f.write_str("a method declaration"),
            Expected::Ident => f.write_str("an identifier"),
            Expected::Param => f.write_str("a parameter"),
            Expected::GlobalInit => f.write_str("a literal"),
        }
    }
}

/// The shape of a separated list
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum List {
    /// `elem,*`: may be empty.
    Any,
    /// `elem,+`: an empty list is an error.
    NonEmpty,
    /// `elem,*` with an optional trailing separator.
    Trailing,
}

// ---- span ----

/// The start of a node, opened by [`Parser::start`] and closed by [`Parser::finish`].
#[derive(Clone, Copy)]
pub struct Marker(u32);

// ---- parser ----

pub struct Parser<'t, 'dcx> {
    /// kinds only, useful for fast parsing
    kinds: &'t [TokenKind],
    /// prefer kinds, only use data when you need the actual token details / span.
    data: &'t [(Token, Span)],
    /// current location of the parser
    pos: usize,
    /// End of the last consumed token: where a node's span should stop.
    /// This is so we don't swallow the trailing whitespace and comments.
    prev_end: u32,
    /// dcx that this parser reports to, as it goes
    dcx: &'dcx DiagCtx,
    /// The last error this parser reported, and the token position it was at: the position is for cascade
    /// suppression, and the proof is what makes the error node.
    last_err: Option<(usize, ErrorGuaranteed)>,
    /// current tokens we can recover on
    recovery: TokenSet,
}

impl<'t, 'dcx> Parser<'t, 'dcx> {
    pub fn new(tokens: &'t Tokens, dcx: &'dcx DiagCtx) -> Self {
        debug_assert!(
            tokens.kinds.last() == Some(&Tok![EOF]),
            "the token stream must be EOF-terminated"
        );
        Parser {
            kinds: &tokens.kinds,
            data: &tokens.data,
            pos: 0,
            prev_end: tokens.data.first().map_or(0, |(_, s)| s.lo),
            dcx,
            last_err: None,
            recovery: TokenSet::EMPTY,
        }
    }

    /// Proof that this parser reported an error, or stood an error node in for one the lexer reported, if it did.
    pub fn finish_parse(self) -> Option<ErrorGuaranteed> {
        self.last_err.map(|(_, reported)| reported)
    }

    // ---- cursor ----

    #[inline]
    pub fn cur(&self) -> TokenKind {
        self.kinds[self.pos]
    }

    #[inline]
    pub fn nth(&self, n: usize) -> TokenKind {
        // The stream is EOF-terminated, so reads past the end are EOF
        self.kinds.get(self.pos + n).copied().unwrap_or(Tok![EOF])
    }

    #[inline]
    pub fn at(&self, k: TokenKind) -> bool {
        self.cur() == k
    }

    /// lookahead N and see if it matches these token sets
    #[inline]
    pub fn matches<const N: usize>(&self, tsl: [TokenSet; N]) -> bool {
        let kinds = &self.kinds[self.pos..];
        if N > kinds.len() {
            return false;
        }
        tsl.iter().zip(kinds).all(|(ts, k)| ts.contains(*k))
    }

    #[inline]
    pub fn nth_at(&self, n: usize, k: TokenKind) -> bool {
        self.nth(n) == k
    }

    #[inline]
    pub fn at_any(&self, ts: TokenSet) -> bool {
        ts.contains(self.cur())
    }

    #[inline]
    pub fn at_eof(&self) -> bool {
        self.at(Tok![EOF])
    }

    #[inline]
    pub fn pos(&self) -> usize {
        self.pos
    }

    #[inline]
    pub fn cur_span(&self) -> Span {
        self.data[self.pos].1
    }

    /// Zero-width span just after the last consumed token, i.e. where a missing token
    /// *should* have been.
    #[inline]
    pub fn gap_span(&self) -> Span {
        Span::new(self.prev_end, self.prev_end)
    }

    #[inline]
    pub fn bump_any(&mut self) -> (Token, Span) {
        let entry = self.data[self.pos];
        if !self.at_eof() {
            self.pos += 1;
            self.prev_end = entry.1.hi;
        }
        entry
    }

    #[inline]
    pub fn bump(&mut self, k: TokenKind) -> Span {
        debug_assert!(self.at(k), "bump({k}) but the cursor is at {}", self.cur());
        self.bump_any().1
    }

    #[inline]
    pub fn eat(&mut self, k: TokenKind) -> bool {
        if self.at(k) {
            self.bump(k);
            true
        } else {
            false
        }
    }

    // ---- spans ----

    #[inline]
    pub fn start(&self) -> Marker {
        Marker(self.cur_span().lo)
    }

    #[inline]
    pub fn finish(&self, m: Marker) -> Span {
        // A production that consumed nothing would otherwise produce an inverted span.
        Span::new(m.0, self.prev_end.max(m.0))
    }

    // ---- recovery anchors ----

    /// Add `extra` to the anchor set for the duration of a nested construct. Returns the
    /// previous set, to be handed back to [`leave_recovery`](Self::leave_recovery).
    #[inline]
    #[must_use]
    pub fn enter_recovery(&mut self, extra: TokenSet) -> TokenSet {
        let saved = self.recovery;
        self.recovery = saved.union(extra);
        saved
    }

    #[inline]
    pub fn leave_recovery(&mut self, saved: TokenSet) {
        self.recovery = saved;
    }

    // ---- diagnostics ----

    /// One diagnostic per token position. The second complaint about the same token is
    /// always a consequence of the first, so it is not made, and the first one's proof serves for both.
    fn suppressed(&self) -> Option<ErrorGuaranteed> {
        match self.last_err {
            Some((pos, reported)) if pos == self.pos => Some(reported),
            _ => None,
        }
    }

    /// Remember that an error was reported at the current token.
    fn reported_here(&mut self, reported: ErrorGuaranteed) -> ErrorGuaranteed {
        self.last_err = Some((self.pos, reported));
        reported
    }

    /// Report a diagnostic at the current token.
    fn push(&mut self, d: DiagBuilder<'dcx>) -> ErrorGuaranteed {
        let reported = d.finish();
        self.reported_here(reported)
    }

    /// Report `what` at the current token, without moving.
    ///
    /// The proof returned is not always of a new diagnostic: a second complaint about one token is
    /// [suppressed](Self::suppressed), and an `Error` token was reported by the lexer.
    pub fn err_expected(&mut self, what: Expected) -> ErrorGuaranteed {
        if let Some(reported) = self.suppressed() {
            return reported;
        }
        // The lexer already complained about this one;
        // Mark the position so the next production stays quiet too.
        if let Token::Error(reported) = self.data[self.pos].0 {
            return self.reported_here(reported);
        }
        let span = if self.at_eof() {
            self.gap_span()
        } else {
            self.cur_span()
        };
        let d = self
            .dcx
            .new_err(span, format!("expected {}, found {}", what, self.cur()))
            .with_message(format!("expected {}", what));
        self.push(d)
    }

    /// one-off messages that don't fit [`Expected`].
    /// the `Diag` is built by the caller so it can carry secondary labels.
    pub fn report(&mut self, d: DiagBuilder<'dcx>) -> ErrorGuaranteed {
        // not suppressed: these are semantic complaints about something the parser understood,
        // not an unknown token cascade.
        self.push(d)
    }

    /// build a diagnostic without reporting it, so a caller can decorate it.
    pub fn dcx(&self) -> &'dcx DiagCtx {
        self.dcx
    }

    // ---- skipping ----

    /// skip forward until the cursor is on something in `stop`, on a closer belonging to
    /// an enclosing construct, or at EOF.
    ///
    /// nesting is tracked, so a `,` or `)` inside a nested `(..)`, `[..]` or `{..}`
    /// doesn't end the search.
    pub fn skip_until(&mut self, stop: TokenSet) -> Span {
        let m = self.start();
        let mut depth = 0u32;
        loop {
            let k = self.cur();
            if k == Tok![EOF] {
                break;
            }
            if depth == 0 && stop.contains(k) {
                break;
            }
            match k {
                Tok!['('] | Tok!['['] | Tok!['{'] => depth += 1,
                Tok![')'] | Tok![']'] | Tok!['}'] => {
                    // closes something further out; leave it for that level to find.
                    if depth == 0 {
                        break;
                    }
                    depth -= 1;
                }
                _ => {}
            }
            self.bump_any();
        }
        self.finish(m)
    }

    /// report `what` at the current token, then skip to `stop` (plus the enclosing
    /// anchors). returns the error node standing in for everything that was thrown away.
    pub fn err_and_recover(&mut self, what: Expected, stop: TokenSet) -> Error {
        let at = self.cur_span();
        let reported = self.err_expected(what);
        let skipped = self.skip_until(stop.union(self.recovery));
        // Nothing was skipped: the offending token is still ahead of us, but it is what
        // the error node stands for.
        let span = if skipped.lo == skipped.hi { at } else { skipped };
        Error { span, reported }
    }

    // ---- expecting ----

    /// consume `k`, reporting if it isn't there.
    ///
    /// heuristic: applies a one-token deletion: `f(a,, b)` reports the stray `,` once and carries
    /// on with `b`, rather than treating the rest of the argument list as garbage.
    ///
    /// `Err` means `k` was not where it belongs, which has been reported. That is so even when the deletion then
    /// found it.
    pub fn expect(&mut self, k: TokenKind) -> Result<(), ErrorGuaranteed> {
        if self.eat(k) {
            return Ok(());
        }
        let reported = self.err_expected(Expected::Tok(k));
        if self.nth(1) == k && !self.at_any(self.recovery) {
            self.bump_any();
            self.bump(k);
        }
        Err(reported)
    }

    // ---- separated lists ----

    /// parse `elem (sep elem)*`, stopping at anything in `close`.
    ///
    /// Returns the elements, and proof of the last error the list itself reported, if it reported any: a missing
    /// element or separator. What `elem` reports is its own business.
    pub fn separated<T>(
        &mut self,
        sep: TokenKind,
        close: TokenSet,
        elem_first: TokenSet,
        what: Expected,
        shape: List,
        mut elem: impl FnMut(&mut Self) -> Option<T>,
    ) -> (Vec<T>, Option<ErrorGuaranteed>) {
        let mut out = Vec::new();
        let mut reported = None;
        let stop = close.union(TokenSet::of(sep));

        // an empty list is only an error when the grammar wanted `elem,+`.
        if self.at_any(close) && shape != List::NonEmpty {
            return (out, reported);
        }

        loop {
            let before = self.pos;
            if self.at_any(elem_first) {
                if let Some(e) = elem(self) {
                    out.push(e);
                }
            } else {
                reported = Some(self.err_and_recover(what, stop).reported);
            }

            if self.pos == before && !self.at_any(stop) && !self.at_eof() {
                self.bump_any();
            }

            if self.eat(sep) {
                // a separator commits to another element, so tokens that are in *both*
                // `close` and `elem_first` (identifier, `(`) mean the element
                // here. Without a separator the same tokens mean the list ended
                // and something new began, which is what makes `;` optional.
                if !self.at_any(elem_first) && (self.at_any(close) || self.at_eof()) {
                    if shape != List::Trailing {
                        reported = Some(self.err_expected(what));
                    }
                    break;
                }
                continue;
            }
            if self.at_any(close) || self.at_eof() {
                break;
            }

            // another element with no separator between: `f(1 2)`. Report the missing
            // sep and keep the list together rather than ending it here.
            if self.at_any(elem_first) {
                reported = Some(self.err_expected(Expected::Tok(sep)));
                continue;
            }

            reported = Some(self.err_and_recover(Expected::Tok(sep), stop).reported);
            if !self.eat(sep) || self.at_any(close) || self.at_eof() {
                break;
            }
        }
        (out, reported)
    }

    /// consume the closing half of a delimited construct.
    ///
    /// junk before the closer is skipped; a missing closer is reported against
    /// the opener.
    pub fn expect_close(&mut self, close: TokenKind, open: Span, what: &str) -> bool {
        if self.eat(close) {
            return true;
        }

        // look for the closer past whatever junk is in the way, but stay silent while
        // doing it: which of the two messages below is the useful one depends on whether
        // the closer turns up.
        let junk = self.skip_until(TokenSet::of(close).union(self.recovery));

        if self.at(close) {
            if junk.hi > junk.lo && self.suppressed().is_none() {
                let d = self
                    .dcx
                    .new_err(junk, format!("unexpected tokens in this {}", what))
                    .with_message(format!("expected {} here", close));
                let _ = self.push(d);
            }
            self.bump(close);
            return true;
        }

        // unclosed. report it against the opener.
        if self.suppressed().is_none() {
            let d = self
                .dcx
                .new_err(open, format!("unclosed {}", what))
                .with_message(format!("this {} is never closed", what))
                .with_note(format!("expected {} to close it", close));
            let _ = self.push(d);
        }
        false
    }
}
