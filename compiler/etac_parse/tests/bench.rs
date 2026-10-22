//! Throughput, for when a grammar change needs a sanity check.
//!
//! `cargo test --release -p etac_parse --test bench -- --ignored --nocapture`
//!
//! Ignored by default: it is a measurement, not an assertion.

use std::time::Instant;

use etac_errors::dcx::DiagCtx;
use etac_intern::EtacInterner;
use etac_parse::{Parse, result::ParseResult};
use etac_source::SourceMap;

#[test]
#[ignore = "measurement, not an assertion; run with --ignored --nocapture"]
fn throughput() {
    let mut src = String::new();
    for _ in 0..200 {
        // this will generate at least use errors
        src.push_str(include_str!("bench/main.eta"));
    }
    let bytes = src.len();

    let (mut lex_ns, mut parse_ns, mut ntok) = (u128::MAX, u128::MAX, 0usize);
    for _ in 0..7 {
        let emitter = etac_errors::emitter::IoEmitter::sink();
        let dcx = DiagCtx::new(emitter);
        let sm = SourceMap::default();
        let mut interner = EtacInterner::with_hasher(std::hash::BuildHasherDefault::default());
        let (id, text) = sm.alloc_source("bench.eta".to_owned(), src.clone());

        let t0 = Instant::now();
        let lexed = etac_lexer::EtaLexer::new(sm.base_offset(id), &mut interner, &dcx).lex(text);
        let t1 = Instant::now();
        let ParseResult { parse: _, diags } = etac_ast::ast::Program::parse(&lexed.tokens, &dcx);
        let t2 = Instant::now();

        ntok = lexed.tokens.kinds.len();
        lex_ns = lex_ns.min((t1 - t0).as_nanos());
        parse_ns = parse_ns.min((t2 - t1).as_nanos());

        for d in diags {
            d.cancel();
        }
        for d in lexed.diags {
            d.cancel();
        }
        dcx.cancel();
    }

    let mb = bytes as f64 / 1_048_576.0;
    eprintln!("source   {bytes} bytes, {ntok} tokens");
    eprintln!(
        "lex      {:>7.2} ms  {:>6.1} MB/s",
        lex_ns as f64 / 1e6,
        mb / (lex_ns as f64 / 1e9)
    );
    eprintln!(
        "parse    {:>7.2} ms  {:>6.1} MB/s  {:.1} ns/token",
        parse_ns as f64 / 1e6,
        mb / (parse_ns as f64 / 1e9),
        parse_ns as f64 / ntok as f64
    );
}
