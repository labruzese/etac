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
//! ```ignore
//! fn lower<H: Hooks>(frontend: &mut Frontend<'_, H>, resolver: &mut Resolver<'_>, ...) {
//!     // for each `use` in a program:
//!     let id = resolver.resolve_use(frontend.sm, frontend.dcx, from, name, span)?;
//!     let interface = frontend.interface(id)?; // lexed, parsed, dumped and diagnosed once
//! }
//! ```

use std::collections::HashMap;
use std::io::Write;
use std::sync::Arc;

use etac_ast::ast::{Interface, Program};
use etac_errors::{
    dcx::{DiagBuilder, DiagCtx},
    guarentee::ErrorGuaranteed,
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
    fn lexed(&mut self, symbols: &EtacInterner, file: FileId<'_>, tokens: &Tokens) {
        let _ = (symbols, file, tokens);
    }

    /// `file` has been parsed.
    fn parsed<T: Parse>(&mut self, symbols: &EtacInterner, file: FileId<'_>, parse: &Parsed<T>) {
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

    /// Whether to dump the tree of each parsed file, with [`debug::write_parse`](crate::debug::write_parse).
    #[must_use]
    pub fn trees(mut self, on: bool) -> Self {
        self.trees = on;
        self
    }
}

impl<W: Write> Hooks for Debugger<'_, W> {
    fn lexed(&mut self, symbols: &EtacInterner, _file: FileId<'_>, tokens: &Tokens) {
        if self.tokens {
            etac_lexer::debug::write_tokens(&mut self.out, self.sm, symbols, tokens);
        }
    }

    fn parsed<T: Parse>(&mut self, symbols: &EtacInterner, file: FileId<'_>, parse: &Parsed<T>) {
        if self.trees {
            etac_parse::debug::write_parse(&mut self.out, self.sm, symbols, file, parse);
        }
    }
}

/// Lexes and parses files out of a [`SourceMap`], reporting their diagnostics to a [`DiagCtx`].
///
/// See the [module docs](crate::frontend).
pub struct Frontend<'a, H = ()> {
    pub sm: &'a SourceMap,
    pub interner: &'a mut EtacInterner,
    pub dcx: &'a DiagCtx,
    pub hooks: H,
    /// Every interface parsed so far, so that a file is only ever parsed, and its errors reported, once.
    interfaces: HashMap<InterfaceId<'a>, Result<Arc<Interface>, ErrorGuaranteed>>,
}

impl<'a, H: Hooks> Frontend<'a, H> {
    pub fn new(sm: &'a SourceMap, interner: &'a mut EtacInterner, dcx: &'a DiagCtx, hooks: H) -> Self {
        Frontend {
            sm,
            interner,
            dcx,
            hooks,
            interfaces: HashMap::new(),
        }
    }

    /// Lex and parse an `.eta` file.
    ///
    /// The parser recovers from errors, so there is a tree even when it reported some. `Err` means there is none.
    pub fn program(&mut self, file: SourceId<'a>) -> Result<Program, ErrorGuaranteed> {
        self.parse_file(file)
    }

    /// Lex and parse an `.eti` file, or return the tree from the first time it was asked for.
    ///
    /// Each interface is parsed once, however many programs `use` it, and they all share the one tree.
    pub fn interface(&mut self, file: InterfaceId<'a>) -> Result<Arc<Interface>, ErrorGuaranteed> {
        if let Some(parsed) = self.interfaces.get(&file) {
            return parsed.clone();
        }
        let parsed = self.parse_file(file).map(Arc::new);
        self.interfaces.insert(file, parsed.clone());
        parsed
    }

    fn parse_file<T: Parse>(&mut self, file: FileId<'a>) -> Result<T, ErrorGuaranteed> {
        let source = self.sm.get_source(file);
        let lexed = EtaLexer::new(self.sm.base_offset(file), self.interner, self.dcx).lex(&source.source);
        self.hooks.lexed(self.interner, file, &lexed.tokens);
        let lex_guarantee = lexed.diags.into_iter().map(DiagBuilder::finish).last();

        let ParseResult { parse, diags } = T::parse(&lexed.tokens, self.dcx);
        self.hooks.parsed(self.interner, file, &parse);
        let parse_guarantee = diags.into_iter().map(DiagBuilder::finish).last();

        match parse {
            Parsed::Ok(tree) | Parsed::Recovered(tree) => Ok(tree),
            Parsed::Failed => Err(parse_guarantee
                .or(lex_guarantee)
                .expect("a failed parse must have reported at least one diagnostic")),
        }
    }
}

