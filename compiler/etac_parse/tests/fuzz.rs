//! The parser must terminate and must not panic, on any input at all.
//!
//! The generator is a deterministic xorshift, so a failure here reproduces exactly.

use std::io::Write;

use etac_ast::display::sexpr::SexpPrinter;
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

/// Parse `src` both ways, walk the tree, and throw it all away. Panics, hangs, stack overflows and debug assertions
/// are the failure modes under test; the output is not.
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
    // walk the whole tree, as every later pass will: printing it visits every node
    write!(std::io::sink(), "{}", SexpPrinter::new(&interner, &program.tree)).unwrap();

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

/// Nesting is limited by memory, not by the parser, and not by the stack of the thread it runs on: these are far
/// deeper than a test thread's stack could take a recursive descent, or the walk and the drop that follow it.
#[test]
fn deep_nesting_is_parsed() {
    const DEEP: usize = 20_000;

    for depth in [50usize, 200, 1000, DEEP] {
        let src = format!("f() {{ x: int = {}1{} }}", "(".repeat(depth), ")".repeat(depth));
        assert_eq!(errors_in(&src), 0, "{depth} parentheses");
    }

    // every way a program can nest
    let nest = |open: &str, close: &str| format!("{}1{}", open.repeat(DEEP), close.repeat(DEEP));
    assert_eq!(errors_in(&format!("f() {{ x: int = {}1 }}", "-".repeat(DEEP))), 0);
    assert_eq!(errors_in(&format!("f() {{ x: bool = {}true }}", "!".repeat(DEEP))), 0);
    assert_eq!(errors_in(&format!("f() {{ x: int = {} }}", nest("g(", ")"))), 0);
    assert_eq!(errors_in(&format!("f() {{ x: int = {} }}", nest("length(", ")"))), 0);
    assert_eq!(errors_in(&format!("f() {{ x: int[] = {} }}", nest("{", "}"))), 0);
    assert_eq!(errors_in(&format!("f(a: int[]) {{ x: int = {} }}", nest("a[", "]"))), 0);
    assert_eq!(errors_in(&format!("f(n: int) {{ x: int[{}] }}", nest("g(", ")"))), 0);
    assert_eq!(errors_in(&format!("f() {}{}", "{".repeat(DEEP), "}".repeat(DEEP))), 0);
    assert_eq!(errors_in(&format!("f() {{ {}x = 1 }}", "if true ".repeat(DEEP))), 0);
    assert_eq!(errors_in(&format!("f() {{ {}x = 1 }}", "while true ".repeat(DEEP))), 0);
    assert_eq!(
        errors_in(&format!("f() {{ {}x = 1 }}", "if true x = 0 else ".repeat(DEEP))),
        0
    );

    // Unbalanced, so recovery has to unwind it too.
    fuzz_one(&format!("f() {{ x: int = {}1 }}", "(".repeat(DEEP)));
    fuzz_one(&format!("f() {{ x: int = {}1 }}", "g(".repeat(DEEP)));
    fuzz_one(&format!("f() {{ x: int[] = {}1 }}", "{".repeat(DEEP)));
    fuzz_one(&format!("f() {}", "{".repeat(DEEP)));
    fuzz_one(&format!("f() {{ {} }}", "if true ".repeat(DEEP)));
    fuzz_one(&format!("f() {{ x: int = {} }}", "-".repeat(DEEP)));
}

/// Chains are parsed in loops and cost the parser no recursion, but every link makes the tree one node deeper, and
/// the tree is walked and dropped by recursion. A long chain is still just a chain.
#[test]
fn long_flat_chains_are_parsed() {
    const LONG: usize = 200_000;

    assert_eq!(errors_in(&format!("f() {{ x: int = {}1 }}", "1 + ".repeat(LONG))), 0);
    assert_eq!(
        errors_in(&format!("f(a: int) {{ x: int = {}a }}", "a + ".repeat(LONG))),
        0
    );
    assert_eq!(
        errors_in(&format!("f() {{ x: int = 1 + {}1 }}", "1 * ".repeat(LONG))),
        0
    );
    assert_eq!(
        errors_in(&format!("f() {{ x: bool = {}true }}", "true & 1 < 2 | ".repeat(LONG))),
        0
    );
    assert_eq!(
        errors_in(&format!("f(a: int[]) {{ x: int = a{} }}", "[0]".repeat(LONG))),
        0
    );
    assert_eq!(errors_in(&format!("f() {{ x: int{} }}", "[]".repeat(LONG))), 0);
    assert_eq!(errors_in(&format!("f(n: int) {{ x: int{} }}", "[n]".repeat(LONG))), 0);

    // a chain that is itself an operand of a chain
    let inner = format!("({}1)", "1 + ".repeat(1000));
    assert_eq!(
        errors_in(&format!("f() {{ x: int = {}1 }}", format!("{inner} * ").repeat(2000))),
        0
    );
    assert_eq!(
        errors_in(&format!(
            "f() {{ x: int = {}{inner}{} }}",
            "-(".repeat(200),
            ")".repeat(200)
        )),
        0
    );
    // unterminated, so recovery has to deal with it too
    fuzz_one(&format!("f() {{ x: int = {}", "1 + ".repeat(LONG)));
    fuzz_one(&format!("f(a: int[]) {{ x: int = a{}", "[0".repeat(LONG)));
    fuzz_one(&format!("f() {{ x: int{}", "[".repeat(LONG)));
}

/// One `else if` per case is how long chains get written. Each one nests in the one before it, as deep as the chain
/// is long.
#[test]
fn long_else_if_chains_are_parsed() {
    // the statement, without a method around it
    let chain = |arms: usize, last: &str| {
        let arms: String = (0..arms).map(|i| format!("if x == {i} {{ x = 0 }} else ")).collect();
        format!("{arms}{last}")
    };
    let errors_in_body = |body: &str| errors_in(&format!("f(x: int) {{ {body} }}"));

    assert_eq!(errors_in_body(&chain(1000, "{ x = 1 }")), 0);
    assert_eq!(errors_in_body(&chain(1000, "if x == 0 { x = 1 }")), 0);
    assert_eq!(errors_in_body(&chain(200_000, "{ x = 1 }")), 0);
    assert_eq!(errors_in_body(&chain(200_000, "if x == 0 { x = 1 }")), 0);

    // a chain in the last branch of a chain nests on top of it
    let mut nested = "{ x = 1 }".to_owned();
    for _ in 0..100 {
        nested = format!("{{ {} }}", chain(900, &nested));
    }
    assert_eq!(errors_in_body(&nested), 0);
    // and so does one in the first branch
    let mut nested = "x = 1".to_owned();
    for _ in 0..100 {
        nested = format!("if x == 0 {{ {nested} }} else {}", chain(500, "{ x = 1 }"));
    }
    assert_eq!(errors_in_body(&nested), 0);

    // unterminated
    fuzz_one(&"if x == 0 { x = 0 } else ".repeat(200_000));
    fuzz_one(&format!("f() {{ {}", "if x == 0 { x = 0 } else ".repeat(200_000)));
}
