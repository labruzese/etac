//! The parser must terminate and must not panic, on any input at all.
//!
//! The generator is a deterministic xorshift, so a failure here reproduces exactly.

use etac_errors::dcx::{Diag, DiagCtx};
use etac_errors::emitter::EmitConfig;
use etac_intern::EtacInterner;
use etac_parse::{Parse, result::ParseResult};
use etac_source::SourceMap;

/// Every token spelling in the language, plus a few shapes that only matter to recovery.
const ATOMS: &[&str] = &[
    "use", "length", "while", "if", "else", "return", "int", "bool", ";", "_", ":", "=", ",", "(",
    ")", "[", "]", "{", "}", "*", "*>>", "/", "%", "!", "-", "+", "==", "!=", ">", ">=", "<", "<=",
    "&", "|", "true", "false", "'a'", "\"s\"", "x", "f", "0", "12", "\n",
];

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        // xorshift64*
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

/// Parse `src` both ways and throw the diagnostics away. Panics, hangs and debug
/// assertions are the failure modes under test; the output is not.
fn fuzz_one(src: &str) {
    let emitter = etac_test::sink_emitter(EmitConfig::default().with_color(false));
    let dcx = DiagCtx::with_emitter(Box::new(emitter));
    let sm = SourceMap::default();
    let mut interner = EtacInterner::with_hasher(std::hash::BuildHasherDefault::default());

    let (id, text) = sm.alloc_source("fuzz.eta".to_owned(), src.to_owned());
    let lexed = etac_lexer::EtaLexer::new(sm.base_offset(id), &mut interner, &dcx).lex(text);
    for d in Vec::from(lexed.diags) {
        d.cancel();
    }

    let ParseResult { parse: _, diags } = etac_ast::ast::Program::parse(&lexed.tokens, &dcx);
    for d in diags {
        d.cancel();
    }
    let ParseResult { parse: _, diags } = etac_ast::ast::Interface::parse(&lexed.tokens, &dcx);
    for d in diags {
        d.cancel();
    }

    let _: Vec<Diag<'_>> = Vec::new();
    dcx.cancel();
}

#[test]
fn random_token_soup_terminates() {
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
    for _ in 0..2000 {
        let len = 1 + rng.below(60);
        let mut src = String::new();
        for _ in 0..len {
            src.push_str(ATOMS[rng.below(ATOMS.len())]);
            src.push(' ');
        }
        fuzz_one(&src);
    }
}

/// Every prefix of a real program. 
#[test]
fn every_prefix_of_a_real_program_terminates() {
    let src = include_str!("real_programs/insertionsort.eta");
    for end in 0..=src.len() {
        if src.is_char_boundary(end) {
            fuzz_one(&src[..end]);
        }
    }
}

/// Real programs with single tokens deleted.
#[test]
fn single_deletions_from_a_real_program_terminate() {
    let src = include_str!("real_programs/binsearch.eta");
    let chars: Vec<char> = src.chars().collect();
    for i in 0..chars.len() {
        let mut m: String = chars[..i].iter().collect();
        m.extend(chars[i + 1..].iter());
        fuzz_one(&m);
    }
}

/// Deep nesting must report, not overflow the stack.
#[test]
fn deep_nesting_is_capped() {
    for depth in [50usize, 200, 1000, 5000] {
        let src = format!("f() {{ x: int = {}1{} }}", "(".repeat(depth), ")".repeat(depth));
        fuzz_one(&src);
    }
    // Unbalanced, so recovery has to unwind it too.
    fuzz_one(&format!("f() {{ x: int = {}1 }}", "(".repeat(5000)));
    fuzz_one(&format!("f() {{ {} }}", "if true ".repeat(5000)));
    fuzz_one(&format!("f() {{ x: int = {}1 }}", "-".repeat(5000)));
}
