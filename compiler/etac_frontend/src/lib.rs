//! Lexing and parsing whole files.
//!
//! [`Frontend`] turns a file in the source map into a tree: it lexes, parses, and reports the diagnostics of both. It
//! is what the driver uses for the files named on the command line, and what lowering uses for the interfaces it
//! finds through `use`, so every file goes through the same code whoever asked for it.
//!
//! The lexer and the parser know nothing about debugging. Instead the frontend carries a [`Hooks`] object and calls
//! it after each phase. `()` does nothing; [`Debugger`] writes the `--dlex` and `--dparse` dumps. Anything a hook
//! needs beyond what a phase produced, such as where to write, belongs to the hook object and not to the lexer or
//! parser.
//!
//! A `Frontend` works through `&self` and is `Send + Sync`, like the source map, interner and diagnostic context it
//! borrows, so threads that parse different files can share one. An interface two of them ask for at the same time is
//! still parsed once, and a hook is never run by two threads at once.
//!
//! ```ignore
//! fn lower<H: Hooks>(frontend: &Frontend<'_, H>, resolver: &mut Resolver, ...) {
//!     // for each `use` in a program:
//!     let id = resolver.resolve_use(frontend.sm, frontend.dcx, from, name, span)?;
//!     let interface = frontend.interface(id).tree; // lexed, parsed, dumped and diagnosed once
//! }
//! ```

use std::collections::HashMap;
use std::io::Write;
use std::sync::{Arc, Mutex, OnceLock, PoisonError};

use etac_ast::ast::{Interface, Program};
use etac_errors::{
    dcx::{DiagBuilder, DiagCtx},
    guarantee::ErrorGuaranteed,
};
use etac_intern::EtacInterner;
use etac_lexer::{EtaLexer, Tokens};
use etac_source::{FileId, InterfaceId, SourceId, SourceMap};

use etac_parse::{
    Parse,
    result::{ParseResult, Parsed},
};

/// Code that runs alongside the [`Frontend`]: each hook is called once per file, right after its phase.
///
/// Every hook does nothing by default. They see the result of the phase before its diagnostics are reported, and
/// can't change it.
pub trait Hooks {
    /// `file` has been lexed into `tokens`.
    fn lexed(&mut self, symbols: &EtacInterner, file: FileId, tokens: &Tokens) {
        let _ = (symbols, file, tokens);
    }

    /// `file` has been parsed.
    fn parsed<T: Parse>(&mut self, symbols: &EtacInterner, file: FileId, parse: &Parsed<T>) {
        let _ = (symbols, file, parse);
    }
}

/// No hooks.
impl Hooks for () {}

/// [`Hooks`] that write what the frontend produced: the tokens of each file it lexes, the tree of each file it
/// parses, or both.
///
/// Both dumps start switched off, so a `Debugger` can always be installed and configured from flags.
pub struct Debugger<'sm, W> {
    out: W,
    sm: &'sm SourceMap,
    tokens: bool,
    trees: bool,
}

impl<'sm, W: Write> Debugger<'sm, W> {
    /// A debugger that writes to `out`, with both dumps off.
    pub fn new(out: W, sm: &'sm SourceMap) -> Self {
        Debugger {
            out,
            sm,
            tokens: false,
            trees: false,
        }
    }

    /// Whether to dump the tokens of each lexed file, with [`etac_lexer::debug::write_tokens`].
    #[must_use]
    pub fn tokens(mut self, on: bool) -> Self {
        self.tokens = on;
        self
    }

    /// Whether to dump the tree of each parsed file, with [`debug::write_parse`](etac_parse::debug::write_parse).
    #[must_use]
    pub fn trees(mut self, on: bool) -> Self {
        self.trees = on;
        self
    }
}

impl<W: Write> Hooks for Debugger<'_, W> {
    fn lexed(&mut self, symbols: &EtacInterner, _file: FileId, tokens: &Tokens) {
        if self.tokens {
            etac_lexer::debug::write_tokens(&mut self.out, self.sm, symbols, tokens);
        }
    }

    fn parsed<T: Parse>(&mut self, symbols: &EtacInterner, file: FileId, parse: &Parsed<T>) {
        if self.trees {
            etac_parse::debug::write_parse(&mut self.out, self.sm, symbols, file, parse);
        }
    }
}

/// A file the [`Frontend`] has lexed and parsed.
///
/// There is always a tree, because the lexer and the parser both recover from the errors they report. `errors` says
/// whether they had to, for this file alone: a caller that must not build on a broken tree checks it here rather
/// than asking the [`DiagCtx`], which counts the errors of every file.
#[derive(Debug, Clone)]
pub struct ParsedFile<T> {
    pub tree: T,
    /// Proof that lexing or parsing this file reported an error, if either did.
    pub errors: Option<ErrorGuaranteed>,
}

impl<T> ParsedFile<T> {
    /// Whether the file lexed and parsed without an error.
    pub fn is_clean(&self) -> bool {
        self.errors.is_none()
    }
}

/// Lexes and parses files out of a [`SourceMap`], reporting their diagnostics to a [`DiagCtx`].
///
/// See the [crate docs](crate).
pub struct Frontend<'a, H = ()> {
    pub sm: &'a SourceMap,
    pub interner: &'a EtacInterner,
    pub dcx: &'a DiagCtx,
    /// Locked for each call, so that a hook has `&mut self` and what it writes for one file stays in one piece.
    hooks: Mutex<H>,
    /// Every interface asked for so far, so that a file is only ever parsed, and its errors reported, once. The cell
    /// is what makes that hold across threads: whoever gets there first parses, and the rest wait for its tree.
    interfaces: Mutex<HashMap<InterfaceId, InterfaceCell>>,
}

type InterfaceCell = Arc<OnceLock<ParsedFile<Arc<Interface>>>>;

const fn assert_send_sync<T: Send + Sync>() {}
const _: () = assert_send_sync::<Frontend<'static, ()>>();
const _: () = assert_send_sync::<Frontend<'static, Debugger<'static, std::io::Stdout>>>();

impl<'a, H: Hooks> Frontend<'a, H> {
    pub fn new(sm: &'a SourceMap, interner: &'a EtacInterner, dcx: &'a DiagCtx, hooks: H) -> Self {
        Frontend {
            sm,
            interner,
            dcx,
            hooks: Mutex::new(hooks),
            interfaces: Mutex::default(),
        }
    }

    /// Lex and parse an `.eta` file.
    pub fn program(&self, file: SourceId) -> ParsedFile<Program> {
        self.parse_file(file.file())
    }

    /// Lex and parse an `.eti` file, or return the tree from the first time it was asked for.
    ///
    /// Each interface is parsed once, however many programs `use` it, and they all share the one tree.
    pub fn interface(&self, file: InterfaceId) -> ParsedFile<Arc<Interface>> {
        // Only hold the table for the lookup: parsing happens with it unlocked, inside the file's own cell.
        let cell = {
            let mut interfaces = self.interfaces.lock().unwrap_or_else(PoisonError::into_inner);
            interfaces.entry(file).or_default().clone()
        };
        cell.get_or_init(|| {
            let ParsedFile { tree, errors } = self.parse_file(file.file());
            ParsedFile {
                tree: Arc::new(tree),
                errors,
            }
        })
        .clone()
    }

    fn hooks(&self) -> std::sync::MutexGuard<'_, H> {
        self.hooks.lock().unwrap_or_else(PoisonError::into_inner)
    }

    fn parse_file<T: Parse>(&self, file: FileId) -> ParsedFile<T> {
        let source = self.sm.get_source(file);
        let lexed = EtaLexer::new(self.sm.base_offset(file), self.interner, self.dcx).lex(&source.source);
        self.hooks().lexed(self.interner, file, &lexed.tokens);
        let lex_errors = lexed.diags.into_iter().map(DiagBuilder::finish).last();

        let ParseResult { parse, diags } = T::parse(&lexed.tokens, self.dcx);
        self.hooks().parsed(self.interner, file, &parse);
        let parse_errors = diags.into_iter().map(DiagBuilder::finish).last();

        ParsedFile {
            tree: parse.into_tree(),
            errors: lex_errors.or(parse_errors),
        }
    }
}
