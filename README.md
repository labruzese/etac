# etac

A compiler for the Eta programming language, written in Rust.

Eta is a small, statically typed imperative language (`int`, `bool`, arrays,
methods, and `use`-based interface files). `etac` currently performs lexical
and syntactic analysis, with an emphasis on good diagnostics and error 
recovery. 

## Build

The toolchain is pinned to nightly by `rust-toolchain.toml`; `rustup` installs it on first use.

```sh
cargo build --release
```

## Usage

```sh
etac [--dlex] [--dparse] [-s <dir>] [-l <dir>] <files...>
```

- `--dlex` — print the tokens of every file that gets lexed
- `--dparse` — print the syntax tree of every file that gets parsed, as an
  S-expression followed by the source each node spans
- `-s`, `--sourcepath <dir>` — where to resolve relative source paths
  (default: the current directory)
- `-l`, `--libpath <dir>` — where to find `.eti` interface files
  (default: the current directory)

`.eta` files are parsed as programs and `.eti` files as interfaces. The dumps
go to stdout and diagnostics to stderr; the exit status is non-zero if any
error was reported.

Example:

```sh
etac --dparse examples/fibonacci.eta
```

## Tests

```sh
cargo test --workspace
```

Most lexer and parser tests are `expect-test` snapshots of the `--dlex` and
`--dparse` output; run with `UPDATE_EXPECT=1` to rewrite them after an
intentional change. `etac_parse/tests/reference.rs` checks that the parser
accepts and rejects the same files as the course's reference parser.

## Layout

The workspace lives under `compiler/`:

- `etac` — the binary
- `etac_cli` — command line flags
- `etac_driver` — runs the phases over the files named on the command line
- `etac_session` — state shared by a whole compilation: sources and interned
  strings
- `etac_source` — the source map, file ids and spans
- `etac_intern` — string interning
- `etac_errors` — diagnostics: building, collecting and rendering them
- `etac_resolve` — finding source and interface files on disk
- `etac_lexer` — tokens and the lexer
- `etac_ast` — the syntax tree, its visitors and its printers
- `etac_parse` — the hand-written recursive descent parser
- `etac_frontend` — lexes and parses whole files, with hooks for the debug
  dumps
- `etac_debug` — support code for the tests

## Status

Lexing and parsing are complete; type checking is in progress.

## Acknowledgments
This uses resources (the Eta language specification, example programs, and test
cases) from Cornell's CS 4120 Introduction to Compilers:
https://www.cs.cornell.edu/courses/cs4120/2026sp/

This is an independent personal project, I'm not enrolled in the course. 
