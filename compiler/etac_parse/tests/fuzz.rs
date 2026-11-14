//! The parser must terminate and must not panic, on any input at all.
//!
//! The generator is a deterministic xorshift, so a failure here reproduces exactly.

use etac_errors::dcx::DiagCtx;
use etac_intern::EtacInterner;
use etac_parse::Parse;
use etac_source::SourceMap;

/// Every token spelling in the language, plus a few shapes that only matter to recovery.
const ATOMS: &[&str] = &[
    "use", "length", "while", "if", "else", "return", "int", "bool", ";", "_", ":", "=", ",", "(", ")", "[", "]", "{",
    "}", "*", "*>>", "/", "%", "!", "-", "+", "==", "!=", ">", ">=", "<", "<=", "&", "|", "true", "false", "'a'",
    "\"s\"", "x", "f", "0", "12", "\n",
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
    errors_in(src);
}

/// [`fuzz_one`], returning how many errors lexing and parsing `src` as a program reported.
fn errors_in(src: &str) -> usize {
    let emitter = etac_errors::emitter::IoEmitter::sink();
    let dcx = DiagCtx::new(emitter);
    let sm = SourceMap::default();
    let interner = EtacInterner::with_hasher(std::hash::BuildHasherDefault::default());

    let (id, text) = sm.alloc_source("fuzz.eta".to_owned(), src.to_owned());
    let tokens = etac_lexer::EtaLexer::new(sm.base_offset(id), &interner, &dcx).lex(text);

    let program = etac_ast::ast::Program::parse(&tokens, &dcx);
    let errors = dcx.err_count();
    assert_eq!(
        program.is_clean(),
        errors == 0,
        "the result says whether anything was reported"
    );

    let _ = etac_ast::ast::Interface::parse(&tokens, &dcx);

    dcx.cancel();
    errors
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

/// Chains are parsed in loops, so the recursion cap never sees them, but every link makes the tree one node taller
/// and the tree is dropped, and later walked, by recursion. A long one must be reported, not built.
#[test]
fn long_flat_chains_are_capped() {
    const LONG: usize = 200_000;

    // within the cap: a long but reasonable sum is just a sum
    assert_eq!(errors_in(&format!("f() {{ x: int = {}1 }}", "1 + ".repeat(1000))), 0);
    assert_eq!(
        errors_in(&format!("f(a: int[]) {{ x: int = a{} }}", "[0]".repeat(1000))),
        0
    );
    assert_eq!(errors_in(&format!("f() {{ x: int{} }}", "[]".repeat(1000))), 0);

    // past it: one diagnostic for the whole chain, however long it goes on
    assert_eq!(errors_in(&format!("f() {{ x: int = {}1 }}", "1 + ".repeat(LONG))), 1);
    assert_eq!(
        errors_in(&format!("f(a: int) {{ x: int = {}a }}", "a + ".repeat(LONG))),
        1
    );
    assert_eq!(
        errors_in(&format!("f() {{ x: int = 1 + {}1 }}", "1 * ".repeat(LONG))),
        1
    );
    assert_eq!(
        errors_in(&format!("f() {{ x: bool = {}true }}", "true & 1 < 2 | ".repeat(LONG))),
        1
    );
    assert_eq!(
        errors_in(&format!("f(a: int[]) {{ x: int = a{} }}", "[0]".repeat(LONG))),
        1
    );
    assert_eq!(errors_in(&format!("f() {{ x: int{} }}", "[]".repeat(LONG))), 1);
    assert_eq!(errors_in(&format!("f(n: int) {{ x: int{} }}", "[n]".repeat(LONG))), 1);

    // a chain that is itself an operand of a chain
    let inner = format!("({}1)", "1 + ".repeat(1000));
    fuzz_one(&format!("f() {{ x: int = {}1 }}", format!("{inner} * ").repeat(2000)));
    fuzz_one(&format!(
        "f() {{ x: int = {}{inner}{} }}",
        "-(".repeat(200),
        ")".repeat(200)
    ));
    // unterminated, so recovery has to deal with it too
    fuzz_one(&format!("f() {{ x: int = {}", "1 + ".repeat(LONG)));
    fuzz_one(&format!("f(a: int[]) {{ x: int = a{}", "[0".repeat(LONG)));
    fuzz_one(&format!("f() {{ x: int{}", "[".repeat(LONG)));
}

/// One `else if` per case is how long chains get written, so they are not charged against the recursion cap, but
/// they do nest, and that has a cap of its own.
#[test]
fn long_else_if_chains_are_capped() {
    let chain = |arms: usize, last: &str| {
        let arms: String = (0..arms).map(|i| format!("if x == {i} {{ x = 0 }} else ")).collect();
        format!("f(x: int) {{ {arms}{last} }}")
    };

    // far more branches than the recursion cap allows levels
    assert_eq!(errors_in(&chain(1000, "{ x = 1 }")), 0);
    assert_eq!(errors_in(&chain(1000, "if x == 0 { x = 1 }")), 0);

    // past the cap: one diagnostic for the whole chain
    assert_eq!(errors_in(&chain(200_000, "{ x = 1 }")), 1);
    assert_eq!(errors_in(&chain(200_000, "if x == 0 { x = 1 }")), 1);

    // a chain in the last branch of a chain nests on top of it: the caps do not multiply
    let mut nested = "{ x = 1 }".to_owned();
    for _ in 0..100 {
        nested = format!("{{ {} }}", chain(900, &nested));
    }
    assert!(errors_in(&format!("f(x: int) {nested}")) > 0);
    // while one in the first branch does not
    let mut nested = "x = 1".to_owned();
    for _ in 0..100 {
        nested = format!(
            "if x == 0 {{ {nested} }} else {}",
            chain(500, "{ x = 1 }").trim_start_matches("f(x: int) ")
        );
    }
    assert_eq!(errors_in(&format!("f(x: int) {{ {nested} }}")), 0);

    // unterminated
    fuzz_one(&"if x == 0 { x = 0 } else ".repeat(200_000));
    fuzz_one(&format!("f() {{ {}", "if x == 0 { x = 0 } else ".repeat(200_000)));
}
