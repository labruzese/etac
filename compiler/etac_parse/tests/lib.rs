#![allow(unused_assignments)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use expect_test::expect;
use indoc::indoc;

#[macro_use]
pub mod parse;

#[test]
fn add() {
    parsetest! {
        ".eta",
        indoc![r#"
            +++
        "#],
        expect![[r#"
            src0.eta: parse(recovered)
            (program
              (uses)
              (defs
                (error)))
            program         : src0.eta:1:1..1:4

            Error: unexpected token `+`
               ╭─[ src0.eta:1:1 ]
               │
             1 │ +++
               │ ┬  
               │ ╰── expected one of "use", or "identifier"
            ───╯
        "#]]
    }
}

#[test]
fn all_comparisons() {
    parsetest! {
        ".eta",
        indoc![r#"
            compare(a: int, b: int) {
                c1: bool = a < b
                c2: bool = a <= b
                c3: bool = a > b
                c4: bool = a >= b
                c5: bool = a == b
                c6: bool = a != b
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl compare (params (: a int) (: b int)) (returns))
                  (block
                    (assign (targets (: c1 bool)) (values (< a b)))
                    (assign (targets (: c2 bool)) (values (<= a b)))
                    (assign (targets (: c3 bool)) (values (> a b)))
                    (assign (targets (: c4 bool)) (values (>= a b)))
                    (assign (targets (: c5 bool)) (values (== a b)))
                    (assign (targets (: c6 bool)) (values (!= a b)))))))
            program         : src0.eta:1:1..8:2
            method          : src0.eta:1:1..8:2
            mdecl           : src0.eta:1:1..1:24
            id compare      : src0.eta:1:1..1:8
            decl            : src0.eta:1:9..1:15
            id a            : src0.eta:1:9..1:10
            decl            : src0.eta:1:17..1:23
            id b            : src0.eta:1:17..1:18
            block           : src0.eta:1:25..8:2
            assign          : src0.eta:2:5..2:21
            bexpr           : src0.eta:2:16..2:21
            binop           : src0.eta:2:18..2:19
            id a            : src0.eta:2:16..2:17
            id b            : src0.eta:2:20..2:21
            decl            : src0.eta:2:5..2:13
            id c1           : src0.eta:2:5..2:7
            assign          : src0.eta:3:5..3:22
            bexpr           : src0.eta:3:16..3:22
            binop           : src0.eta:3:18..3:20
            id a            : src0.eta:3:16..3:17
            id b            : src0.eta:3:21..3:22
            decl            : src0.eta:3:5..3:13
            id c2           : src0.eta:3:5..3:7
            assign          : src0.eta:4:5..4:21
            bexpr           : src0.eta:4:16..4:21
            binop           : src0.eta:4:18..4:19
            id a            : src0.eta:4:16..4:17
            id b            : src0.eta:4:20..4:21
            decl            : src0.eta:4:5..4:13
            id c3           : src0.eta:4:5..4:7
            assign          : src0.eta:5:5..5:22
            bexpr           : src0.eta:5:16..5:22
            binop           : src0.eta:5:18..5:20
            id a            : src0.eta:5:16..5:17
            id b            : src0.eta:5:21..5:22
            decl            : src0.eta:5:5..5:13
            id c4           : src0.eta:5:5..5:7
            assign          : src0.eta:6:5..6:22
            bexpr           : src0.eta:6:16..6:22
            binop           : src0.eta:6:18..6:20
            id a            : src0.eta:6:16..6:17
            id b            : src0.eta:6:21..6:22
            decl            : src0.eta:6:5..6:13
            id c5           : src0.eta:6:5..6:7
            assign          : src0.eta:7:5..7:22
            bexpr           : src0.eta:7:16..7:22
            binop           : src0.eta:7:18..7:20
            id a            : src0.eta:7:16..7:17
            id b            : src0.eta:7:21..7:22
            decl            : src0.eta:7:5..7:13
            id c6           : src0.eta:7:5..7:7

        "#]]
    }
}

#[test]
fn array_chain_index() {
    parsetest! {
        ".eta",
        indoc![r#"
            f() {
                a: int[][] = {{1, 2}, {3, 4}}
                x: int = a[0][1]
                a[1][0] = 5
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl f (params) (returns))
                  (block
                    (assign (targets (: a (array (array int)))) (values (array (array 1 2) (array 3 4))))
                    (assign (targets (: x int)) (values (index (index a 0) 1)))
                    (assign (targets (index (index a 1) 0)) (values 5))))))
            program         : src0.eta:1:1..5:2
            method          : src0.eta:1:1..5:2
            mdecl           : src0.eta:1:1..1:4
            id f            : src0.eta:1:1..1:2
            block           : src0.eta:1:5..5:2
            assign          : src0.eta:2:5..2:34
            arr_lit         : src0.eta:2:18..2:34
            arr_lit         : src0.eta:2:19..2:25
            int_lit         : src0.eta:2:20..2:21
            int_lit         : src0.eta:2:23..2:24
            arr_lit         : src0.eta:2:27..2:33
            int_lit         : src0.eta:2:28..2:29
            int_lit         : src0.eta:2:31..2:32
            decl            : src0.eta:2:5..2:15
            arr_t           : src0.eta:2:13..2:15
            arr_t           : src0.eta:2:11..2:13
            id a            : src0.eta:2:5..2:6
            assign          : src0.eta:3:5..3:21
            expr_idx        : src0.eta:3:14..3:21
            expr_idx        : src0.eta:3:14..3:18
            id a            : src0.eta:3:14..3:15
            int_lit         : src0.eta:3:16..3:17
            int_lit         : src0.eta:3:19..3:20
            decl            : src0.eta:3:5..3:11
            id x            : src0.eta:3:5..3:6
            assign          : src0.eta:4:5..4:16
            int_lit         : src0.eta:4:15..4:16
            expr_idx        : src0.eta:4:5..4:12
            expr_idx        : src0.eta:4:5..4:9
            id a            : src0.eta:4:5..4:6
            int_lit         : src0.eta:4:7..4:8
            int_lit         : src0.eta:4:10..4:11

        "#]]
    }
}

#[test]
fn array_decls() {
    parsetest! {
        ".eta",
        indoc![r#"
            f() {
                a: int[][]
                b: int[i][]
                d: int[i][j]
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl f (params) (returns))
                  (block
                    (decls (: a (array (array int))))
                    (decls (: b (array (array int i))))
                    (decls (: d (array (array int i) j)))))))
            program         : src0.eta:1:1..5:2
            method          : src0.eta:1:1..5:2
            mdecl           : src0.eta:1:1..1:4
            id f            : src0.eta:1:1..1:2
            block           : src0.eta:1:5..5:2
            decls           : src0.eta:2:5..2:15
            decl            : src0.eta:2:5..2:15
            arr_t           : src0.eta:2:13..2:15
            arr_t           : src0.eta:2:11..2:13
            id a            : src0.eta:2:5..2:6
            decls           : src0.eta:3:5..3:16
            decl            : src0.eta:3:5..3:16
            arr_t           : src0.eta:3:14..3:16
            arr_t           : src0.eta:3:11..3:14
            id i            : src0.eta:3:12..3:13
            id b            : src0.eta:3:5..3:6
            decls           : src0.eta:4:5..4:17
            decl            : src0.eta:4:5..4:17
            arr_t           : src0.eta:4:14..4:17
            arr_t           : src0.eta:4:11..4:14
            id i            : src0.eta:4:12..4:13
            id j            : src0.eta:4:15..4:16
            id d            : src0.eta:4:5..4:6

        "#]]
    }
}

#[test]
fn arrayinit() {
    parsetest! {
        ".eta",
        indoc![r#"
            foo() {
                a: int[] = {72,101,108,108,111}
                a: int[] = "Hello"
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl foo (params) (returns))
                  (block
                    (assign (targets (: a (array int))) (values (array 72 101 108 108 111)))
                    (assign (targets (: a (array int))) (values Spur(3)))))))
            program         : src0.eta:1:1..4:2
            method          : src0.eta:1:1..4:2
            mdecl           : src0.eta:1:1..1:6
            id foo          : src0.eta:1:1..1:4
            block           : src0.eta:1:7..4:2
            assign          : src0.eta:2:5..2:36
            arr_lit         : src0.eta:2:16..2:36
            int_lit         : src0.eta:2:17..2:19
            int_lit         : src0.eta:2:20..2:23
            int_lit         : src0.eta:2:24..2:27
            int_lit         : src0.eta:2:28..2:31
            int_lit         : src0.eta:2:32..2:35
            decl            : src0.eta:2:5..2:13
            arr_t           : src0.eta:2:11..2:13
            id a            : src0.eta:2:5..2:6
            assign          : src0.eta:3:5..3:23
            string_lit      : src0.eta:3:16..3:23
            decl            : src0.eta:3:5..3:13
            arr_t           : src0.eta:3:11..3:13
            id a            : src0.eta:3:5..3:6

        "#]]
    }
}

#[test]
fn arrayinit2() {
    parsetest! {
        ".eta",
        indoc![r#"
            foo() {
                n: int = gcd(10, 2)
                a: int[n]
                while (n > 0) {
                  n = n - 1
                  a[n] = n
                }
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl foo (params) (returns))
                  (block
                    (assign (targets (: n int)) (values (call gcd 10 2)))
                    (decls (: a (array int n)))
                    (while (> n 0)
                      (block
                        (assign (targets n) (values (- n 1)))
                        (assign (targets (index a n)) (values n))))))))
            program         : src0.eta:1:1..8:2
            method          : src0.eta:1:1..8:2
            mdecl           : src0.eta:1:1..1:6
            id foo          : src0.eta:1:1..1:4
            block           : src0.eta:1:7..8:2
            assign          : src0.eta:2:5..2:24
            pcall           : src0.eta:2:14..2:24
            id gcd          : src0.eta:2:14..2:17
            int_lit         : src0.eta:2:18..2:20
            int_lit         : src0.eta:2:22..2:23
            decl            : src0.eta:2:5..2:11
            id n            : src0.eta:2:5..2:6
            decls           : src0.eta:3:5..3:14
            decl            : src0.eta:3:5..3:14
            arr_t           : src0.eta:3:11..3:14
            id n            : src0.eta:3:12..3:13
            id a            : src0.eta:3:5..3:6
            while           : src0.eta:4:5..7:6
            bexpr           : src0.eta:4:12..4:17
            binop           : src0.eta:4:14..4:15
            id n            : src0.eta:4:12..4:13
            int_lit         : src0.eta:4:16..4:17
            block           : src0.eta:4:19..7:6
            assign          : src0.eta:5:7..5:16
            bexpr           : src0.eta:5:11..5:16
            binop           : src0.eta:5:13..5:14
            id n            : src0.eta:5:11..5:12
            int_lit         : src0.eta:5:15..5:16
            id n            : src0.eta:5:7..5:8
            assign          : src0.eta:6:7..6:15
            id n            : src0.eta:6:14..6:15
            expr_idx        : src0.eta:6:7..6:11
            id a            : src0.eta:6:7..6:8
            id n            : src0.eta:6:9..6:10

        "#]]
    }
}

#[test]
fn beauty() {
    parsetest! {
        ".eta",
        indoc![r#"
            ===============================================================================
            = This is a beautiful document heading, not a xi program, but it still lexes! =
            ===============================================================================
        "#],
        expect![[r#"
            src0.eta: parse(recovered)
            (program
              (uses)
              (defs
                (error)
                (error)
                (error)
                (error)
                (error)
                (error)
                (error)
                (error)
                (error)
                (error)
                (error)
                (error)
                (error)
                (error)
                (error)))
            program         : src0.eta:1:1..3:80

            Error: unexpected token `==`
               ╭─[ src0.eta:1:1 ]
               │
             1 │ ===============================================================================
               │ ─┬  
               │  ╰── expected one of "use", or "identifier"
            ───╯
            Error: unexpected token `is`
               ╭─[ src0.eta:2:8 ]
               │
             2 │ = This is a beautiful document heading, not a xi program, but it still lexes! =
               │        ─┬  
               │         ╰── expected one of ":", or "("
            ───╯
            Error: unexpected token `a`
               ╭─[ src0.eta:2:11 ]
               │
             2 │ = This is a beautiful document heading, not a xi program, but it still lexes! =
               │           ┬  
               │           ╰── expected one of ":", or "("
            ───╯
            Error: unexpected token `beautiful`
               ╭─[ src0.eta:2:13 ]
               │
             2 │ = This is a beautiful document heading, not a xi program, but it still lexes! =
               │             ────┬────  
               │                 ╰────── expected one of ":", or "("
            ───╯
            Error: unexpected token `document`
               ╭─[ src0.eta:2:23 ]
               │
             2 │ = This is a beautiful document heading, not a xi program, but it still lexes! =
               │                       ────┬───  
               │                           ╰───── expected one of ":", or "("
            ───╯
            Error: unexpected token `heading`
               ╭─[ src0.eta:2:32 ]
               │
             2 │ = This is a beautiful document heading, not a xi program, but it still lexes! =
               │                                ───┬───  
               │                                   ╰───── expected one of ":", or "("
            ───╯
            Error: unexpected token `,`
               ╭─[ src0.eta:2:39 ]
               │
             2 │ = This is a beautiful document heading, not a xi program, but it still lexes! =
               │                                       ┬  
               │                                       ╰── expected one of ":", or "("
            ───╯
            Error: unexpected token `a`
               ╭─[ src0.eta:2:45 ]
               │
             2 │ = This is a beautiful document heading, not a xi program, but it still lexes! =
               │                                             ┬  
               │                                             ╰── expected one of ":", or "("
            ───╯
            Error: unexpected token `xi`
               ╭─[ src0.eta:2:47 ]
               │
             2 │ = This is a beautiful document heading, not a xi program, but it still lexes! =
               │                                               ─┬  
               │                                                ╰── expected one of ":", or "("
            ───╯
            Error: unexpected token `program`
               ╭─[ src0.eta:2:50 ]
               │
             2 │ = This is a beautiful document heading, not a xi program, but it still lexes! =
               │                                                  ───┬───  
               │                                                     ╰───── expected one of ":", or "("
            ───╯
            Error: unexpected token `,`
               ╭─[ src0.eta:2:57 ]
               │
             2 │ = This is a beautiful document heading, not a xi program, but it still lexes! =
               │                                                         ┬  
               │                                                         ╰── expected one of ":", or "("
            ───╯
            Error: unexpected token `it`
               ╭─[ src0.eta:2:63 ]
               │
             2 │ = This is a beautiful document heading, not a xi program, but it still lexes! =
               │                                                               ─┬  
               │                                                                ╰── expected one of ":", or "("
            ───╯
            Error: unexpected token `still`
               ╭─[ src0.eta:2:66 ]
               │
             2 │ = This is a beautiful document heading, not a xi program, but it still lexes! =
               │                                                                  ──┬──  
               │                                                                    ╰──── expected one of ":", or "("
            ───╯
            Error: unexpected token `lexes`
               ╭─[ src0.eta:2:72 ]
               │
             2 │ = This is a beautiful document heading, not a xi program, but it still lexes! =
               │                                                                        ──┬──  
               │                                                                          ╰──── expected one of ":", or "("
            ───╯
            Error: unexpected token `!`
               ╭─[ src0.eta:2:77 ]
               │
             2 │ = This is a beautiful document heading, not a xi program, but it still lexes! =
               │                                                                             ┬  
               │                                                                             ╰── expected one of ":", or "("
            ───╯
        "#]]
    }
}

#[test]
fn bool_short_circuit() {
    parsetest! {
        ".eta",
        indoc![r#"
            f(a: bool, b: bool, c: bool): bool {
                d: bool = a & b | c & !a
                e: bool = (a | b) & (b | c)
                return d == e
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl f (params (: a bool) (: b bool) (: c bool)) (returns bool))
                  (block
                    (assign (targets (: d bool)) (values (| (& a b) (& c (! a)))))
                    (assign (targets (: e bool)) (values (& (| a b) (| b c))))
                    (return (== d e))))))
            program         : src0.eta:1:1..5:2
            method          : src0.eta:1:1..5:2
            mdecl           : src0.eta:1:1..1:35
            id f            : src0.eta:1:1..1:2
            decl            : src0.eta:1:3..1:10
            id a            : src0.eta:1:3..1:4
            decl            : src0.eta:1:12..1:19
            id b            : src0.eta:1:12..1:13
            decl            : src0.eta:1:21..1:28
            id c            : src0.eta:1:21..1:22
            block           : src0.eta:1:36..5:2
            assign          : src0.eta:2:5..2:29
            bexpr           : src0.eta:2:15..2:29
            binop           : src0.eta:2:21..2:22
            bexpr           : src0.eta:2:15..2:20
            binop           : src0.eta:2:17..2:18
            id a            : src0.eta:2:15..2:16
            id b            : src0.eta:2:19..2:20
            bexpr           : src0.eta:2:23..2:29
            binop           : src0.eta:2:25..2:26
            id c            : src0.eta:2:23..2:24
            uexpr           : src0.eta:2:27..2:29
            uop             : src0.eta:2:27..2:28
            id a            : src0.eta:2:28..2:29
            decl            : src0.eta:2:5..2:12
            id d            : src0.eta:2:5..2:6
            assign          : src0.eta:3:5..3:32
            bexpr           : src0.eta:3:15..3:32
            binop           : src0.eta:3:23..3:24
            bexpr           : src0.eta:3:16..3:21
            binop           : src0.eta:3:18..3:19
            id a            : src0.eta:3:16..3:17
            id b            : src0.eta:3:20..3:21
            bexpr           : src0.eta:3:26..3:31
            binop           : src0.eta:3:28..3:29
            id b            : src0.eta:3:26..3:27
            id c            : src0.eta:3:30..3:31
            decl            : src0.eta:3:5..3:12
            id e            : src0.eta:3:5..3:6
            return          : src0.eta:4:5..4:18
            bexpr           : src0.eta:4:12..4:18
            binop           : src0.eta:4:14..4:16
            id d            : src0.eta:4:12..4:13
            id e            : src0.eta:4:17..4:18

        "#]]
    }
}

#[test]
fn chained_comparison() {
    parsetest! {
        ".eta",
        indoc![r#"
            f(a: int, b: int, c: int): bool {
                return a < b < c
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl f (params (: a int) (: b int) (: c int)) (returns bool))
                  (block
                    (return (< (< a b) c))))))
            program         : src0.eta:1:1..3:2
            method          : src0.eta:1:1..3:2
            mdecl           : src0.eta:1:1..1:32
            id f            : src0.eta:1:1..1:2
            decl            : src0.eta:1:3..1:9
            id a            : src0.eta:1:3..1:4
            decl            : src0.eta:1:11..1:17
            id b            : src0.eta:1:11..1:12
            decl            : src0.eta:1:19..1:25
            id c            : src0.eta:1:19..1:20
            block           : src0.eta:1:33..3:2
            return          : src0.eta:2:5..2:21
            bexpr           : src0.eta:2:12..2:21
            binop           : src0.eta:2:18..2:19
            bexpr           : src0.eta:2:12..2:17
            binop           : src0.eta:2:14..2:15
            id a            : src0.eta:2:12..2:13
            id b            : src0.eta:2:16..2:17
            id c            : src0.eta:2:20..2:21

        "#]]
    }
}

#[test]
fn chained_if() {
    parsetest! {
        ".eta",
        indoc![r#"
            classify(n: int): int {
                r: int = 0
                if n < 0 r = -1
                else if n == 0 r = 0
                else r = 1
                return r
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl classify (params (: n int)) (returns int))
                  (block
                    (assign (targets (: r int)) (values 0))
                    (if (< n 0)
                      (assign (targets r) (values (- 1)))
                      (if (== n 0)
                        (assign (targets r) (values 0))
                        (assign (targets r) (values 1))))
                    (return r)))))
            program         : src0.eta:1:1..7:2
            method          : src0.eta:1:1..7:2
            mdecl           : src0.eta:1:1..1:22
            id classify     : src0.eta:1:1..1:9
            decl            : src0.eta:1:10..1:16
            id n            : src0.eta:1:10..1:11
            block           : src0.eta:1:23..7:2
            assign          : src0.eta:2:5..2:15
            int_lit         : src0.eta:2:14..2:15
            decl            : src0.eta:2:5..2:11
            id r            : src0.eta:2:5..2:6
            if              : src0.eta:3:5..5:15
            bexpr           : src0.eta:3:8..3:13
            binop           : src0.eta:3:10..3:11
            id n            : src0.eta:3:8..3:9
            int_lit         : src0.eta:3:12..3:13
            assign          : src0.eta:3:14..3:20
            uexpr           : src0.eta:3:18..3:20
            uop             : src0.eta:3:18..3:19
            int_lit         : src0.eta:3:19..3:20
            id r            : src0.eta:3:14..3:15
            if              : src0.eta:4:10..5:15
            bexpr           : src0.eta:4:13..4:19
            binop           : src0.eta:4:15..4:17
            id n            : src0.eta:4:13..4:14
            int_lit         : src0.eta:4:18..4:19
            assign          : src0.eta:4:20..4:25
            int_lit         : src0.eta:4:24..4:25
            id r            : src0.eta:4:20..4:21
            assign          : src0.eta:5:10..5:15
            int_lit         : src0.eta:5:14..5:15
            id r            : src0.eta:5:10..5:11
            return          : src0.eta:6:5..6:13
            id r            : src0.eta:6:12..6:13

        "#]]
    }
}

#[test]
fn complex_interface() {
    parsetest! {
        ".eti",
        indoc![r#"
            // Math operations
            add(a: int, b: int): int
            subtract(a: int, b: int): int
            divide(a: int, b: int): int, int
            noop()
            identity(x: int[]): int[]
            matrix(m: int[][]): int[][]
        "#],
        expect![[r#"
            src0.eti: parse(clean)
            (interface
              (decl add (params (: a int) (: b int)) (returns int))
              (decl subtract (params (: a int) (: b int)) (returns int))
              (decl divide (params (: a int) (: b int)) (returns int int))
              (decl noop (params) (returns))
              (decl identity (params (: x (array int))) (returns (array int)))
              (decl matrix (params (: m (array (array int)))) (returns (array (array int)))))
            interface       : src0.eti:2:1..7:28
            mdecl           : src0.eti:2:1..2:25
            id add          : src0.eti:2:1..2:4
            decl            : src0.eti:2:5..2:11
            id a            : src0.eti:2:5..2:6
            decl            : src0.eti:2:13..2:19
            id b            : src0.eti:2:13..2:14
            mdecl           : src0.eti:3:1..3:30
            id subtract     : src0.eti:3:1..3:9
            decl            : src0.eti:3:10..3:16
            id a            : src0.eti:3:10..3:11
            decl            : src0.eti:3:18..3:24
            id b            : src0.eti:3:18..3:19
            mdecl           : src0.eti:4:1..4:33
            id divide       : src0.eti:4:1..4:7
            decl            : src0.eti:4:8..4:14
            id a            : src0.eti:4:8..4:9
            decl            : src0.eti:4:16..4:22
            id b            : src0.eti:4:16..4:17
            mdecl           : src0.eti:5:1..5:7
            id noop         : src0.eti:5:1..5:5
            mdecl           : src0.eti:6:1..6:26
            id identity     : src0.eti:6:1..6:9
            decl            : src0.eti:6:10..6:18
            arr_t           : src0.eti:6:16..6:18
            id x            : src0.eti:6:10..6:11
            arr_t           : src0.eti:6:24..6:26
            mdecl           : src0.eti:7:1..7:28
            id matrix       : src0.eti:7:1..7:7
            decl            : src0.eti:7:8..7:18
            arr_t           : src0.eti:7:16..7:18
            arr_t           : src0.eti:7:14..7:16
            id m            : src0.eti:7:8..7:9
            arr_t           : src0.eti:7:26..7:28
            arr_t           : src0.eti:7:24..7:26

        "#]]
    }
}

#[test]
fn dangling_else() {
    parsetest! {
        ".eta",
        indoc![r#"
            f(a: bool, b: bool) {
                x: int = 0
                if a if b x = 1 else x = 2
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl f (params (: a bool) (: b bool)) (returns))
                  (block
                    (assign (targets (: x int)) (values 0))
                    (if a
                      (if b
                        (assign (targets x) (values 1))
                        (assign (targets x) (values 2))))))))
            program         : src0.eta:1:1..4:2
            method          : src0.eta:1:1..4:2
            mdecl           : src0.eta:1:1..1:20
            id f            : src0.eta:1:1..1:2
            decl            : src0.eta:1:3..1:10
            id a            : src0.eta:1:3..1:4
            decl            : src0.eta:1:12..1:19
            id b            : src0.eta:1:12..1:13
            block           : src0.eta:1:21..4:2
            assign          : src0.eta:2:5..2:15
            int_lit         : src0.eta:2:14..2:15
            decl            : src0.eta:2:5..2:11
            id x            : src0.eta:2:5..2:6
            if              : src0.eta:3:5..3:31
            id a            : src0.eta:3:8..3:9
            if              : src0.eta:3:10..3:31
            id b            : src0.eta:3:13..3:14
            assign          : src0.eta:3:15..3:20
            int_lit         : src0.eta:3:19..3:20
            id x            : src0.eta:3:15..3:16
            assign          : src0.eta:3:26..3:31
            int_lit         : src0.eta:3:30..3:31
            id x            : src0.eta:3:26..3:27

        "#]]
    }
}

#[test]
fn deeply_nested() {
    parsetest! {
        ".eta",
        indoc![r#"
            f(): int {
                x: int = ((1 + 2) * (3 + 4)) / ((5 - 6) * (7 - 8))
                return x
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl f (params) (returns int))
                  (block
                    (assign (targets (: x int)) (values (/ (* (+ 1 2) (+ 3 4)) (* (- 5 6) (- 7 8)))))
                    (return x)))))
            program         : src0.eta:1:1..4:2
            method          : src0.eta:1:1..4:2
            mdecl           : src0.eta:1:1..1:9
            id f            : src0.eta:1:1..1:2
            block           : src0.eta:1:10..4:2
            assign          : src0.eta:2:5..2:55
            bexpr           : src0.eta:2:14..2:55
            binop           : src0.eta:2:34..2:35
            bexpr           : src0.eta:2:15..2:32
            binop           : src0.eta:2:23..2:24
            bexpr           : src0.eta:2:16..2:21
            binop           : src0.eta:2:18..2:19
            int_lit         : src0.eta:2:16..2:17
            int_lit         : src0.eta:2:20..2:21
            bexpr           : src0.eta:2:26..2:31
            binop           : src0.eta:2:28..2:29
            int_lit         : src0.eta:2:26..2:27
            int_lit         : src0.eta:2:30..2:31
            bexpr           : src0.eta:2:37..2:54
            binop           : src0.eta:2:45..2:46
            bexpr           : src0.eta:2:38..2:43
            binop           : src0.eta:2:40..2:41
            int_lit         : src0.eta:2:38..2:39
            int_lit         : src0.eta:2:42..2:43
            bexpr           : src0.eta:2:48..2:53
            binop           : src0.eta:2:50..2:51
            int_lit         : src0.eta:2:48..2:49
            int_lit         : src0.eta:2:52..2:53
            decl            : src0.eta:2:5..2:11
            id x            : src0.eta:2:5..2:6
            return          : src0.eta:3:5..3:13
            id x            : src0.eta:3:12..3:13

        "#]]
    }
}

#[test]
fn empty_array() {
    parsetest! {
        ".eta",
        indoc![r#"
            f() {
                a: int[] = {}
                b: bool[] = {}
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl f (params) (returns))
                  (block
                    (assign (targets (: a (array int))) (values (array)))
                    (assign (targets (: b (array bool))) (values (array)))))))
            program         : src0.eta:1:1..4:2
            method          : src0.eta:1:1..4:2
            mdecl           : src0.eta:1:1..1:4
            id f            : src0.eta:1:1..1:2
            block           : src0.eta:1:5..4:2
            assign          : src0.eta:2:5..2:18
            arr_lit         : src0.eta:2:16..2:18
            decl            : src0.eta:2:5..2:13
            arr_t           : src0.eta:2:11..2:13
            id a            : src0.eta:2:5..2:6
            assign          : src0.eta:3:5..3:19
            arr_lit         : src0.eta:3:17..3:19
            decl            : src0.eta:3:5..3:14
            arr_t           : src0.eta:3:12..3:14
            id b            : src0.eta:3:5..3:6

        "#]]
    }
}

#[test]
fn empty_body() {
    parsetest! {
        ".eta",
        indoc![r#"
            f() {
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl f (params) (returns))
                  (block))))
            program         : src0.eta:1:1..2:2
            method          : src0.eta:1:1..2:2
            mdecl           : src0.eta:1:1..1:4
            id f            : src0.eta:1:1..1:2
            block           : src0.eta:1:5..2:2

        "#]]
    }
}

#[test]
fn err_toss_indexing() {
    parsetest! {
        ".eta",
        indoc![r#"
            main(args: int[][]) {
                _[0] = true
            }
        "#],
        expect![[r#"
            src0.eta: parse(recovered)
            (program
              (uses)
              (defs
                (method (decl main (params (: args (array (array int)))) (returns))
                  (block
                    (error)))))
            program         : src0.eta:1:1..3:2
            method          : src0.eta:1:1..3:2
            mdecl           : src0.eta:1:1..1:20
            id main         : src0.eta:1:1..1:5
            decl            : src0.eta:1:6..1:19
            arr_t           : src0.eta:1:17..1:19
            arr_t           : src0.eta:1:15..1:17
            id args         : src0.eta:1:6..1:10
            block           : src0.eta:1:21..3:2

            Error: unexpected token `[`
               ╭─[ src0.eta:2:6 ]
               │
             2 │     _[0] = true
               │      ┬  
               │      ╰── expected one of "=", or ","
            ───╯
        "#]]
    }
}

#[test]
fn error_assign_brace() {
    parsetest! {
        ".eta",
        indoc![r#"
            f() {
                {1,2,3}[0] = 5
            }
        "#],
        expect![[r#"
            src0.eta: parse(recovered)
            (program
              (uses)
              (defs
                (method (decl f (params) (returns))
                  (block
                    (block
                      (error))
                    (error)))))
            program         : src0.eta:1:1..3:2
            method          : src0.eta:1:1..3:2
            mdecl           : src0.eta:1:1..1:4
            id f            : src0.eta:1:1..1:2
            block           : src0.eta:1:5..3:2
            block           : src0.eta:2:5..2:12

            Error: unexpected token `1`
               ╭─[ src0.eta:2:6 ]
               │
             2 │     {1,2,3}[0] = 5
               │      ┬  
               │      ╰── expected one of "while", "if", "return", "_", "{", "}", or "identifier"
            ───╯
            Error: unexpected token `[`
               ╭─[ src0.eta:2:12 ]
               │
             2 │     {1,2,3}[0] = 5
               │            ┬  
               │            ╰── expected one of "while", "if", "return", ";", "_", "{", "}", or "identifier"
            ───╯
        "#]]
    }
}

#[test]
fn error_assign_paren() {
    parsetest! {
        ".eta",
        indoc![r#"
            f(x: int) {
                (x) = 5
            }
        "#],
        expect![[r#"
            src0.eta: parse(recovered)
            (program
              (uses)
              (defs
                (method (decl f (params (: x int)) (returns))
                  (block
                    (error)
                    (error)))))
            program         : src0.eta:1:1..3:2
            method          : src0.eta:1:1..3:2
            mdecl           : src0.eta:1:1..1:10
            id f            : src0.eta:1:1..1:2
            decl            : src0.eta:1:3..1:9
            id x            : src0.eta:1:3..1:4
            block           : src0.eta:1:11..3:2

            Error: unexpected token `(`
               ╭─[ src0.eta:2:5 ]
               │
             2 │     (x) = 5
               │     ┬  
               │     ╰── expected one of "while", "if", "return", "_", "{", "}", or "identifier"
            ───╯
            Error: unexpected token `)`
               ╭─[ src0.eta:2:7 ]
               │
             2 │     (x) = 5
               │       ┬  
               │       ╰── expected one of ":", "=", ",", "(", or "["
            ───╯
        "#]]
    }
}

#[test]
fn error_assign_string() {
    parsetest! {
        ".eta",
        indoc![r#"
            f() {
                "hello"[0] = 5
            }
        "#],
        expect![[r#"
            src0.eta: parse(recovered)
            (program
              (uses)
              (defs
                (method (decl f (params) (returns))
                  (block
                    (error)))))
            program         : src0.eta:1:1..3:2
            method          : src0.eta:1:1..3:2
            mdecl           : src0.eta:1:1..1:4
            id f            : src0.eta:1:1..1:2
            block           : src0.eta:1:5..3:2

            Error: unexpected token `"hello"`
               ╭─[ src0.eta:2:5 ]
               │
             2 │     "hello"[0] = 5
               │     ───┬───  
               │        ╰───── expected one of "while", "if", "return", "_", "{", "}", or "identifier"
            ───╯
        "#]]
    }
}

#[test]
fn error_empty_eta() {
    parsetest! {
        ".eta",
        indoc![r#"
        "#],
        expect![[r#"
            src0.eta: parse(failed)
            Error: Program must contain at least one definition
               ╭─[ src0.eta:1:1 ]
               │
             1 │ 
               │ │ 
               │ ╰─ expected definition
            ───╯
        "#]]
    }
}

#[test]
fn error_empty_eti() {
    parsetest! {
        ".eti",
        indoc![r#"
        "#],
        expect![[r#"
            src0.eti: parse(failed)
            Error: Interface must contain at least one method declaration
               ╭─[ src0.eti:1:1 ]
               │
             1 │ 
               │ │ 
               │ ╰─ expected declaration
            ───╯
        "#]]
    }
}

#[test]
fn error_eta_nofunctions() {
    parsetest! {
        ".eta",
        indoc![r#"
            use io;
        "#],
        expect![[r#"
            src0.eta: parse(failed)
            Error: Program must contain at least one definition
               ╭─[ src0.eta:1:8 ]
               │
             1 │ use io;
               │        │ 
               │        ╰─ expected definition
            ───╯
        "#]]
    }
}

#[test]
fn error_eta_unfinished() {
    parsetest! {
        ".eta",
        indoc![r#"
            use io
 
            f() {
        "#],
        expect![[r#"
            src0.eta: parse(recovered)
            (program
              (uses
                (use io))
              (defs
                (method (decl f (params) (returns))
                  (error))))
            program         : src0.eta:1:1..3:6
            use             : src0.eta:1:1..1:7
            id io           : src0.eta:1:5..1:7
            method          : src0.eta:3:1..3:6
            mdecl           : src0.eta:3:1..3:4
            id f            : src0.eta:3:1..3:2

            Error: unexpected end of file
               ╭─[ src0.eta:3:6 ]
               │
             3 │ f() {
               │      │ 
               │      ╰─ expected one of "while", "if", "return", "_", "{", "}", or "identifier"
            ───╯
        "#]]
    }
}

#[test]
fn error_eti_unfinished() {
    parsetest! {
        ".eti",
        indoc![r#"
            f():
        "#],
        expect![[r#"
            src0.eti: parse(recovered)
            (interface
              (decl f (params) (returns (error))))
            interface       : src0.eti:1:1..1:5
            mdecl           : src0.eti:1:1..1:5
            id f            : src0.eti:1:1..1:2

            Error: unexpected end of file
               ╭─[ src0.eti:1:5 ]
               │
             1 │ f():
               │     │ 
               │     ╰─ expected one of "int", or "bool"
            ───╯
        "#]]
    }
}

#[test]
fn error_expr_as_stmt() {
    parsetest! {
        ".eta",
        indoc![r#"
            f() {
                1 + 2
            }
        "#],
        expect![[r#"
            src0.eta: parse(recovered)
            (program
              (uses)
              (defs
                (method (decl f (params) (returns))
                  (block
                    (error)))))
            program         : src0.eta:1:1..3:2
            method          : src0.eta:1:1..3:2
            mdecl           : src0.eta:1:1..1:4
            id f            : src0.eta:1:1..1:2
            block           : src0.eta:1:5..3:2

            Error: unexpected token `1`
               ╭─[ src0.eta:2:5 ]
               │
             2 │     1 + 2
               │     ┬  
               │     ╰── expected one of "while", "if", "return", "_", "{", "}", or "identifier"
            ───╯
        "#]]
    }
}

#[test]
fn error_if_start() {
    parsetest! {
        ".eta",
        indoc![r#"
            if true {}
        "#],
        expect![[r#"
            src0.eta: parse(recovered)
            (program
              (uses)
              (defs
                (error)))
            program         : src0.eta:1:1..1:11

            Error: unexpected token `if`
               ╭─[ src0.eta:1:1 ]
               │
             1 │ if true {}
               │ ─┬  
               │  ╰── expected one of "use", or "identifier"
            ───╯
        "#]]
    }
}

#[test]
fn error_length_as_id() {
    parsetest! {
        ".eta",
        indoc![r#"
            f() {
                length: int = 5
            }
        "#],
        expect![[r#"
            src0.eta: parse(recovered)
            (program
              (uses)
              (defs
                (method (decl f (params) (returns))
                  (block
                    (error)))))
            program         : src0.eta:1:1..3:2
            method          : src0.eta:1:1..3:2
            mdecl           : src0.eta:1:1..1:4
            id f            : src0.eta:1:1..1:2
            block           : src0.eta:1:5..3:2

            Error: unexpected token `length`
               ╭─[ src0.eta:2:5 ]
               │
             2 │     length: int = 5
               │     ───┬──  
               │        ╰──── expected one of "while", "if", "return", "_", "{", "}", or "identifier"
            ───╯
        "#]]
    }
}

#[test]
fn error_return_not_last() {
    parsetest! {
        ".eta",
        indoc![r#"
            f(): int {
                return 1
                x: int = 2
            }
        "#],
        expect![[r#"
            src0.eta: parse(recovered)
            (program
              (uses)
              (defs
                (method (decl f (params) (returns int))
                  (block
                    (error)
                    (assign (targets (: x int)) (values 2))))))
            program         : src0.eta:1:1..4:2
            method          : src0.eta:1:1..4:2
            mdecl           : src0.eta:1:1..1:9
            id f            : src0.eta:1:1..1:2
            block           : src0.eta:1:10..4:2
            assign          : src0.eta:3:5..3:15
            int_lit         : src0.eta:3:14..3:15
            decl            : src0.eta:3:5..3:11
            id x            : src0.eta:3:5..3:6

            Error: unexpected token `x`
               ╭─[ src0.eta:3:5 ]
               │
             3 │     x: int = 2
               │     ┬  
               │     ╰── expected one of ";", ",", or "}"
            ───╯
        "#]]
    }
}

#[test]
fn error_return_start() {
    parsetest! {
        ".eta",
        indoc![r#"
            return
        "#],
        expect![[r#"
            src0.eta: parse(recovered)
            (program
              (uses)
              (defs
                (error)))
            program         : src0.eta:1:1..1:7

            Error: unexpected token `return`
               ╭─[ src0.eta:1:1 ]
               │
             1 │ return
               │ ───┬──  
               │    ╰──── expected one of "use", or "identifier"
            ───╯
        "#]]
    }
}

#[test]
fn error_use_after_def() {
    parsetest! {
        ".eta",
        indoc![r#"
            f() {
            }
            use io
        "#],
        expect![[r#"
            src0.eta: parse(failed)
            Error: Use statements must go above all definitions
               ╭─[ src0.eta:3:1 ]
               │
             3 │ use io
               │ ───┬──  
               │    ╰──── Use not allowed here
               │ 
               │ Note: Try moving this use to the top of the file
            ───╯
        "#]]
    }
}

#[test]
fn error_while_start() {
    parsetest! {
        ".eta",
        indoc![r#"
            while true {}
        "#],
        expect![[r#"
            src0.eta: parse(recovered)
            (program
              (uses)
              (defs
                (error)))
            program         : src0.eta:1:1..1:14

            Error: unexpected token `while`
               ╭─[ src0.eta:1:1 ]
               │
             1 │ while true {}
               │ ──┬──  
               │   ╰──── expected one of "use", or "identifier"
            ───╯
        "#]]
    }
}

#[test]
fn ex1() {
    parsetest! {
        ".eta",
        indoc![r#"
            use io
 
            main(args: int[][]) {
              print("Hello, Worl\x{64}!\n")
              c3po: int = 'x' + 47;
              r2d2: int = c3po // No Han Solo
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses
                (use io))
              (defs
                (method (decl main (params (: args (array (array int)))) (returns))
                  (block
                    (call print Spur(5))
                    (assign (targets (: c3po int)) (values (+ x 47)))
                    (assign (targets (: r2d2 int)) (values c3po))))))
            program         : src0.eta:1:1..7:2
            use             : src0.eta:1:1..1:7
            id io           : src0.eta:1:5..1:7
            method          : src0.eta:3:1..7:2
            mdecl           : src0.eta:3:1..3:20
            id main         : src0.eta:3:1..3:5
            decl            : src0.eta:3:6..3:19
            arr_t           : src0.eta:3:17..3:19
            arr_t           : src0.eta:3:15..3:17
            id args         : src0.eta:3:6..3:10
            block           : src0.eta:3:21..7:2
            pcall           : src0.eta:4:3..4:32
            id print        : src0.eta:4:3..4:8
            string_lit      : src0.eta:4:9..4:31
            assign          : src0.eta:5:3..5:23
            bexpr           : src0.eta:5:15..5:23
            binop           : src0.eta:5:19..5:20
            char_lit        : src0.eta:5:15..5:18
            int_lit         : src0.eta:5:21..5:23
            decl            : src0.eta:5:3..5:12
            id c3po         : src0.eta:5:3..5:7
            assign          : src0.eta:6:3..6:19
            id c3po         : src0.eta:6:15..6:19
            decl            : src0.eta:6:3..6:12
            id r2d2         : src0.eta:6:3..6:7

        "#]]
    }
}

#[test]
fn ex2() {
    parsetest! {
        ".eta",
        indoc![r#"
            foo(): bool, int {
              expr: int = 1 - 2 * 3 * -4 *
              5pred: bool = true & true | false;
              if (expr <= 47) { }
              else pred = !pred
              if (pred) { expr = 59 }
              return pred, expr;
            }
 
            bar() {
              _, i: int = foo()
              b: int[i][]
              b[0] = {1, 0}
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl foo (params) (returns bool int))
                  (block
                    (assign (targets (: expr int)) (values (- 1 (* (* (* 2 3) (- 4)) 5))))
                    (assign (targets (: pred bool)) (values (| (& true true) false)))
                    (if (<= expr 47)
                      (block)
                      (assign (targets pred) (values (! pred))))
                    (if pred
                      (block
                        (assign (targets expr) (values 59))))
                    (return pred expr)))
                (method (decl bar (params) (returns))
                  (block
                    (assign (targets _ (: i int)) (values (call foo)))
                    (decls (: b (array (array int i))))
                    (assign (targets (index b 0)) (values (array 1 0)))))))
            program         : src0.eta:1:1..14:2
            method          : src0.eta:1:1..8:2
            mdecl           : src0.eta:1:1..1:17
            id foo          : src0.eta:1:1..1:4
            block           : src0.eta:1:18..8:2
            assign          : src0.eta:2:3..3:4
            bexpr           : src0.eta:2:15..3:4
            binop           : src0.eta:2:17..2:18
            int_lit         : src0.eta:2:15..2:16
            bexpr           : src0.eta:2:19..3:4
            binop           : src0.eta:2:30..2:31
            bexpr           : src0.eta:2:19..2:29
            binop           : src0.eta:2:25..2:26
            bexpr           : src0.eta:2:19..2:24
            binop           : src0.eta:2:21..2:22
            int_lit         : src0.eta:2:19..2:20
            int_lit         : src0.eta:2:23..2:24
            uexpr           : src0.eta:2:27..2:29
            uop             : src0.eta:2:27..2:28
            int_lit         : src0.eta:2:28..2:29
            int_lit         : src0.eta:3:3..3:4
            decl            : src0.eta:2:3..2:12
            id expr         : src0.eta:2:3..2:7
            assign          : src0.eta:3:4..3:36
            bexpr           : src0.eta:3:17..3:36
            binop           : src0.eta:3:29..3:30
            bexpr           : src0.eta:3:17..3:28
            binop           : src0.eta:3:22..3:23
            bool_lit        : src0.eta:3:17..3:21
            bool_lit        : src0.eta:3:24..3:28
            bool_lit        : src0.eta:3:31..3:36
            decl            : src0.eta:3:4..3:14
            id pred         : src0.eta:3:4..3:8
            if              : src0.eta:4:3..5:20
            bexpr           : src0.eta:4:7..4:17
            binop           : src0.eta:4:12..4:14
            id expr         : src0.eta:4:7..4:11
            int_lit         : src0.eta:4:15..4:17
            block           : src0.eta:4:19..4:22
            assign          : src0.eta:5:8..5:20
            uexpr           : src0.eta:5:15..5:20
            uop             : src0.eta:5:15..5:16
            id pred         : src0.eta:5:16..5:20
            id pred         : src0.eta:5:8..5:12
            if              : src0.eta:6:3..6:26
            id pred         : src0.eta:6:7..6:11
            block           : src0.eta:6:13..6:26
            assign          : src0.eta:6:15..6:24
            int_lit         : src0.eta:6:22..6:24
            id expr         : src0.eta:6:15..6:19
            return          : src0.eta:7:3..7:21
            id pred         : src0.eta:7:10..7:14
            id expr         : src0.eta:7:16..7:20
            method          : src0.eta:10:1..14:2
            mdecl           : src0.eta:10:1..10:6
            id bar          : src0.eta:10:1..10:4
            block           : src0.eta:10:7..14:2
            assign          : src0.eta:11:3..11:20
            pcall           : src0.eta:11:15..11:20
            id foo          : src0.eta:11:15..11:18
            decl            : src0.eta:11:6..11:12
            id i            : src0.eta:11:6..11:7
            decls           : src0.eta:12:3..12:14
            decl            : src0.eta:12:3..12:14
            arr_t           : src0.eta:12:12..12:14
            arr_t           : src0.eta:12:9..12:12
            id i            : src0.eta:12:10..12:11
            id b            : src0.eta:12:3..12:4
            assign          : src0.eta:13:3..13:16
            arr_lit         : src0.eta:13:10..13:16
            int_lit         : src0.eta:13:11..13:12
            int_lit         : src0.eta:13:14..13:15
            expr_idx        : src0.eta:13:3..13:7
            id b            : src0.eta:13:3..13:4
            int_lit         : src0.eta:13:5..13:6

        "#]]
    }
}

#[test]
fn ex3() {
    parsetest! {
        ".eta",
        indoc![r#"
            +------------------------------------+
            | What a beautiful, invalid program! |
            +------------------------------------+
        "#],
        expect![[r#"
            src0.eta: parse(recovered)
            (program
              (uses)
              (defs
                (error)
                (error)
                (error)
                (error)
                (error)
                (error)))
            program         : src0.eta:1:1..3:39

            Error: unexpected token `+`
               ╭─[ src0.eta:1:1 ]
               │
             1 │ +------------------------------------+
               │ ┬  
               │ ╰── expected one of "use", or "identifier"
            ───╯
            Error: unexpected token `a`
               ╭─[ src0.eta:2:8 ]
               │
             2 │ | What a beautiful, invalid program! |
               │        ┬  
               │        ╰── expected one of ":", or "("
            ───╯
            Error: unexpected token `beautiful`
               ╭─[ src0.eta:2:10 ]
               │
             2 │ | What a beautiful, invalid program! |
               │          ────┬────  
               │              ╰────── expected one of ":", or "("
            ───╯
            Error: unexpected token `,`
               ╭─[ src0.eta:2:19 ]
               │
             2 │ | What a beautiful, invalid program! |
               │                   ┬  
               │                   ╰── expected one of ":", or "("
            ───╯
            Error: unexpected token `program`
               ╭─[ src0.eta:2:29 ]
               │
             2 │ | What a beautiful, invalid program! |
               │                             ───┬───  
               │                                ╰───── expected one of ":", or "("
            ───╯
            Error: unexpected token `!`
               ╭─[ src0.eta:2:36 ]
               │
             2 │ | What a beautiful, invalid program! |
               │                                    ┬  
               │                                    ╰── expected one of ":", or "("
            ───╯
        "#]]
    }
}

#[test]
fn fancy_ids() {
    parsetest! {
        ".eta",
        indoc![r#"
            x'y(a_b: int, c'd: int): int {
                x'_1: int = a_b + c'd
                return x'_1
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl x'y (params (: a_b int) (: c'd int)) (returns int))
                  (block
                    (assign (targets (: x'_1 int)) (values (+ a_b c'd)))
                    (return x'_1)))))
            program         : src0.eta:1:1..4:2
            method          : src0.eta:1:1..4:2
            mdecl           : src0.eta:1:1..1:29
            id x'y          : src0.eta:1:1..1:4
            decl            : src0.eta:1:5..1:13
            id a_b          : src0.eta:1:5..1:8
            decl            : src0.eta:1:15..1:23
            id c'd          : src0.eta:1:15..1:18
            block           : src0.eta:1:30..4:2
            assign          : src0.eta:2:5..2:26
            bexpr           : src0.eta:2:17..2:26
            binop           : src0.eta:2:21..2:22
            id a_b          : src0.eta:2:17..2:20
            id c'd          : src0.eta:2:23..2:26
            decl            : src0.eta:2:5..2:14
            id x'_1         : src0.eta:2:5..2:9
            return          : src0.eta:3:5..3:16
            id x'_1         : src0.eta:3:12..3:16

        "#]]
    }
}

#[test]
fn full_precedence() {
    parsetest! {
        ".eta",
        indoc![r#"
            f(a: int, b: int, c: int, d: bool, e: bool): bool {
                return d | e & a + b * c < a - b / c == d != e
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl f (params (: a int) (: b int) (: c int) (: d bool) (: e bool)) (returns bool))
                  (block
                    (return (| d (& e (!= (== (< (+ a (* b c)) (- a (/ b c))) d) e))))))))
            program         : src0.eta:1:1..3:2
            method          : src0.eta:1:1..3:2
            mdecl           : src0.eta:1:1..1:50
            id f            : src0.eta:1:1..1:2
            decl            : src0.eta:1:3..1:9
            id a            : src0.eta:1:3..1:4
            decl            : src0.eta:1:11..1:17
            id b            : src0.eta:1:11..1:12
            decl            : src0.eta:1:19..1:25
            id c            : src0.eta:1:19..1:20
            decl            : src0.eta:1:27..1:34
            id d            : src0.eta:1:27..1:28
            decl            : src0.eta:1:36..1:43
            id e            : src0.eta:1:36..1:37
            block           : src0.eta:1:51..3:2
            return          : src0.eta:2:5..2:51
            bexpr           : src0.eta:2:12..2:51
            binop           : src0.eta:2:14..2:15
            id d            : src0.eta:2:12..2:13
            bexpr           : src0.eta:2:16..2:51
            binop           : src0.eta:2:18..2:19
            id e            : src0.eta:2:16..2:17
            bexpr           : src0.eta:2:20..2:51
            binop           : src0.eta:2:47..2:49
            bexpr           : src0.eta:2:20..2:46
            binop           : src0.eta:2:42..2:44
            bexpr           : src0.eta:2:20..2:41
            binop           : src0.eta:2:30..2:31
            bexpr           : src0.eta:2:20..2:29
            binop           : src0.eta:2:22..2:23
            id a            : src0.eta:2:20..2:21
            bexpr           : src0.eta:2:24..2:29
            binop           : src0.eta:2:26..2:27
            id b            : src0.eta:2:24..2:25
            id c            : src0.eta:2:28..2:29
            bexpr           : src0.eta:2:32..2:41
            binop           : src0.eta:2:34..2:35
            id a            : src0.eta:2:32..2:33
            bexpr           : src0.eta:2:36..2:41
            binop           : src0.eta:2:38..2:39
            id b            : src0.eta:2:36..2:37
            id c            : src0.eta:2:40..2:41
            id d            : src0.eta:2:45..2:46
            id e            : src0.eta:2:50..2:51

        "#]]
    }
}

#[test]
fn funccall_index_rhs() {
    parsetest! {
        ".eta",
        indoc![r#"
            f(): int {
                x: int = g()[0]
                y: int = g(1, 2)[h()]
                return x + y
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl f (params) (returns int))
                  (block
                    (assign (targets (: x int)) (values (index (call g) 0)))
                    (assign (targets (: y int)) (values (index (call g 1 2) (call h))))
                    (return (+ x y))))))
            program         : src0.eta:1:1..5:2
            method          : src0.eta:1:1..5:2
            mdecl           : src0.eta:1:1..1:9
            id f            : src0.eta:1:1..1:2
            block           : src0.eta:1:10..5:2
            assign          : src0.eta:2:5..2:20
            expr_idx        : src0.eta:2:14..2:20
            pcall           : src0.eta:2:14..2:17
            id g            : src0.eta:2:14..2:15
            int_lit         : src0.eta:2:18..2:19
            decl            : src0.eta:2:5..2:11
            id x            : src0.eta:2:5..2:6
            assign          : src0.eta:3:5..3:26
            expr_idx        : src0.eta:3:14..3:26
            pcall           : src0.eta:3:14..3:21
            id g            : src0.eta:3:14..3:15
            int_lit         : src0.eta:3:16..3:17
            int_lit         : src0.eta:3:19..3:20
            pcall           : src0.eta:3:22..3:25
            id h            : src0.eta:3:22..3:23
            decl            : src0.eta:3:5..3:11
            id y            : src0.eta:3:5..3:6
            return          : src0.eta:4:5..4:17
            bexpr           : src0.eta:4:12..4:17
            binop           : src0.eta:4:14..4:15
            id x            : src0.eta:4:12..4:13
            id y            : src0.eta:4:16..4:17

        "#]]
    }
}

#[test]
fn funccall_indexing() {
    parsetest! {
        ".eta",
        indoc![r#"
            main(args: int[][]) {
                g()[0] = 5
                g(arg)[x] = y
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl main (params (: args (array (array int)))) (returns))
                  (block
                    (assign (targets (index (call g) 0)) (values 5))
                    (assign (targets (index (call g arg) x)) (values y))))))
            program         : src0.eta:1:1..4:2
            method          : src0.eta:1:1..4:2
            mdecl           : src0.eta:1:1..1:20
            id main         : src0.eta:1:1..1:5
            decl            : src0.eta:1:6..1:19
            arr_t           : src0.eta:1:17..1:19
            arr_t           : src0.eta:1:15..1:17
            id args         : src0.eta:1:6..1:10
            block           : src0.eta:1:21..4:2
            assign          : src0.eta:2:5..2:15
            int_lit         : src0.eta:2:14..2:15
            expr_idx        : src0.eta:2:5..2:11
            pcall           : src0.eta:2:5..2:8
            id g            : src0.eta:2:5..2:6
            int_lit         : src0.eta:2:9..2:10
            assign          : src0.eta:3:5..3:18
            id y            : src0.eta:3:17..3:18
            expr_idx        : src0.eta:3:5..3:14
            pcall           : src0.eta:3:5..3:11
            id g            : src0.eta:3:5..3:6
            id arg          : src0.eta:3:7..3:10
            id x            : src0.eta:3:12..3:13

        "#]]
    }
}

#[test]
fn gcd() {
    parsetest! {
        ".eta",
        indoc![r#"
            // Return the greatest common divisor of two integers
            gcd(a:int, b:int):int {
              while (a != 0) {
                if (a<b) b = b - a
                else a = a - b
              }
              return b
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl gcd (params (: a int) (: b int)) (returns int))
                  (block
                    (while (!= a 0)
                      (block
                        (if (< a b)
                          (assign (targets b) (values (- b a)))
                          (assign (targets a) (values (- a b))))))
                    (return b)))))
            program         : src0.eta:2:1..8:2
            method          : src0.eta:2:1..8:2
            mdecl           : src0.eta:2:1..2:22
            id gcd          : src0.eta:2:1..2:4
            decl            : src0.eta:2:5..2:10
            id a            : src0.eta:2:5..2:6
            decl            : src0.eta:2:12..2:17
            id b            : src0.eta:2:12..2:13
            block           : src0.eta:2:23..8:2
            while           : src0.eta:3:3..6:4
            bexpr           : src0.eta:3:10..3:16
            binop           : src0.eta:3:12..3:14
            id a            : src0.eta:3:10..3:11
            int_lit         : src0.eta:3:15..3:16
            block           : src0.eta:3:18..6:4
            if              : src0.eta:4:5..5:19
            bexpr           : src0.eta:4:9..4:12
            binop           : src0.eta:4:10..4:11
            id a            : src0.eta:4:9..4:10
            id b            : src0.eta:4:11..4:12
            assign          : src0.eta:4:14..4:23
            bexpr           : src0.eta:4:18..4:23
            binop           : src0.eta:4:20..4:21
            id b            : src0.eta:4:18..4:19
            id a            : src0.eta:4:22..4:23
            id b            : src0.eta:4:14..4:15
            assign          : src0.eta:5:10..5:19
            bexpr           : src0.eta:5:14..5:19
            binop           : src0.eta:5:16..5:17
            id a            : src0.eta:5:14..5:15
            id b            : src0.eta:5:18..5:19
            id a            : src0.eta:5:10..5:11
            return          : src0.eta:7:3..7:11
            id b            : src0.eta:7:10..7:11

        "#]]
    }
}

#[test]
fn gd_assign01() {
    parsetest! {
        ".eta",
        indoc![r#"
            f(x:int, y:bool, z:int[]) {
              x = 0
              y = true
              x = x
              z = {1, 2, 3}
              x = g1()
              x = z[0]
              x = -1
              x = 0 + 0
              x = (0)
            }
 
            x: int;
 
            g1():int {
              return 0
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl f (params (: x int) (: y bool) (: z (array int))) (returns))
                  (block
                    (assign (targets x) (values 0))
                    (assign (targets y) (values true))
                    (assign (targets x) (values x))
                    (assign (targets z) (values (array 1 2 3)))
                    (assign (targets x) (values (call g1)))
                    (assign (targets x) (values (index z 0)))
                    (assign (targets x) (values (- 1)))
                    (assign (targets x) (values (+ 0 0)))
                    (assign (targets x) (values 0))))
                (global x int)
                (method (decl g1 (params) (returns int))
                  (block
                    (return 0)))))
            program         : src0.eta:1:1..17:2
            method          : src0.eta:1:1..11:2
            mdecl           : src0.eta:1:1..1:26
            id f            : src0.eta:1:1..1:2
            decl            : src0.eta:1:3..1:8
            id x            : src0.eta:1:3..1:4
            decl            : src0.eta:1:10..1:16
            id y            : src0.eta:1:10..1:11
            decl            : src0.eta:1:18..1:25
            arr_t           : src0.eta:1:23..1:25
            id z            : src0.eta:1:18..1:19
            block           : src0.eta:1:27..11:2
            assign          : src0.eta:2:3..2:8
            int_lit         : src0.eta:2:7..2:8
            id x            : src0.eta:2:3..2:4
            assign          : src0.eta:3:3..3:11
            bool_lit        : src0.eta:3:7..3:11
            id y            : src0.eta:3:3..3:4
            assign          : src0.eta:4:3..4:8
            id x            : src0.eta:4:7..4:8
            id x            : src0.eta:4:3..4:4
            assign          : src0.eta:5:3..5:16
            arr_lit         : src0.eta:5:7..5:16
            int_lit         : src0.eta:5:8..5:9
            int_lit         : src0.eta:5:11..5:12
            int_lit         : src0.eta:5:14..5:15
            id z            : src0.eta:5:3..5:4
            assign          : src0.eta:6:3..6:11
            pcall           : src0.eta:6:7..6:11
            id g1           : src0.eta:6:7..6:9
            id x            : src0.eta:6:3..6:4
            assign          : src0.eta:7:3..7:11
            expr_idx        : src0.eta:7:7..7:11
            id z            : src0.eta:7:7..7:8
            int_lit         : src0.eta:7:9..7:10
            id x            : src0.eta:7:3..7:4
            assign          : src0.eta:8:3..8:9
            uexpr           : src0.eta:8:7..8:9
            uop             : src0.eta:8:7..8:8
            int_lit         : src0.eta:8:8..8:9
            id x            : src0.eta:8:3..8:4
            assign          : src0.eta:9:3..9:12
            bexpr           : src0.eta:9:7..9:12
            binop           : src0.eta:9:9..9:10
            int_lit         : src0.eta:9:7..9:8
            int_lit         : src0.eta:9:11..9:12
            id x            : src0.eta:9:3..9:4
            assign          : src0.eta:10:3..10:10
            int_lit         : src0.eta:10:8..10:9
            id x            : src0.eta:10:3..10:4
            gdecl           : src0.eta:13:1..13:8
            id x            : src0.eta:13:1..13:2
            method          : src0.eta:15:1..17:2
            mdecl           : src0.eta:15:1..15:9
            id g1           : src0.eta:15:1..15:3
            block           : src0.eta:15:10..17:2
            return          : src0.eta:16:3..16:11
            int_lit         : src0.eta:16:10..16:11

        "#]]
    }
}

#[test]
fn gd_assign03() {
    parsetest! {
        ".eta",
        indoc![r#"
            f(x:int[], y:int) {
              x[0] = 0
              x[y] = 0
              x[{1,2,3}[0]] = 0
              x[g()] = 0
              x[x[0]] = 0
              x[-1] = 0
              x[0+0] = 0
              x[(0)] = 0
            }
 
            g():int {
              return 0
            }
            booleanboi :bool;
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl f (params (: x (array int)) (: y int)) (returns))
                  (block
                    (assign (targets (index x 0)) (values 0))
                    (assign (targets (index x y)) (values 0))
                    (assign (targets (index x (index (array 1 2 3) 0))) (values 0))
                    (assign (targets (index x (call g))) (values 0))
                    (assign (targets (index x (index x 0))) (values 0))
                    (assign (targets (index x (- 1))) (values 0))
                    (assign (targets (index x (+ 0 0))) (values 0))
                    (assign (targets (index x 0)) (values 0))))
                (method (decl g (params) (returns int))
                  (block
                    (return 0)))
                (global booleanboi bool)))
            program         : src0.eta:1:1..15:18
            method          : src0.eta:1:1..10:2
            mdecl           : src0.eta:1:1..1:18
            id f            : src0.eta:1:1..1:2
            decl            : src0.eta:1:3..1:10
            arr_t           : src0.eta:1:8..1:10
            id x            : src0.eta:1:3..1:4
            decl            : src0.eta:1:12..1:17
            id y            : src0.eta:1:12..1:13
            block           : src0.eta:1:19..10:2
            assign          : src0.eta:2:3..2:11
            int_lit         : src0.eta:2:10..2:11
            expr_idx        : src0.eta:2:3..2:7
            id x            : src0.eta:2:3..2:4
            int_lit         : src0.eta:2:5..2:6
            assign          : src0.eta:3:3..3:11
            int_lit         : src0.eta:3:10..3:11
            expr_idx        : src0.eta:3:3..3:7
            id x            : src0.eta:3:3..3:4
            id y            : src0.eta:3:5..3:6
            assign          : src0.eta:4:3..4:20
            int_lit         : src0.eta:4:19..4:20
            expr_idx        : src0.eta:4:3..4:16
            id x            : src0.eta:4:3..4:4
            expr_idx        : src0.eta:4:5..4:15
            arr_lit         : src0.eta:4:5..4:12
            int_lit         : src0.eta:4:6..4:7
            int_lit         : src0.eta:4:8..4:9
            int_lit         : src0.eta:4:10..4:11
            int_lit         : src0.eta:4:13..4:14
            assign          : src0.eta:5:3..5:13
            int_lit         : src0.eta:5:12..5:13
            expr_idx        : src0.eta:5:3..5:9
            id x            : src0.eta:5:3..5:4
            pcall           : src0.eta:5:5..5:8
            id g            : src0.eta:5:5..5:6
            assign          : src0.eta:6:3..6:14
            int_lit         : src0.eta:6:13..6:14
            expr_idx        : src0.eta:6:3..6:10
            id x            : src0.eta:6:3..6:4
            expr_idx        : src0.eta:6:5..6:9
            id x            : src0.eta:6:5..6:6
            int_lit         : src0.eta:6:7..6:8
            assign          : src0.eta:7:3..7:12
            int_lit         : src0.eta:7:11..7:12
            expr_idx        : src0.eta:7:3..7:8
            id x            : src0.eta:7:3..7:4
            uexpr           : src0.eta:7:5..7:7
            uop             : src0.eta:7:5..7:6
            int_lit         : src0.eta:7:6..7:7
            assign          : src0.eta:8:3..8:13
            int_lit         : src0.eta:8:12..8:13
            expr_idx        : src0.eta:8:3..8:9
            id x            : src0.eta:8:3..8:4
            bexpr           : src0.eta:8:5..8:8
            binop           : src0.eta:8:6..8:7
            int_lit         : src0.eta:8:5..8:6
            int_lit         : src0.eta:8:7..8:8
            assign          : src0.eta:9:3..9:13
            int_lit         : src0.eta:9:12..9:13
            expr_idx        : src0.eta:9:3..9:9
            id x            : src0.eta:9:3..9:4
            int_lit         : src0.eta:9:6..9:7
            method          : src0.eta:12:1..14:2
            mdecl           : src0.eta:12:1..12:8
            id g            : src0.eta:12:1..12:2
            block           : src0.eta:12:9..14:2
            return          : src0.eta:13:3..13:11
            int_lit         : src0.eta:13:10..13:11
            gdecl           : src0.eta:15:1..15:18
            id booleanboi   : src0.eta:15:1..15:11

        "#]]
    }
}

#[test]
fn gd_assign04() {
    parsetest! {
        ".eta",
        indoc![r#"
            f(x:int[][], y:int, z:int) {
              x[0][1] = 0
              x[y][z] = 0
              x[{1,2,3}[0]][{1,2,3}[1]] = 0
              x[g1()][g2()] = 0
              x[x[0][1]][x[2][3]] = 0
              x[-1][-2] = 0
              x[0+0][0-0] = 0
              x[(0)][(1)] = 0
            }
            heyitsaboolean :bool = false;
            g1():int {
              return 0
            }
 
            g2():int {
              return 0
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl f (params (: x (array (array int))) (: y int) (: z int)) (returns))
                  (block
                    (assign (targets (index (index x 0) 1)) (values 0))
                    (assign (targets (index (index x y) z)) (values 0))
                    (assign (targets (index (index x (index (array 1 2 3) 0)) (index (array 1 2 3) 1))) (values 0))
                    (assign (targets (index (index x (call g1)) (call g2))) (values 0))
                    (assign (targets (index (index x (index (index x 0) 1)) (index (index x 2) 3))) (values 0))
                    (assign (targets (index (index x (- 1)) (- 2))) (values 0))
                    (assign (targets (index (index x (+ 0 0)) (- 0 0))) (values 0))
                    (assign (targets (index (index x 0) 1)) (values 0))))
                (global heyitsaboolean bool false)
                (method (decl g1 (params) (returns int))
                  (block
                    (return 0)))
                (method (decl g2 (params) (returns int))
                  (block
                    (return 0)))))
            program         : src0.eta:1:1..18:2
            method          : src0.eta:1:1..10:2
            mdecl           : src0.eta:1:1..1:27
            id f            : src0.eta:1:1..1:2
            decl            : src0.eta:1:3..1:12
            arr_t           : src0.eta:1:10..1:12
            arr_t           : src0.eta:1:8..1:10
            id x            : src0.eta:1:3..1:4
            decl            : src0.eta:1:14..1:19
            id y            : src0.eta:1:14..1:15
            decl            : src0.eta:1:21..1:26
            id z            : src0.eta:1:21..1:22
            block           : src0.eta:1:28..10:2
            assign          : src0.eta:2:3..2:14
            int_lit         : src0.eta:2:13..2:14
            expr_idx        : src0.eta:2:3..2:10
            expr_idx        : src0.eta:2:3..2:7
            id x            : src0.eta:2:3..2:4
            int_lit         : src0.eta:2:5..2:6
            int_lit         : src0.eta:2:8..2:9
            assign          : src0.eta:3:3..3:14
            int_lit         : src0.eta:3:13..3:14
            expr_idx        : src0.eta:3:3..3:10
            expr_idx        : src0.eta:3:3..3:7
            id x            : src0.eta:3:3..3:4
            id y            : src0.eta:3:5..3:6
            id z            : src0.eta:3:8..3:9
            assign          : src0.eta:4:3..4:32
            int_lit         : src0.eta:4:31..4:32
            expr_idx        : src0.eta:4:3..4:28
            expr_idx        : src0.eta:4:3..4:16
            id x            : src0.eta:4:3..4:4
            expr_idx        : src0.eta:4:5..4:15
            arr_lit         : src0.eta:4:5..4:12
            int_lit         : src0.eta:4:6..4:7
            int_lit         : src0.eta:4:8..4:9
            int_lit         : src0.eta:4:10..4:11
            int_lit         : src0.eta:4:13..4:14
            expr_idx        : src0.eta:4:17..4:27
            arr_lit         : src0.eta:4:17..4:24
            int_lit         : src0.eta:4:18..4:19
            int_lit         : src0.eta:4:20..4:21
            int_lit         : src0.eta:4:22..4:23
            int_lit         : src0.eta:4:25..4:26
            assign          : src0.eta:5:3..5:20
            int_lit         : src0.eta:5:19..5:20
            expr_idx        : src0.eta:5:3..5:16
            expr_idx        : src0.eta:5:3..5:10
            id x            : src0.eta:5:3..5:4
            pcall           : src0.eta:5:5..5:9
            id g1           : src0.eta:5:5..5:7
            pcall           : src0.eta:5:11..5:15
            id g2           : src0.eta:5:11..5:13
            assign          : src0.eta:6:3..6:26
            int_lit         : src0.eta:6:25..6:26
            expr_idx        : src0.eta:6:3..6:22
            expr_idx        : src0.eta:6:3..6:13
            id x            : src0.eta:6:3..6:4
            expr_idx        : src0.eta:6:5..6:12
            expr_idx        : src0.eta:6:5..6:9
            id x            : src0.eta:6:5..6:6
            int_lit         : src0.eta:6:7..6:8
            int_lit         : src0.eta:6:10..6:11
            expr_idx        : src0.eta:6:14..6:21
            expr_idx        : src0.eta:6:14..6:18
            id x            : src0.eta:6:14..6:15
            int_lit         : src0.eta:6:16..6:17
            int_lit         : src0.eta:6:19..6:20
            assign          : src0.eta:7:3..7:16
            int_lit         : src0.eta:7:15..7:16
            expr_idx        : src0.eta:7:3..7:12
            expr_idx        : src0.eta:7:3..7:8
            id x            : src0.eta:7:3..7:4
            uexpr           : src0.eta:7:5..7:7
            uop             : src0.eta:7:5..7:6
            int_lit         : src0.eta:7:6..7:7
            uexpr           : src0.eta:7:9..7:11
            uop             : src0.eta:7:9..7:10
            int_lit         : src0.eta:7:10..7:11
            assign          : src0.eta:8:3..8:18
            int_lit         : src0.eta:8:17..8:18
            expr_idx        : src0.eta:8:3..8:14
            expr_idx        : src0.eta:8:3..8:9
            id x            : src0.eta:8:3..8:4
            bexpr           : src0.eta:8:5..8:8
            binop           : src0.eta:8:6..8:7
            int_lit         : src0.eta:8:5..8:6
            int_lit         : src0.eta:8:7..8:8
            bexpr           : src0.eta:8:10..8:13
            binop           : src0.eta:8:11..8:12
            int_lit         : src0.eta:8:10..8:11
            int_lit         : src0.eta:8:12..8:13
            assign          : src0.eta:9:3..9:18
            int_lit         : src0.eta:9:17..9:18
            expr_idx        : src0.eta:9:3..9:14
            expr_idx        : src0.eta:9:3..9:9
            id x            : src0.eta:9:3..9:4
            int_lit         : src0.eta:9:6..9:7
            int_lit         : src0.eta:9:11..9:12
            gdecl           : src0.eta:11:1..11:30
            id heyitsaboolean: src0.eta:11:1..11:15
            bool_lit        : src0.eta:11:24..11:29
            method          : src0.eta:12:1..14:2
            mdecl           : src0.eta:12:1..12:9
            id g1           : src0.eta:12:1..12:3
            block           : src0.eta:12:10..14:2
            return          : src0.eta:13:3..13:11
            int_lit         : src0.eta:13:10..13:11
            method          : src0.eta:16:1..18:2
            mdecl           : src0.eta:16:1..16:9
            id g2           : src0.eta:16:1..16:3
            block           : src0.eta:16:10..18:2
            return          : src0.eta:17:3..17:11
            int_lit         : src0.eta:17:10..17:11

        "#]]
    }
}

#[test]
fn gd_block01() {
    parsetest! {
        ".eta",
        indoc![r#"
            f1() {
              x:int
            }
            thebigglobarray :int[];
            f2() {
              x:int;
            }
 
            f3() {
              x01:int;
              x02:int
              x03:int;
              x04:int
              x05:int
              x06:int;
              x07:int
              x08:int
              x09:int
              x10:int;
              x11:int;
              x12:int
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl f1 (params) (returns))
                  (block
                    (decls (: x int))))
                (global thebigglobarray (array int))
                (method (decl f2 (params) (returns))
                  (block
                    (decls (: x int))))
                (method (decl f3 (params) (returns))
                  (block
                    (decls (: x01 int))
                    (decls (: x02 int))
                    (decls (: x03 int))
                    (decls (: x04 int))
                    (decls (: x05 int))
                    (decls (: x06 int))
                    (decls (: x07 int))
                    (decls (: x08 int))
                    (decls (: x09 int))
                    (decls (: x10 int))
                    (decls (: x11 int))
                    (decls (: x12 int))))))
            program         : src0.eta:1:1..22:2
            method          : src0.eta:1:1..3:2
            mdecl           : src0.eta:1:1..1:5
            id f1           : src0.eta:1:1..1:3
            block           : src0.eta:1:6..3:2
            decls           : src0.eta:2:3..2:8
            decl            : src0.eta:2:3..2:8
            id x            : src0.eta:2:3..2:4
            gdecl           : src0.eta:4:1..4:24
            arr_t           : src0.eta:4:21..4:23
            id thebigglobarray: src0.eta:4:1..4:16
            method          : src0.eta:5:1..7:2
            mdecl           : src0.eta:5:1..5:5
            id f2           : src0.eta:5:1..5:3
            block           : src0.eta:5:6..7:2
            decls           : src0.eta:6:3..6:8
            decl            : src0.eta:6:3..6:8
            id x            : src0.eta:6:3..6:4
            method          : src0.eta:9:1..22:2
            mdecl           : src0.eta:9:1..9:5
            id f3           : src0.eta:9:1..9:3
            block           : src0.eta:9:6..22:2
            decls           : src0.eta:10:3..10:10
            decl            : src0.eta:10:3..10:10
            id x01          : src0.eta:10:3..10:6
            decls           : src0.eta:11:3..11:10
            decl            : src0.eta:11:3..11:10
            id x02          : src0.eta:11:3..11:6
            decls           : src0.eta:12:3..12:10
            decl            : src0.eta:12:3..12:10
            id x03          : src0.eta:12:3..12:6
            decls           : src0.eta:13:3..13:10
            decl            : src0.eta:13:3..13:10
            id x04          : src0.eta:13:3..13:6
            decls           : src0.eta:14:3..14:10
            decl            : src0.eta:14:3..14:10
            id x05          : src0.eta:14:3..14:6
            decls           : src0.eta:15:3..15:10
            decl            : src0.eta:15:3..15:10
            id x06          : src0.eta:15:3..15:6
            decls           : src0.eta:16:3..16:10
            decl            : src0.eta:16:3..16:10
            id x07          : src0.eta:16:3..16:6
            decls           : src0.eta:17:3..17:10
            decl            : src0.eta:17:3..17:10
            id x08          : src0.eta:17:3..17:6
            decls           : src0.eta:18:3..18:10
            decl            : src0.eta:18:3..18:10
            id x09          : src0.eta:18:3..18:6
            decls           : src0.eta:19:3..19:10
            decl            : src0.eta:19:3..19:10
            id x10          : src0.eta:19:3..19:6
            decls           : src0.eta:20:3..20:10
            decl            : src0.eta:20:3..20:10
            id x11          : src0.eta:20:3..20:6
            decls           : src0.eta:21:3..21:10
            decl            : src0.eta:21:3..21:10
            id x12          : src0.eta:21:3..21:6

        "#]]
    }
}

#[test]
fn gd_block02() {
    parsetest! {
        ".eta",
        indoc![r#"
            f1(): int {
              return 0;
            }
 
            f2(): int {
              x:int
              return 0
            }
            arrayisquitedeep :int[][][][][][];
            f3(): int {
              x:int
              return 0;
            }
 
            f4(): int {
              x:int;
              return 0
            }
 
            f5(): int {
              x:int;
              return 0;
            }
 
            f6(): int {
              x01:int;
              x02:int
              x03:int;
              x04:int
              x05:int
              x06:int;
              x07:int
              x08:int
              x09:int
              x10:int;
              x11:int;
              x12:int
              return 0
            }
 
            f7(): int {
              x01:int;
              x02:int
              x03:int;
              x04:int
              x05:int
              x06:int;
              x07:int
              x08:int
              x09:int
              x10:int;
              x11:int;
              x12:int
              return 0;
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl f1 (params) (returns int))
                  (block
                    (return 0)))
                (method (decl f2 (params) (returns int))
                  (block
                    (decls (: x int))
                    (return 0)))
                (global arrayisquitedeep (array (array (array (array (array (array int)))))))
                (method (decl f3 (params) (returns int))
                  (block
                    (decls (: x int))
                    (return 0)))
                (method (decl f4 (params) (returns int))
                  (block
                    (decls (: x int))
                    (return 0)))
                (method (decl f5 (params) (returns int))
                  (block
                    (decls (: x int))
                    (return 0)))
                (method (decl f6 (params) (returns int))
                  (block
                    (decls (: x01 int))
                    (decls (: x02 int))
                    (decls (: x03 int))
                    (decls (: x04 int))
                    (decls (: x05 int))
                    (decls (: x06 int))
                    (decls (: x07 int))
                    (decls (: x08 int))
                    (decls (: x09 int))
                    (decls (: x10 int))
                    (decls (: x11 int))
                    (decls (: x12 int))
                    (return 0)))
                (method (decl f7 (params) (returns int))
                  (block
                    (decls (: x01 int))
                    (decls (: x02 int))
                    (decls (: x03 int))
                    (decls (: x04 int))
                    (decls (: x05 int))
                    (decls (: x06 int))
                    (decls (: x07 int))
                    (decls (: x08 int))
                    (decls (: x09 int))
                    (decls (: x10 int))
                    (decls (: x11 int))
                    (decls (: x12 int))
                    (return 0)))))
            program         : src0.eta:1:1..55:2
            method          : src0.eta:1:1..3:2
            mdecl           : src0.eta:1:1..1:10
            id f1           : src0.eta:1:1..1:3
            block           : src0.eta:1:11..3:2
            return          : src0.eta:2:3..2:12
            int_lit         : src0.eta:2:10..2:11
            method          : src0.eta:5:1..8:2
            mdecl           : src0.eta:5:1..5:10
            id f2           : src0.eta:5:1..5:3
            block           : src0.eta:5:11..8:2
            decls           : src0.eta:6:3..6:8
            decl            : src0.eta:6:3..6:8
            id x            : src0.eta:6:3..6:4
            return          : src0.eta:7:3..7:11
            int_lit         : src0.eta:7:10..7:11
            gdecl           : src0.eta:9:1..9:35
            arr_t           : src0.eta:9:32..9:34
            arr_t           : src0.eta:9:30..9:32
            arr_t           : src0.eta:9:28..9:30
            arr_t           : src0.eta:9:26..9:28
            arr_t           : src0.eta:9:24..9:26
            arr_t           : src0.eta:9:22..9:24
            id arrayisquitedeep: src0.eta:9:1..9:17
            method          : src0.eta:10:1..13:2
            mdecl           : src0.eta:10:1..10:10
            id f3           : src0.eta:10:1..10:3
            block           : src0.eta:10:11..13:2
            decls           : src0.eta:11:3..11:8
            decl            : src0.eta:11:3..11:8
            id x            : src0.eta:11:3..11:4
            return          : src0.eta:12:3..12:12
            int_lit         : src0.eta:12:10..12:11
            method          : src0.eta:15:1..18:2
            mdecl           : src0.eta:15:1..15:10
            id f4           : src0.eta:15:1..15:3
            block           : src0.eta:15:11..18:2
            decls           : src0.eta:16:3..16:8
            decl            : src0.eta:16:3..16:8
            id x            : src0.eta:16:3..16:4
            return          : src0.eta:17:3..17:11
            int_lit         : src0.eta:17:10..17:11
            method          : src0.eta:20:1..23:2
            mdecl           : src0.eta:20:1..20:10
            id f5           : src0.eta:20:1..20:3
            block           : src0.eta:20:11..23:2
            decls           : src0.eta:21:3..21:8
            decl            : src0.eta:21:3..21:8
            id x            : src0.eta:21:3..21:4
            return          : src0.eta:22:3..22:12
            int_lit         : src0.eta:22:10..22:11
            method          : src0.eta:25:1..39:2
            mdecl           : src0.eta:25:1..25:10
            id f6           : src0.eta:25:1..25:3
            block           : src0.eta:25:11..39:2
            decls           : src0.eta:26:3..26:10
            decl            : src0.eta:26:3..26:10
            id x01          : src0.eta:26:3..26:6
            decls           : src0.eta:27:3..27:10
            decl            : src0.eta:27:3..27:10
            id x02          : src0.eta:27:3..27:6
            decls           : src0.eta:28:3..28:10
            decl            : src0.eta:28:3..28:10
            id x03          : src0.eta:28:3..28:6
            decls           : src0.eta:29:3..29:10
            decl            : src0.eta:29:3..29:10
            id x04          : src0.eta:29:3..29:6
            decls           : src0.eta:30:3..30:10
            decl            : src0.eta:30:3..30:10
            id x05          : src0.eta:30:3..30:6
            decls           : src0.eta:31:3..31:10
            decl            : src0.eta:31:3..31:10
            id x06          : src0.eta:31:3..31:6
            decls           : src0.eta:32:3..32:10
            decl            : src0.eta:32:3..32:10
            id x07          : src0.eta:32:3..32:6
            decls           : src0.eta:33:3..33:10
            decl            : src0.eta:33:3..33:10
            id x08          : src0.eta:33:3..33:6
            decls           : src0.eta:34:3..34:10
            decl            : src0.eta:34:3..34:10
            id x09          : src0.eta:34:3..34:6
            decls           : src0.eta:35:3..35:10
            decl            : src0.eta:35:3..35:10
            id x10          : src0.eta:35:3..35:6
            decls           : src0.eta:36:3..36:10
            decl            : src0.eta:36:3..36:10
            id x11          : src0.eta:36:3..36:6
            decls           : src0.eta:37:3..37:10
            decl            : src0.eta:37:3..37:10
            id x12          : src0.eta:37:3..37:6
            return          : src0.eta:38:3..38:11
            int_lit         : src0.eta:38:10..38:11
            method          : src0.eta:41:1..55:2
            mdecl           : src0.eta:41:1..41:10
            id f7           : src0.eta:41:1..41:3
            block           : src0.eta:41:11..55:2
            decls           : src0.eta:42:3..42:10
            decl            : src0.eta:42:3..42:10
            id x01          : src0.eta:42:3..42:6
            decls           : src0.eta:43:3..43:10
            decl            : src0.eta:43:3..43:10
            id x02          : src0.eta:43:3..43:6
            decls           : src0.eta:44:3..44:10
            decl            : src0.eta:44:3..44:10
            id x03          : src0.eta:44:3..44:6
            decls           : src0.eta:45:3..45:10
            decl            : src0.eta:45:3..45:10
            id x04          : src0.eta:45:3..45:6
            decls           : src0.eta:46:3..46:10
            decl            : src0.eta:46:3..46:10
            id x05          : src0.eta:46:3..46:6
            decls           : src0.eta:47:3..47:10
            decl            : src0.eta:47:3..47:10
            id x06          : src0.eta:47:3..47:6
            decls           : src0.eta:48:3..48:10
            decl            : src0.eta:48:3..48:10
            id x07          : src0.eta:48:3..48:6
            decls           : src0.eta:49:3..49:10
            decl            : src0.eta:49:3..49:10
            id x08          : src0.eta:49:3..49:6
            decls           : src0.eta:50:3..50:10
            decl            : src0.eta:50:3..50:10
            id x09          : src0.eta:50:3..50:6
            decls           : src0.eta:51:3..51:10
            decl            : src0.eta:51:3..51:10
            id x10          : src0.eta:51:3..51:6
            decls           : src0.eta:52:3..52:10
            decl            : src0.eta:52:3..52:10
            id x11          : src0.eta:52:3..52:6
            decls           : src0.eta:53:3..53:10
            decl            : src0.eta:53:3..53:10
            id x12          : src0.eta:53:3..53:6
            return          : src0.eta:54:3..54:12
            int_lit         : src0.eta:54:10..54:11

        "#]]
    }
}

#[test]
fn gd_globlarrinit() {
    parsetest! {
        ".eta",
        indoc![r#"
            x :int[] = "whoops";
 
            foo(x :int) :int
            {
               return x;
            }
        "#],
        expect![[r#"
            src0.eta: parse(failed)
            Error: global arrays can't be initialized at the point of declaration
               ╭─[ src0.eta:1:10 ]
               │
             1 │ x :int[] = "whoops";
               │          ─────┬────  
               │               ╰────── initialization happens here
            ───╯
        "#]]
    }
}

#[test]
fn gd_globlarrinit1() {
    parsetest! {
        ".eta",
        indoc![r#"
            x :int[4];
 
            foo(x :int) :int
            {
               return x;
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (global x (array int 4))
                (method (decl foo (params (: x int)) (returns int))
                  (block
                    (return x)))))
            program         : src0.eta:1:1..6:2
            gdecl           : src0.eta:1:1..1:11
            arr_t           : src0.eta:1:7..1:10
            int_lit         : src0.eta:1:8..1:9
            id x            : src0.eta:1:1..1:2
            method          : src0.eta:3:1..6:2
            mdecl           : src0.eta:3:1..3:17
            id foo          : src0.eta:3:1..3:4
            decl            : src0.eta:3:5..3:11
            id x            : src0.eta:3:5..3:6
            block           : src0.eta:4:1..6:2
            return          : src0.eta:5:4..5:13
            id x            : src0.eta:5:11..5:12

        "#]]
    }
}

#[test]
fn gd_globlarrinit2() {
    parsetest! {
        ".eta",
        indoc![r#"
            x :bool[] = {true, false,};
 
            foo(x :int) :int
            {
               return x;
            }
        "#],
        expect![[r#"
            src0.eta: parse(failed)
            Error: global arrays can't be initialized at the point of declaration
               ╭─[ src0.eta:1:11 ]
               │
             1 │ x :bool[] = {true, false,};
               │           ────────┬───────  
               │                   ╰───────── initialization happens here
            ───╯
        "#]]
    }
}

#[test]
fn global_vars() {
    parsetest! {
        ".eta",
        indoc![r#"
            count: int = 42
            flag: bool = true
            data: int[]
            n: int
 
            flag2: bool = true;
            m: int;
 
            reset() {
                count = 0
                flag = false
            }
 
            count: int = 42
            flag: bool = true
            data: int[]
            n: int
 
            flag2: bool = true;
            m: int;
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (global count int 42)
                (global flag bool true)
                (global data (array int))
                (global n int)
                (global flag2 bool true)
                (global m int)
                (method (decl reset (params) (returns))
                  (block
                    (assign (targets count) (values 0))
                    (assign (targets flag) (values false))))
                (global count int 42)
                (global flag bool true)
                (global data (array int))
                (global n int)
                (global flag2 bool true)
                (global m int)))
            program         : src0.eta:1:1..20:8
            gdecl           : src0.eta:1:1..1:16
            id count        : src0.eta:1:1..1:6
            int_lit         : src0.eta:1:14..1:16
            gdecl           : src0.eta:2:1..2:18
            id flag         : src0.eta:2:1..2:5
            bool_lit        : src0.eta:2:14..2:18
            gdecl           : src0.eta:3:1..3:12
            arr_t           : src0.eta:3:10..3:12
            id data         : src0.eta:3:1..3:5
            gdecl           : src0.eta:4:1..4:7
            id n            : src0.eta:4:1..4:2
            gdecl           : src0.eta:6:1..6:20
            id flag2        : src0.eta:6:1..6:6
            bool_lit        : src0.eta:6:15..6:19
            gdecl           : src0.eta:7:1..7:8
            id m            : src0.eta:7:1..7:2
            method          : src0.eta:9:1..12:2
            mdecl           : src0.eta:9:1..9:8
            id reset        : src0.eta:9:1..9:6
            block           : src0.eta:9:9..12:2
            assign          : src0.eta:10:5..10:14
            int_lit         : src0.eta:10:13..10:14
            id count        : src0.eta:10:5..10:10
            assign          : src0.eta:11:5..11:17
            bool_lit        : src0.eta:11:12..11:17
            id flag         : src0.eta:11:5..11:9
            gdecl           : src0.eta:14:1..14:16
            id count        : src0.eta:14:1..14:6
            int_lit         : src0.eta:14:14..14:16
            gdecl           : src0.eta:15:1..15:18
            id flag         : src0.eta:15:1..15:5
            bool_lit        : src0.eta:15:14..15:18
            gdecl           : src0.eta:16:1..16:12
            arr_t           : src0.eta:16:10..16:12
            id data         : src0.eta:16:1..16:5
            gdecl           : src0.eta:17:1..17:7
            id n            : src0.eta:17:1..17:2
            gdecl           : src0.eta:19:1..19:20
            id flag2        : src0.eta:19:1..19:6
            bool_lit        : src0.eta:19:15..19:19
            gdecl           : src0.eta:20:1..20:8
            id m            : src0.eta:20:1..20:2

        "#]]
    }
}

#[test]
fn globals_interleaved() {
    parsetest! {
        ".eta",
        indoc![r#"
            x: int = 1
            f() {}
            y: int = 2
            g() {}
            z: bool = true
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (global x int 1)
                (method (decl f (params) (returns))
                  (block))
                (global y int 2)
                (method (decl g (params) (returns))
                  (block))
                (global z bool true)))
            program         : src0.eta:1:1..5:15
            gdecl           : src0.eta:1:1..1:11
            id x            : src0.eta:1:1..1:2
            int_lit         : src0.eta:1:10..1:11
            method          : src0.eta:2:1..2:7
            mdecl           : src0.eta:2:1..2:4
            id f            : src0.eta:2:1..2:2
            block           : src0.eta:2:5..2:7
            gdecl           : src0.eta:3:1..3:11
            id y            : src0.eta:3:1..3:2
            int_lit         : src0.eta:3:10..3:11
            method          : src0.eta:4:1..4:7
            mdecl           : src0.eta:4:1..4:4
            id g            : src0.eta:4:1..4:2
            block           : src0.eta:4:5..4:7
            gdecl           : src0.eta:5:1..5:15
            id z            : src0.eta:5:1..5:2
            bool_lit        : src0.eta:5:11..5:15

        "#]]
    }
}

#[test]
fn high_mult_mod() {
    parsetest! {
        ".eta",
        indoc![r#"
            ops(a: int, b: int): int, int {
                h: int = a *>> b
                m: int = a % b
                return h, m
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl ops (params (: a int) (: b int)) (returns int int))
                  (block
                    (assign (targets (: h int)) (values (*>> a b)))
                    (assign (targets (: m int)) (values (% a b)))
                    (return h m)))))
            program         : src0.eta:1:1..5:2
            method          : src0.eta:1:1..5:2
            mdecl           : src0.eta:1:1..1:30
            id ops          : src0.eta:1:1..1:4
            decl            : src0.eta:1:5..1:11
            id a            : src0.eta:1:5..1:6
            decl            : src0.eta:1:13..1:19
            id b            : src0.eta:1:13..1:14
            block           : src0.eta:1:31..5:2
            assign          : src0.eta:2:5..2:21
            bexpr           : src0.eta:2:14..2:21
            binop           : src0.eta:2:16..2:19
            id a            : src0.eta:2:14..2:15
            id b            : src0.eta:2:20..2:21
            decl            : src0.eta:2:5..2:11
            id h            : src0.eta:2:5..2:6
            assign          : src0.eta:3:5..3:19
            bexpr           : src0.eta:3:14..3:19
            binop           : src0.eta:3:16..3:17
            id a            : src0.eta:3:14..3:15
            id b            : src0.eta:3:18..3:19
            decl            : src0.eta:3:5..3:11
            id m            : src0.eta:3:5..3:6
            return          : src0.eta:4:5..4:16
            id h            : src0.eta:4:12..4:13
            id m            : src0.eta:4:15..4:16

        "#]]
    }
}

#[test]
fn if_no_parens() {
    parsetest! {
        ".eta",
        indoc![r#"
            f(x: int) {
                if x > 0 x = x - 1
                if x == 0 x = 0
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl f (params (: x int)) (returns))
                  (block
                    (if (> x 0)
                      (assign (targets x) (values (- x 1))))
                    (if (== x 0)
                      (assign (targets x) (values 0)))))))
            program         : src0.eta:1:1..4:2
            method          : src0.eta:1:1..4:2
            mdecl           : src0.eta:1:1..1:10
            id f            : src0.eta:1:1..1:2
            decl            : src0.eta:1:3..1:9
            id x            : src0.eta:1:3..1:4
            block           : src0.eta:1:11..4:2
            if              : src0.eta:2:5..2:23
            bexpr           : src0.eta:2:8..2:13
            binop           : src0.eta:2:10..2:11
            id x            : src0.eta:2:8..2:9
            int_lit         : src0.eta:2:12..2:13
            assign          : src0.eta:2:14..2:23
            bexpr           : src0.eta:2:18..2:23
            binop           : src0.eta:2:20..2:21
            id x            : src0.eta:2:18..2:19
            int_lit         : src0.eta:2:22..2:23
            id x            : src0.eta:2:14..2:15
            if              : src0.eta:3:5..3:20
            bexpr           : src0.eta:3:8..3:14
            binop           : src0.eta:3:10..3:12
            id x            : src0.eta:3:8..3:9
            int_lit         : src0.eta:3:13..3:14
            assign          : src0.eta:3:15..3:20
            int_lit         : src0.eta:3:19..3:20
            id x            : src0.eta:3:15..3:16

        "#]]
    }
}

#[test]
fn insertionsort() {
    parsetest! {
        ".eta",
        indoc![r#"
            sort(a: int[]) {
              i:int = 0
              n:int = length(a)
              while (i < n) {
                  j:int = i
                  while (j > 0) {
                    if (a[j-1] > a[j]) {
                        swap:int = a[j]
                        a[j] = a[j-1]
                        a[j-1] = swap
                    }
                    j = j-1
                  }
                  i = i+1
              }
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl sort (params (: a (array int))) (returns))
                  (block
                    (assign (targets (: i int)) (values 0))
                    (assign (targets (: n int)) (values (length a)))
                    (while (< i n)
                      (block
                        (assign (targets (: j int)) (values i))
                        (while (> j 0)
                          (block
                            (if (> (index a (- j 1)) (index a j))
                              (block
                                (assign (targets (: swap int)) (values (index a j)))
                                (assign (targets (index a j)) (values (index a (- j 1))))
                                (assign (targets (index a (- j 1))) (values swap))))
                            (assign (targets j) (values (- j 1)))))
                        (assign (targets i) (values (+ i 1)))))))))
            program         : src0.eta:1:1..16:2
            method          : src0.eta:1:1..16:2
            mdecl           : src0.eta:1:1..1:15
            id sort         : src0.eta:1:1..1:5
            decl            : src0.eta:1:6..1:14
            arr_t           : src0.eta:1:12..1:14
            id a            : src0.eta:1:6..1:7
            block           : src0.eta:1:16..16:2
            assign          : src0.eta:2:3..2:12
            int_lit         : src0.eta:2:11..2:12
            decl            : src0.eta:2:3..2:8
            id i            : src0.eta:2:3..2:4
            assign          : src0.eta:3:3..3:20
            length          : src0.eta:3:11..3:20
            id a            : src0.eta:3:18..3:19
            decl            : src0.eta:3:3..3:8
            id n            : src0.eta:3:3..3:4
            while           : src0.eta:4:3..15:4
            bexpr           : src0.eta:4:10..4:15
            binop           : src0.eta:4:12..4:13
            id i            : src0.eta:4:10..4:11
            id n            : src0.eta:4:14..4:15
            block           : src0.eta:4:17..15:4
            assign          : src0.eta:5:7..5:16
            id i            : src0.eta:5:15..5:16
            decl            : src0.eta:5:7..5:12
            id j            : src0.eta:5:7..5:8
            while           : src0.eta:6:7..13:8
            bexpr           : src0.eta:6:14..6:19
            binop           : src0.eta:6:16..6:17
            id j            : src0.eta:6:14..6:15
            int_lit         : src0.eta:6:18..6:19
            block           : src0.eta:6:21..13:8
            if              : src0.eta:7:9..11:10
            bexpr           : src0.eta:7:13..7:26
            binop           : src0.eta:7:20..7:21
            expr_idx        : src0.eta:7:13..7:19
            id a            : src0.eta:7:13..7:14
            bexpr           : src0.eta:7:15..7:18
            binop           : src0.eta:7:16..7:17
            id j            : src0.eta:7:15..7:16
            int_lit         : src0.eta:7:17..7:18
            expr_idx        : src0.eta:7:22..7:26
            id a            : src0.eta:7:22..7:23
            id j            : src0.eta:7:24..7:25
            block           : src0.eta:7:28..11:10
            assign          : src0.eta:8:13..8:28
            expr_idx        : src0.eta:8:24..8:28
            id a            : src0.eta:8:24..8:25
            id j            : src0.eta:8:26..8:27
            decl            : src0.eta:8:13..8:21
            id swap         : src0.eta:8:13..8:17
            assign          : src0.eta:9:13..9:26
            expr_idx        : src0.eta:9:20..9:26
            id a            : src0.eta:9:20..9:21
            bexpr           : src0.eta:9:22..9:25
            binop           : src0.eta:9:23..9:24
            id j            : src0.eta:9:22..9:23
            int_lit         : src0.eta:9:24..9:25
            expr_idx        : src0.eta:9:13..9:17
            id a            : src0.eta:9:13..9:14
            id j            : src0.eta:9:15..9:16
            assign          : src0.eta:10:13..10:26
            id swap         : src0.eta:10:22..10:26
            expr_idx        : src0.eta:10:13..10:19
            id a            : src0.eta:10:13..10:14
            bexpr           : src0.eta:10:15..10:18
            binop           : src0.eta:10:16..10:17
            id j            : src0.eta:10:15..10:16
            int_lit         : src0.eta:10:17..10:18
            assign          : src0.eta:12:9..12:16
            bexpr           : src0.eta:12:13..12:16
            binop           : src0.eta:12:14..12:15
            id j            : src0.eta:12:13..12:14
            int_lit         : src0.eta:12:15..12:16
            id j            : src0.eta:12:9..12:10
            assign          : src0.eta:14:7..14:14
            bexpr           : src0.eta:14:11..14:14
            binop           : src0.eta:14:12..14:13
            id i            : src0.eta:14:11..14:12
            int_lit         : src0.eta:14:13..14:14
            id i            : src0.eta:14:7..14:8

        "#]]
    }
}

#[test]
fn io() {
    parsetest! {
        ".eti",
        indoc![r#"
            // I/O support
 
            print(str: int[])     // Print a string to standard output.
            println(str: int[])   // Print a string to standard output, followed by a newline.
            readln() : int[]      // Read from standard input until a newline.
            getchar() : int       // Read a single character from standard input.
                                  // Returns -1 if the end of input has been reached.
            eof() : bool          // Test for end of file on standard input.
        "#],
        expect![[r#"
            src0.eti: parse(clean)
            (interface
              (decl print (params (: str (array int))) (returns))
              (decl println (params (: str (array int))) (returns))
              (decl readln (params) (returns (array int)))
              (decl getchar (params) (returns int))
              (decl eof (params) (returns bool)))
            interface       : src0.eti:3:1..8:13
            mdecl           : src0.eti:3:1..3:18
            id print        : src0.eti:3:1..3:6
            decl            : src0.eti:3:7..3:17
            arr_t           : src0.eti:3:15..3:17
            id str          : src0.eti:3:7..3:10
            mdecl           : src0.eti:4:1..4:20
            id println      : src0.eti:4:1..4:8
            decl            : src0.eti:4:9..4:19
            arr_t           : src0.eti:4:17..4:19
            id str          : src0.eti:4:9..4:12
            mdecl           : src0.eti:5:1..5:17
            id readln       : src0.eti:5:1..5:7
            arr_t           : src0.eti:5:15..5:17
            mdecl           : src0.eti:6:1..6:16
            id getchar      : src0.eti:6:1..6:8
            mdecl           : src0.eti:8:1..8:13
            id eof          : src0.eti:8:1..8:4

        "#]]
    }
}

#[test]
fn length_array() {
    parsetest! {
        ".eta",
        indoc![r#"
            main(args: int[][]) {
                x = length(arr)
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl main (params (: args (array (array int)))) (returns))
                  (block
                    (assign (targets x) (values (length arr)))))))
            program         : src0.eta:1:1..3:2
            method          : src0.eta:1:1..3:2
            mdecl           : src0.eta:1:1..1:20
            id main         : src0.eta:1:1..1:5
            decl            : src0.eta:1:6..1:19
            arr_t           : src0.eta:1:17..1:19
            arr_t           : src0.eta:1:15..1:17
            id args         : src0.eta:1:6..1:10
            block           : src0.eta:1:21..3:2
            assign          : src0.eta:2:5..2:20
            length          : src0.eta:2:9..2:20
            id arr          : src0.eta:2:16..2:19
            id x            : src0.eta:2:5..2:6

        "#]]
    }
}

#[test]
fn lexer_error() {
    parsetest! {
        ".eta",
        indoc![r#"
            // lexically invalid
            main(args: int[][]) {
                x:int = '\q'
            }
        "#],
        expect![[r#"
            src0.eta: parse(failed)
            Error: unknown escape: '\q' is not a recognized escape sequence
               ╭─[ src0.eta:3:14 ]
               │
             3 │     x:int = '\q'
               │              ─┬  
               │               ╰── this isn't a recognized escape sequence
               │ 
               │ Note: valid escapes: '\n', '\t', '\r', '\\', '\'', '\"', '\0', '\x{..}'
            ───╯
        "#]]
    }
}

#[test]
fn mdarrays() {
    parsetest! {
        ".eta",
        indoc![r#"
            foo() {
                a: int[][]
                b: int[3][4]
                a = b
                c: int[3][]
                c[0] = b[0]; c[1] = b[1]; c[2] = b[2]
                d: int[][] = {{1, 0}, {0, 1}}
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl foo (params) (returns))
                  (block
                    (decls (: a (array (array int))))
                    (decls (: b (array (array int 3) 4)))
                    (assign (targets a) (values b))
                    (decls (: c (array (array int 3))))
                    (assign (targets (index c 0)) (values (index b 0)))
                    (assign (targets (index c 1)) (values (index b 1)))
                    (assign (targets (index c 2)) (values (index b 2)))
                    (assign (targets (: d (array (array int)))) (values (array (array 1 0) (array 0 1))))))))
            program         : src0.eta:1:1..8:2
            method          : src0.eta:1:1..8:2
            mdecl           : src0.eta:1:1..1:6
            id foo          : src0.eta:1:1..1:4
            block           : src0.eta:1:7..8:2
            decls           : src0.eta:2:5..2:15
            decl            : src0.eta:2:5..2:15
            arr_t           : src0.eta:2:13..2:15
            arr_t           : src0.eta:2:11..2:13
            id a            : src0.eta:2:5..2:6
            decls           : src0.eta:3:5..3:17
            decl            : src0.eta:3:5..3:17
            arr_t           : src0.eta:3:14..3:17
            arr_t           : src0.eta:3:11..3:14
            int_lit         : src0.eta:3:12..3:13
            int_lit         : src0.eta:3:15..3:16
            id b            : src0.eta:3:5..3:6
            assign          : src0.eta:4:5..4:10
            id b            : src0.eta:4:9..4:10
            id a            : src0.eta:4:5..4:6
            decls           : src0.eta:5:5..5:16
            decl            : src0.eta:5:5..5:16
            arr_t           : src0.eta:5:14..5:16
            arr_t           : src0.eta:5:11..5:14
            int_lit         : src0.eta:5:12..5:13
            id c            : src0.eta:5:5..5:6
            assign          : src0.eta:6:5..6:16
            expr_idx        : src0.eta:6:12..6:16
            id b            : src0.eta:6:12..6:13
            int_lit         : src0.eta:6:14..6:15
            expr_idx        : src0.eta:6:5..6:9
            id c            : src0.eta:6:5..6:6
            int_lit         : src0.eta:6:7..6:8
            assign          : src0.eta:6:18..6:29
            expr_idx        : src0.eta:6:25..6:29
            id b            : src0.eta:6:25..6:26
            int_lit         : src0.eta:6:27..6:28
            expr_idx        : src0.eta:6:18..6:22
            id c            : src0.eta:6:18..6:19
            int_lit         : src0.eta:6:20..6:21
            assign          : src0.eta:6:31..6:42
            expr_idx        : src0.eta:6:38..6:42
            id b            : src0.eta:6:38..6:39
            int_lit         : src0.eta:6:40..6:41
            expr_idx        : src0.eta:6:31..6:35
            id c            : src0.eta:6:31..6:32
            int_lit         : src0.eta:6:33..6:34
            assign          : src0.eta:7:5..7:34
            arr_lit         : src0.eta:7:18..7:34
            arr_lit         : src0.eta:7:19..7:25
            int_lit         : src0.eta:7:20..7:21
            int_lit         : src0.eta:7:23..7:24
            arr_lit         : src0.eta:7:27..7:33
            int_lit         : src0.eta:7:28..7:29
            int_lit         : src0.eta:7:31..7:32
            decl            : src0.eta:7:5..7:15
            arr_t           : src0.eta:7:13..7:15
            arr_t           : src0.eta:7:11..7:13
            id d            : src0.eta:7:5..7:6

        "#]]
    }
}

#[test]
fn multi_return() {
    parsetest! {
        ".eta",
        indoc![r#"
            triple(x: int): int, int, int {
                a: int = x + 1
                b: int = x + 2
                return x, a, b
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl triple (params (: x int)) (returns int int int))
                  (block
                    (assign (targets (: a int)) (values (+ x 1)))
                    (assign (targets (: b int)) (values (+ x 2)))
                    (return x a b)))))
            program         : src0.eta:1:1..5:2
            method          : src0.eta:1:1..5:2
            mdecl           : src0.eta:1:1..1:30
            id triple       : src0.eta:1:1..1:7
            decl            : src0.eta:1:8..1:14
            id x            : src0.eta:1:8..1:9
            block           : src0.eta:1:31..5:2
            assign          : src0.eta:2:5..2:19
            bexpr           : src0.eta:2:14..2:19
            binop           : src0.eta:2:16..2:17
            id x            : src0.eta:2:14..2:15
            int_lit         : src0.eta:2:18..2:19
            decl            : src0.eta:2:5..2:11
            id a            : src0.eta:2:5..2:6
            assign          : src0.eta:3:5..3:19
            bexpr           : src0.eta:3:14..3:19
            binop           : src0.eta:3:16..3:17
            id x            : src0.eta:3:14..3:15
            int_lit         : src0.eta:3:18..3:19
            decl            : src0.eta:3:5..3:11
            id b            : src0.eta:3:5..3:6
            return          : src0.eta:4:5..4:19
            id x            : src0.eta:4:12..4:13
            id a            : src0.eta:4:15..4:16
            id b            : src0.eta:4:18..4:19

        "#]]
    }
}

#[test]
fn multi_use() {
    parsetest! {
        ".eta",
        indoc![r#"
            use io
            use conv;
 
            main(args: int[][]) {
                println(unparseInt(42))
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses
                (use io)
                (use conv))
              (defs
                (method (decl main (params (: args (array (array int)))) (returns))
                  (block
                    (call println (call unparseInt 42))))))
            program         : src0.eta:1:1..6:2
            use             : src0.eta:1:1..1:7
            id io           : src0.eta:1:5..1:7
            use             : src0.eta:2:1..2:10
            id conv         : src0.eta:2:5..2:9
            method          : src0.eta:4:1..6:2
            mdecl           : src0.eta:4:1..4:20
            id main         : src0.eta:4:1..4:5
            decl            : src0.eta:4:6..4:19
            arr_t           : src0.eta:4:17..4:19
            arr_t           : src0.eta:4:15..4:17
            id args         : src0.eta:4:6..4:10
            block           : src0.eta:4:21..6:2
            pcall           : src0.eta:5:5..5:28
            id println      : src0.eta:5:5..5:12
            pcall           : src0.eta:5:13..5:27
            id unparseInt   : src0.eta:5:13..5:23
            int_lit         : src0.eta:5:24..5:26

        "#]]
    }
}

#[test]
fn multi_var_decls() {
    parsetest! {
        ".eta",
        indoc![r#"
            main(args: int[][]) {
                x:int, z, y:int = 1, 2
                a:int, b:bool = 3, true
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl main (params (: args (array (array int)))) (returns))
                  (block
                    (assign (targets (: x int) z (: y int)) (values 1 2))
                    (assign (targets (: a int) (: b bool)) (values 3 true))))))
            program         : src0.eta:1:1..4:2
            method          : src0.eta:1:1..4:2
            mdecl           : src0.eta:1:1..1:20
            id main         : src0.eta:1:1..1:5
            decl            : src0.eta:1:6..1:19
            arr_t           : src0.eta:1:17..1:19
            arr_t           : src0.eta:1:15..1:17
            id args         : src0.eta:1:6..1:10
            block           : src0.eta:1:21..4:2
            assign          : src0.eta:2:5..2:27
            int_lit         : src0.eta:2:23..2:24
            int_lit         : src0.eta:2:26..2:27
            decl            : src0.eta:2:5..2:10
            id x            : src0.eta:2:5..2:6
            id z            : src0.eta:2:12..2:13
            decl            : src0.eta:2:15..2:20
            id y            : src0.eta:2:15..2:16
            assign          : src0.eta:3:5..3:28
            int_lit         : src0.eta:3:21..3:22
            bool_lit        : src0.eta:3:24..3:28
            decl            : src0.eta:3:5..3:10
            id a            : src0.eta:3:5..3:6
            decl            : src0.eta:3:12..3:18
            id b            : src0.eta:3:12..3:13

        "#]]
    }
}

#[test]
fn multi_wildcard() {
    parsetest! {
        ".eta",
        indoc![r#"
            foo() {
                _, _, _ = g()
                _ = h()
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl foo (params) (returns))
                  (block
                    (assign (targets _ _ _) (values (call g)))
                    (assign (targets _) (values (call h)))))))
            program         : src0.eta:1:1..4:2
            method          : src0.eta:1:1..4:2
            mdecl           : src0.eta:1:1..1:6
            id foo          : src0.eta:1:1..1:4
            block           : src0.eta:1:7..4:2
            assign          : src0.eta:2:5..2:18
            pcall           : src0.eta:2:15..2:18
            id g            : src0.eta:2:15..2:16
            assign          : src0.eta:3:5..3:12
            pcall           : src0.eta:3:9..3:12
            id h            : src0.eta:3:9..3:10

        "#]]
    }
}

#[test]
fn negative_global() {
    parsetest! {
        ".eta",
        indoc![r#"
            n: int = -1
            x: int = -9223372036854775808
            f() {}
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (global n int -1)
                (global x int -9223372036854775808)
                (method (decl f (params) (returns))
                  (block))))
            program         : src0.eta:1:1..3:7
            gdecl           : src0.eta:1:1..1:12
            id n            : src0.eta:1:1..1:2
            int_lit         : src0.eta:1:11..1:12
            gdecl           : src0.eta:2:1..2:30
            id x            : src0.eta:2:1..2:2
            int_lit         : src0.eta:2:11..2:30
            method          : src0.eta:3:1..3:7
            mdecl           : src0.eta:3:1..3:4
            id f            : src0.eta:3:1..3:2
            block           : src0.eta:3:5..3:7

        "#]]
    }
}

#[test]
fn nested_blocks() {
    parsetest! {
        ".eta",
        indoc![r#"
            f() {
                x: int = 1
                {
                    y: int = 2
                    x = x + y
                }
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl f (params) (returns))
                  (block
                    (assign (targets (: x int)) (values 1))
                    (block
                      (assign (targets (: y int)) (values 2))
                      (assign (targets x) (values (+ x y))))))))
            program         : src0.eta:1:1..7:2
            method          : src0.eta:1:1..7:2
            mdecl           : src0.eta:1:1..1:4
            id f            : src0.eta:1:1..1:2
            block           : src0.eta:1:5..7:2
            assign          : src0.eta:2:5..2:15
            int_lit         : src0.eta:2:14..2:15
            decl            : src0.eta:2:5..2:11
            id x            : src0.eta:2:5..2:6
            block           : src0.eta:3:5..6:6
            assign          : src0.eta:4:9..4:19
            int_lit         : src0.eta:4:18..4:19
            decl            : src0.eta:4:9..4:15
            id y            : src0.eta:4:9..4:10
            assign          : src0.eta:5:9..5:18
            bexpr           : src0.eta:5:13..5:18
            binop           : src0.eta:5:15..5:16
            id x            : src0.eta:5:13..5:14
            id y            : src0.eta:5:17..5:18
            id x            : src0.eta:5:9..5:10

        "#]]
    }
}

#[test]
fn nested_empty_blocks() {
    parsetest! {
        ".eta",
        indoc![r#"
            f() {
                {}
                {{{}}}
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl f (params) (returns))
                  (block
                    (block)
                    (block
                      (block
                        (block)))))))
            program         : src0.eta:1:1..4:2
            method          : src0.eta:1:1..4:2
            mdecl           : src0.eta:1:1..1:4
            id f            : src0.eta:1:1..1:2
            block           : src0.eta:1:5..4:2
            block           : src0.eta:2:5..2:7
            block           : src0.eta:3:5..3:11
            block           : src0.eta:3:6..3:10
            block           : src0.eta:3:7..3:9

        "#]]
    }
}

#[test]
fn no_type_decl() {
    parsetest! {
        ".eta",
        indoc![r#"
            f() {
                x = 5
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl f (params) (returns))
                  (block
                    (assign (targets x) (values 5))))))
            program         : src0.eta:1:1..3:2
            method          : src0.eta:1:1..3:2
            mdecl           : src0.eta:1:1..1:4
            id f            : src0.eta:1:1..1:2
            block           : src0.eta:1:5..3:2
            assign          : src0.eta:2:5..2:10
            int_lit         : src0.eta:2:9..2:10
            id x            : src0.eta:2:5..2:6

        "#]]
    }
}

#[test]
fn only_globals() {
    parsetest! {
        ".eta",
        indoc![r#"
            x: int = 1
            y: bool = false
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (global x int 1)
                (global y bool false)))
            program         : src0.eta:1:1..2:16
            gdecl           : src0.eta:1:1..1:11
            id x            : src0.eta:1:1..1:2
            int_lit         : src0.eta:1:10..1:11
            gdecl           : src0.eta:2:1..2:16
            id y            : src0.eta:2:1..2:2
            bool_lit        : src0.eta:2:11..2:16

        "#]]
    }
}

#[test]
fn paren_expr() {
    parsetest! {
        ".eta",
        indoc![r#"
            f(a: int, b: int): int {
                c: int = (a + b) * (a - b)
                return c
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl f (params (: a int) (: b int)) (returns int))
                  (block
                    (assign (targets (: c int)) (values (* (+ a b) (- a b))))
                    (return c)))))
            program         : src0.eta:1:1..4:2
            method          : src0.eta:1:1..4:2
            mdecl           : src0.eta:1:1..1:23
            id f            : src0.eta:1:1..1:2
            decl            : src0.eta:1:3..1:9
            id a            : src0.eta:1:3..1:4
            decl            : src0.eta:1:11..1:17
            id b            : src0.eta:1:11..1:12
            block           : src0.eta:1:24..4:2
            assign          : src0.eta:2:5..2:31
            bexpr           : src0.eta:2:14..2:31
            binop           : src0.eta:2:22..2:23
            bexpr           : src0.eta:2:15..2:20
            binop           : src0.eta:2:17..2:18
            id a            : src0.eta:2:15..2:16
            id b            : src0.eta:2:19..2:20
            bexpr           : src0.eta:2:25..2:30
            binop           : src0.eta:2:27..2:28
            id a            : src0.eta:2:25..2:26
            id b            : src0.eta:2:29..2:30
            decl            : src0.eta:2:5..2:11
            id c            : src0.eta:2:5..2:6
            return          : src0.eta:3:5..3:13
            id c            : src0.eta:3:12..3:13

        "#]]
    }
}

#[test]
fn proc_return_value() {
    parsetest! {
        ".eta",
        indoc![r#"
            f() {
                return 1
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl f (params) (returns))
                  (block
                    (return 1)))))
            program         : src0.eta:1:1..3:2
            method          : src0.eta:1:1..3:2
            mdecl           : src0.eta:1:1..1:4
            id f            : src0.eta:1:1..1:2
            block           : src0.eta:1:5..3:2
            return          : src0.eta:2:5..2:13
            int_lit         : src0.eta:2:12..2:13

        "#]]
    }
}

#[test]
fn procedure() {
    parsetest! {
        ".eta",
        indoc![r#"
            doNothing(x: int) {
                x = 0
                return
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl doNothing (params (: x int)) (returns))
                  (block
                    (assign (targets x) (values 0))
                    (return)))))
            program         : src0.eta:1:1..4:2
            method          : src0.eta:1:1..4:2
            mdecl           : src0.eta:1:1..1:18
            id doNothing    : src0.eta:1:1..1:10
            decl            : src0.eta:1:11..1:17
            id x            : src0.eta:1:11..1:12
            block           : src0.eta:1:19..4:2
            assign          : src0.eta:2:5..2:10
            int_lit         : src0.eta:2:9..2:10
            id x            : src0.eta:2:5..2:6
            return          : src0.eta:3:5..4:1

        "#]]
    }
}

#[test]
fn ratadd() {
    parsetest! {
        ".eta",
        indoc![r#"
            // Add two rational numbers p1/q1 and p2/q2, returning
            // a pair (p3, q3) representing their sum p3/q3.
            ratadd(p1:int, q1:int, p2:int, q2:int) : int, int {
                g:int = gcd(q1,q2)
                p3:int = p1*(q2/g) + p2*(q1/g)
                return p3, q1/g*q2
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl ratadd (params (: p1 int) (: q1 int) (: p2 int) (: q2 int)) (returns int int))
                  (block
                    (assign (targets (: g int)) (values (call gcd q1 q2)))
                    (assign (targets (: p3 int)) (values (+ (* p1 (/ q2 g)) (* p2 (/ q1 g)))))
                    (return p3 (* (/ q1 g) q2))))))
            program         : src0.eta:3:1..7:2
            method          : src0.eta:3:1..7:2
            mdecl           : src0.eta:3:1..3:50
            id ratadd       : src0.eta:3:1..3:7
            decl            : src0.eta:3:8..3:14
            id p1           : src0.eta:3:8..3:10
            decl            : src0.eta:3:16..3:22
            id q1           : src0.eta:3:16..3:18
            decl            : src0.eta:3:24..3:30
            id p2           : src0.eta:3:24..3:26
            decl            : src0.eta:3:32..3:38
            id q2           : src0.eta:3:32..3:34
            block           : src0.eta:3:51..7:2
            assign          : src0.eta:4:5..4:23
            pcall           : src0.eta:4:13..4:23
            id gcd          : src0.eta:4:13..4:16
            id q1           : src0.eta:4:17..4:19
            id q2           : src0.eta:4:20..4:22
            decl            : src0.eta:4:5..4:10
            id g            : src0.eta:4:5..4:6
            assign          : src0.eta:5:5..5:35
            bexpr           : src0.eta:5:14..5:35
            binop           : src0.eta:5:24..5:25
            bexpr           : src0.eta:5:14..5:23
            binop           : src0.eta:5:16..5:17
            id p1           : src0.eta:5:14..5:16
            bexpr           : src0.eta:5:18..5:22
            binop           : src0.eta:5:20..5:21
            id q2           : src0.eta:5:18..5:20
            id g            : src0.eta:5:21..5:22
            bexpr           : src0.eta:5:26..5:35
            binop           : src0.eta:5:28..5:29
            id p2           : src0.eta:5:26..5:28
            bexpr           : src0.eta:5:30..5:34
            binop           : src0.eta:5:32..5:33
            id q1           : src0.eta:5:30..5:32
            id g            : src0.eta:5:33..5:34
            decl            : src0.eta:5:5..5:11
            id p3           : src0.eta:5:5..5:7
            return          : src0.eta:6:5..6:23
            id p3           : src0.eta:6:12..6:14
            bexpr           : src0.eta:6:16..6:23
            binop           : src0.eta:6:20..6:21
            bexpr           : src0.eta:6:16..6:20
            binop           : src0.eta:6:18..6:19
            id q1           : src0.eta:6:16..6:18
            id g            : src0.eta:6:19..6:20
            id q2           : src0.eta:6:21..6:23

        "#]]
    }
}

#[test]
fn ratadduse() {
    parsetest! {
        ".eta",
        indoc![r#"
            foo() {
                p:int, q:int = ratadd(2, 5, 1, 3)
                _, q':int = ratadd(1, 2, 1, 3)
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl foo (params) (returns))
                  (block
                    (assign (targets (: p int) (: q int)) (values (call ratadd 2 5 1 3)))
                    (assign (targets _ (: q' int)) (values (call ratadd 1 2 1 3)))))))
            program         : src0.eta:1:1..4:2
            method          : src0.eta:1:1..4:2
            mdecl           : src0.eta:1:1..1:6
            id foo          : src0.eta:1:1..1:4
            block           : src0.eta:1:7..4:2
            assign          : src0.eta:2:5..2:38
            pcall           : src0.eta:2:20..2:38
            id ratadd       : src0.eta:2:20..2:26
            int_lit         : src0.eta:2:27..2:28
            int_lit         : src0.eta:2:30..2:31
            int_lit         : src0.eta:2:33..2:34
            int_lit         : src0.eta:2:36..2:37
            decl            : src0.eta:2:5..2:10
            id p            : src0.eta:2:5..2:6
            decl            : src0.eta:2:12..2:17
            id q            : src0.eta:2:12..2:13
            assign          : src0.eta:3:5..3:35
            pcall           : src0.eta:3:17..3:35
            id ratadd       : src0.eta:3:17..3:23
            int_lit         : src0.eta:3:24..3:25
            int_lit         : src0.eta:3:27..3:28
            int_lit         : src0.eta:3:30..3:31
            int_lit         : src0.eta:3:33..3:34
            decl            : src0.eta:3:8..3:14
            id q'           : src0.eta:3:8..3:10

        "#]]
    }
}

#[test]
fn referr_block01() {
    parsetest! {
        ".eta",
        indoc![r#"
            f() {
              x:int;
              x;=;2
            }
        "#],
        expect![[r#"
            src0.eta: parse(recovered)
            (program
              (uses)
              (defs
                (method (decl f (params) (returns))
                  (block
                    (decls (: x int))
                    (error)
                    (error)
                    (error)))))
            program         : src0.eta:1:1..4:2
            method          : src0.eta:1:1..4:2
            mdecl           : src0.eta:1:1..1:4
            id f            : src0.eta:1:1..1:2
            block           : src0.eta:1:5..4:2
            decls           : src0.eta:2:3..2:8
            decl            : src0.eta:2:3..2:8
            id x            : src0.eta:2:3..2:4

            Error: unexpected token `;`
               ╭─[ src0.eta:3:4 ]
               │
             3 │   x;=;2
               │    ┬  
               │    ╰── expected one of ":", "=", ",", "(", or "["
            ───╯
            Error: unexpected token `=`
               ╭─[ src0.eta:3:5 ]
               │
             3 │   x;=;2
               │     ┬  
               │     ╰── expected one of "while", "if", "return", "_", "{", "}", or "identifier"
            ───╯
            Error: unexpected token `2`
               ╭─[ src0.eta:3:7 ]
               │
             3 │   x;=;2
               │       ┬  
               │       ╰── expected one of "while", "if", "return", "_", "{", "}", or "identifier"
            ───╯
        "#]]
    }
}


#[test]
fn referr_use01() {
    parsetest! {
        ".eta",
        indoc![r#"
            use A
        "#],
        expect![[r#"
            src0.eta: parse(failed)
            Error: Program must contain at least one definition
               ╭─[ src0.eta:1:6 ]
               │
             1 │ use A
               │      │ 
               │      ╰─ expected definition
            ───╯
        "#]]
    }
}

#[test]
fn referr_use02() {
    parsetest! {
        ".eta",
        indoc![r#"
            use A;
        "#],
        expect![[r#"
            src0.eta: parse(failed)
            Error: Program must contain at least one definition
               ╭─[ src0.eta:1:7 ]
               │
             1 │ use A;
               │       │ 
               │       ╰─ expected definition
            ───╯
        "#]]
    }
}

#[test]
fn referr_vardecl01() {
    parsetest! {
        ".eta",
        indoc![r#"
            f() {
              x:bool[][1]
            }
        "#],
        expect![[r#"
            src0.eta: parse(recovered)
            (program
              (uses)
              (defs
                (method (decl f (params) (returns))
                  (block
                    (decls (: x (error)))))))
            program         : src0.eta:1:1..3:2
            method          : src0.eta:1:1..3:2
            mdecl           : src0.eta:1:1..1:4
            id f            : src0.eta:1:1..1:2
            block           : src0.eta:1:5..3:2
            decls           : src0.eta:2:3..2:14
            decl            : src0.eta:2:3..2:14
            id x            : src0.eta:2:3..2:4

            Error: unexpected token `1`
               ╭─[ src0.eta:2:12 ]
               │
             2 │   x:bool[][1]
               │            ┬  
               │            ╰── expected "]"
            ───╯
        "#]]
    }
}

#[test]
fn referr_vardecl02() {
    parsetest! {
        ".eta",
        indoc![r#"
            f() {
              x:bool[][1][]
            }
        "#],
        expect![[r#"
            src0.eta: parse(recovered)
            (program
              (uses)
              (defs
                (method (decl f (params) (returns))
                  (block
                    (decls (: x (error)))))))
            program         : src0.eta:1:1..3:2
            method          : src0.eta:1:1..3:2
            mdecl           : src0.eta:1:1..1:4
            id f            : src0.eta:1:1..1:2
            block           : src0.eta:1:5..3:2
            decls           : src0.eta:2:3..2:16
            decl            : src0.eta:2:3..2:16
            id x            : src0.eta:2:3..2:4

            Error: unexpected token `1`
               ╭─[ src0.eta:2:12 ]
               │
             2 │   x:bool[][1][]
               │            ┬  
               │            ╰── expected "]"
            ───╯
        "#]]
    }
}

#[test]
fn referr_vardecl03() {
    parsetest! {
        ".eta",
        indoc![r#"
            f() {
              __ = g()
            }
 
            g():int {
              return 0
            }
        "#],
        expect![[r#"
            src0.eta: parse(recovered)
            (program
              (uses)
              (defs
                (method (decl f (params) (returns))
                  (block
                    (error)
                    (assign (targets _) (values (call g)))))
                (method (decl g (params) (returns int))
                  (block
                    (return 0)))))
            program         : src0.eta:1:1..7:2
            method          : src0.eta:1:1..3:2
            mdecl           : src0.eta:1:1..1:4
            id f            : src0.eta:1:1..1:2
            block           : src0.eta:1:5..3:2
            assign          : src0.eta:2:4..2:11
            pcall           : src0.eta:2:8..2:11
            id g            : src0.eta:2:8..2:9
            method          : src0.eta:5:1..7:2
            mdecl           : src0.eta:5:1..5:8
            id g            : src0.eta:5:1..5:2
            block           : src0.eta:5:9..7:2
            return          : src0.eta:6:3..6:11
            int_lit         : src0.eta:6:10..6:11

            Error: unexpected token `_`
               ╭─[ src0.eta:2:4 ]
               │
             2 │   __ = g()
               │    ┬  
               │    ╰── expected one of "=", or ","
            ───╯
        "#]]
    }
}

#[test]
fn return_only_block() {
    parsetest! {
        ".eta",
        indoc![r#"
            identity(x: int): int {
                return x
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl identity (params (: x int)) (returns int))
                  (block
                    (return x)))))
            program         : src0.eta:1:1..3:2
            method          : src0.eta:1:1..3:2
            mdecl           : src0.eta:1:1..1:22
            id identity     : src0.eta:1:1..1:9
            decl            : src0.eta:1:10..1:16
            id x            : src0.eta:1:10..1:11
            block           : src0.eta:1:23..3:2
            return          : src0.eta:2:5..2:13
            id x            : src0.eta:2:12..2:13

        "#]]
    }
}

#[test]
fn sized_array_with_init() {
    parsetest! {
        ".eta",
        indoc![r#"
            f() {
                a: int[3] = {1, 2, 3}
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl f (params) (returns))
                  (block
                    (assign (targets (: a (array int 3))) (values (array 1 2 3)))))))
            program         : src0.eta:1:1..3:2
            method          : src0.eta:1:1..3:2
            mdecl           : src0.eta:1:1..1:4
            id f            : src0.eta:1:1..1:2
            block           : src0.eta:1:5..3:2
            assign          : src0.eta:2:5..2:26
            arr_lit         : src0.eta:2:17..2:26
            int_lit         : src0.eta:2:18..2:19
            int_lit         : src0.eta:2:21..2:22
            int_lit         : src0.eta:2:24..2:25
            decl            : src0.eta:2:5..2:14
            arr_t           : src0.eta:2:11..2:14
            int_lit         : src0.eta:2:12..2:13
            id a            : src0.eta:2:5..2:6

        "#]]
    }
}

#[test]
fn spec1() {
    parsetest! {
        ".eta",
        indoc![r#"
            foo() {
                x:int = 2;
                z:int;
                b: bool, i:int = f(x);
                s: int[] = "Hello";
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl foo (params) (returns))
                  (block
                    (assign (targets (: x int)) (values 2))
                    (decls (: z int))
                    (assign (targets (: b bool) (: i int)) (values (call f x)))
                    (assign (targets (: s (array int))) (values Spur(8)))))))
            program         : src0.eta:1:1..6:2
            method          : src0.eta:1:1..6:2
            mdecl           : src0.eta:1:1..1:6
            id foo          : src0.eta:1:1..1:4
            block           : src0.eta:1:7..6:2
            assign          : src0.eta:2:5..2:14
            int_lit         : src0.eta:2:13..2:14
            decl            : src0.eta:2:5..2:10
            id x            : src0.eta:2:5..2:6
            decls           : src0.eta:3:5..3:10
            decl            : src0.eta:3:5..3:10
            id z            : src0.eta:3:5..3:6
            assign          : src0.eta:4:5..4:26
            pcall           : src0.eta:4:22..4:26
            id f            : src0.eta:4:22..4:23
            id x            : src0.eta:4:24..4:25
            decl            : src0.eta:4:5..4:12
            id b            : src0.eta:4:5..4:6
            decl            : src0.eta:4:14..4:19
            id i            : src0.eta:4:14..4:15
            assign          : src0.eta:5:5..5:23
            string_lit      : src0.eta:5:16..5:23
            decl            : src0.eta:5:5..5:13
            arr_t           : src0.eta:5:11..5:13
            id s            : src0.eta:5:5..5:6

        "#]]
    }
}

#[test]
fn spec2() {
    parsetest! {
        ".eta",
        indoc![r#"
            foo() {
              x = x + 1
              s = {1, 2, 3}
              b = !b
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl foo (params) (returns))
                  (block
                    (assign (targets x) (values (+ x 1)))
                    (assign (targets s) (values (array 1 2 3)))
                    (assign (targets b) (values (! b)))))))
            program         : src0.eta:1:1..5:2
            method          : src0.eta:1:1..5:2
            mdecl           : src0.eta:1:1..1:6
            id foo          : src0.eta:1:1..1:4
            block           : src0.eta:1:7..5:2
            assign          : src0.eta:2:3..2:12
            bexpr           : src0.eta:2:7..2:12
            binop           : src0.eta:2:9..2:10
            id x            : src0.eta:2:7..2:8
            int_lit         : src0.eta:2:11..2:12
            id x            : src0.eta:2:3..2:4
            assign          : src0.eta:3:3..3:16
            arr_lit         : src0.eta:3:7..3:16
            int_lit         : src0.eta:3:8..3:9
            int_lit         : src0.eta:3:11..3:12
            int_lit         : src0.eta:3:14..3:15
            id s            : src0.eta:3:3..3:4
            assign          : src0.eta:4:3..4:9
            uexpr           : src0.eta:4:7..4:9
            uop             : src0.eta:4:7..4:8
            id b            : src0.eta:4:8..4:9
            id b            : src0.eta:4:3..4:4

        "#]]
    }
}

#[test]
fn spec3() {
    parsetest! {
        ".eta",
        indoc![r#"
            foo() {
                s: int[] = "Hello" + {13, 10}
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl foo (params) (returns))
                  (block
                    (assign (targets (: s (array int))) (values (+ Spur(3) (array 13 10))))))))
            program         : src0.eta:1:1..3:2
            method          : src0.eta:1:1..3:2
            mdecl           : src0.eta:1:1..1:6
            id foo          : src0.eta:1:1..1:4
            block           : src0.eta:1:7..3:2
            assign          : src0.eta:2:5..2:34
            bexpr           : src0.eta:2:16..2:34
            binop           : src0.eta:2:24..2:25
            string_lit      : src0.eta:2:16..2:23
            arr_lit         : src0.eta:2:26..2:34
            int_lit         : src0.eta:2:27..2:29
            int_lit         : src0.eta:2:31..2:33
            decl            : src0.eta:2:5..2:13
            arr_t           : src0.eta:2:11..2:13
            id s            : src0.eta:2:5..2:6

        "#]]
    }
}

#[test]
fn string_index() {
    parsetest! {
        ".eta",
        indoc![r#"
            f(): int {
                x: int = "Hello"[0]
                return x
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl f (params) (returns int))
                  (block
                    (assign (targets (: x int)) (values (index Spur(3) 0)))
                    (return x)))))
            program         : src0.eta:1:1..4:2
            method          : src0.eta:1:1..4:2
            mdecl           : src0.eta:1:1..1:9
            id f            : src0.eta:1:1..1:2
            block           : src0.eta:1:10..4:2
            assign          : src0.eta:2:5..2:24
            expr_idx        : src0.eta:2:14..2:24
            string_lit      : src0.eta:2:14..2:21
            int_lit         : src0.eta:2:22..2:23
            decl            : src0.eta:2:5..2:11
            id x            : src0.eta:2:5..2:6
            return          : src0.eta:3:5..3:13
            id x            : src0.eta:3:12..3:13

        "#]]
    }
}

#[test]
fn swap_assign() {
    parsetest! {
        ".eta",
        indoc![r#"
            f() {
                a: int = 1
                b: int = 2
                a, b = swap(a, b)
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl f (params) (returns))
                  (block
                    (assign (targets (: a int)) (values 1))
                    (assign (targets (: b int)) (values 2))
                    (assign (targets a b) (values (call swap a b)))))))
            program         : src0.eta:1:1..5:2
            method          : src0.eta:1:1..5:2
            mdecl           : src0.eta:1:1..1:4
            id f            : src0.eta:1:1..1:2
            block           : src0.eta:1:5..5:2
            assign          : src0.eta:2:5..2:15
            int_lit         : src0.eta:2:14..2:15
            decl            : src0.eta:2:5..2:11
            id a            : src0.eta:2:5..2:6
            assign          : src0.eta:3:5..3:15
            int_lit         : src0.eta:3:14..3:15
            decl            : src0.eta:3:5..3:11
            id b            : src0.eta:3:5..3:6
            assign          : src0.eta:4:5..4:22
            pcall           : src0.eta:4:12..4:22
            id swap         : src0.eta:4:12..4:16
            id a            : src0.eta:4:17..4:18
            id b            : src0.eta:4:20..4:21
            id a            : src0.eta:4:5..4:6
            id b            : src0.eta:4:8..4:9

        "#]]
    }
}

#[test]
fn trailing_comma() {
    parsetest! {
        ".eta",
        indoc![r#"
            f() {
                a: int[] = {2,}
                b: int[] = {1, 2, 3,}
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl f (params) (returns))
                  (block
                    (assign (targets (: a (array int))) (values (array 2)))
                    (assign (targets (: b (array int))) (values (array 1 2 3)))))))
            program         : src0.eta:1:1..4:2
            method          : src0.eta:1:1..4:2
            mdecl           : src0.eta:1:1..1:4
            id f            : src0.eta:1:1..1:2
            block           : src0.eta:1:5..4:2
            assign          : src0.eta:2:5..2:20
            arr_lit         : src0.eta:2:16..2:20
            int_lit         : src0.eta:2:17..2:18
            decl            : src0.eta:2:5..2:13
            arr_t           : src0.eta:2:11..2:13
            id a            : src0.eta:2:5..2:6
            assign          : src0.eta:3:5..3:26
            arr_lit         : src0.eta:3:16..3:26
            int_lit         : src0.eta:3:17..3:18
            int_lit         : src0.eta:3:20..3:21
            int_lit         : src0.eta:3:23..3:24
            decl            : src0.eta:3:5..3:13
            arr_t           : src0.eta:3:11..3:13
            id b            : src0.eta:3:5..3:6

        "#]]
    }
}

#[test]
fn triple_unary() {
    parsetest! {
        ".eta",
        indoc![r#"
            f(x: int, b: bool): int, bool {
                return ---x, !!!b
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl f (params (: x int) (: b bool)) (returns int bool))
                  (block
                    (return (- (- (- x))) (! (! (! b))))))))
            program         : src0.eta:1:1..3:2
            method          : src0.eta:1:1..3:2
            mdecl           : src0.eta:1:1..1:30
            id f            : src0.eta:1:1..1:2
            decl            : src0.eta:1:3..1:9
            id x            : src0.eta:1:3..1:4
            decl            : src0.eta:1:11..1:18
            id b            : src0.eta:1:11..1:12
            block           : src0.eta:1:31..3:2
            return          : src0.eta:2:5..2:22
            uexpr           : src0.eta:2:12..2:16
            uop             : src0.eta:2:12..2:13
            uexpr           : src0.eta:2:13..2:16
            uop             : src0.eta:2:13..2:14
            uexpr           : src0.eta:2:14..2:16
            uop             : src0.eta:2:14..2:15
            id x            : src0.eta:2:15..2:16
            uexpr           : src0.eta:2:18..2:22
            uop             : src0.eta:2:18..2:19
            uexpr           : src0.eta:2:19..2:22
            uop             : src0.eta:2:19..2:20
            uexpr           : src0.eta:2:20..2:22
            uop             : src0.eta:2:20..2:21
            id b            : src0.eta:2:21..2:22

        "#]]
    }
}

#[test]
fn unary_chain() {
    parsetest! {
        ".eta",
        indoc![r#"
            f(x: int, b: bool) {
                y: int = - -x
                c: bool = !!b
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl f (params (: x int) (: b bool)) (returns))
                  (block
                    (assign (targets (: y int)) (values (- (- x))))
                    (assign (targets (: c bool)) (values (! (! b))))))))
            program         : src0.eta:1:1..4:2
            method          : src0.eta:1:1..4:2
            mdecl           : src0.eta:1:1..1:19
            id f            : src0.eta:1:1..1:2
            decl            : src0.eta:1:3..1:9
            id x            : src0.eta:1:3..1:4
            decl            : src0.eta:1:11..1:18
            id b            : src0.eta:1:11..1:12
            block           : src0.eta:1:20..4:2
            assign          : src0.eta:2:5..2:18
            uexpr           : src0.eta:2:14..2:18
            uop             : src0.eta:2:14..2:15
            uexpr           : src0.eta:2:16..2:18
            uop             : src0.eta:2:16..2:17
            id x            : src0.eta:2:17..2:18
            decl            : src0.eta:2:5..2:11
            id y            : src0.eta:2:5..2:6
            assign          : src0.eta:3:5..3:18
            uexpr           : src0.eta:3:15..3:18
            uop             : src0.eta:3:15..3:16
            uexpr           : src0.eta:3:16..3:18
            uop             : src0.eta:3:16..3:17
            id b            : src0.eta:3:17..3:18
            decl            : src0.eta:3:5..3:12
            id c            : src0.eta:3:5..3:6

        "#]]
    }
}

#[test]
fn unary_minus() {
    parsetest! {
        ".eta",
        indoc![r#"
            f(x:int, y:int[]) {
              x = -0
              x = -'a'
              x = -x
              x = -i()
              x = -y[0]
              x = - -1
              x = (0)
              x = -9223372036854775808
            }
 
            i(): int {
              return 0
            }
 
            g(x:bool) {
              x = !true
              x = !false
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl f (params (: x int) (: y (array int))) (returns))
                  (block
                    (assign (targets x) (values (- 0)))
                    (assign (targets x) (values (- a)))
                    (assign (targets x) (values (- x)))
                    (assign (targets x) (values (- (call i))))
                    (assign (targets x) (values (- (index y 0))))
                    (assign (targets x) (values (- (- 1))))
                    (assign (targets x) (values 0))
                    (assign (targets x) (values (- 9223372036854775808)))))
                (method (decl i (params) (returns int))
                  (block
                    (return 0)))
                (method (decl g (params (: x bool)) (returns))
                  (block
                    (assign (targets x) (values (! true)))
                    (assign (targets x) (values (! false)))))))
            program         : src0.eta:1:1..19:2
            method          : src0.eta:1:1..10:2
            mdecl           : src0.eta:1:1..1:18
            id f            : src0.eta:1:1..1:2
            decl            : src0.eta:1:3..1:8
            id x            : src0.eta:1:3..1:4
            decl            : src0.eta:1:10..1:17
            arr_t           : src0.eta:1:15..1:17
            id y            : src0.eta:1:10..1:11
            block           : src0.eta:1:19..10:2
            assign          : src0.eta:2:3..2:9
            uexpr           : src0.eta:2:7..2:9
            uop             : src0.eta:2:7..2:8
            int_lit         : src0.eta:2:8..2:9
            id x            : src0.eta:2:3..2:4
            assign          : src0.eta:3:3..3:11
            uexpr           : src0.eta:3:7..3:11
            uop             : src0.eta:3:7..3:8
            char_lit        : src0.eta:3:8..3:11
            id x            : src0.eta:3:3..3:4
            assign          : src0.eta:4:3..4:9
            uexpr           : src0.eta:4:7..4:9
            uop             : src0.eta:4:7..4:8
            id x            : src0.eta:4:8..4:9
            id x            : src0.eta:4:3..4:4
            assign          : src0.eta:5:3..5:11
            uexpr           : src0.eta:5:7..5:11
            uop             : src0.eta:5:7..5:8
            pcall           : src0.eta:5:8..5:11
            id i            : src0.eta:5:8..5:9
            id x            : src0.eta:5:3..5:4
            assign          : src0.eta:6:3..6:12
            uexpr           : src0.eta:6:7..6:12
            uop             : src0.eta:6:7..6:8
            expr_idx        : src0.eta:6:8..6:12
            id y            : src0.eta:6:8..6:9
            int_lit         : src0.eta:6:10..6:11
            id x            : src0.eta:6:3..6:4
            assign          : src0.eta:7:3..7:11
            uexpr           : src0.eta:7:7..7:11
            uop             : src0.eta:7:7..7:8
            uexpr           : src0.eta:7:9..7:11
            uop             : src0.eta:7:9..7:10
            int_lit         : src0.eta:7:10..7:11
            id x            : src0.eta:7:3..7:4
            assign          : src0.eta:8:3..8:10
            int_lit         : src0.eta:8:8..8:9
            id x            : src0.eta:8:3..8:4
            assign          : src0.eta:9:3..9:27
            uexpr           : src0.eta:9:7..9:27
            uop             : src0.eta:9:7..9:8
            int_lit         : src0.eta:9:8..9:27
            id x            : src0.eta:9:3..9:4
            method          : src0.eta:12:1..14:2
            mdecl           : src0.eta:12:1..12:9
            id i            : src0.eta:12:1..12:2
            block           : src0.eta:12:10..14:2
            return          : src0.eta:13:3..13:11
            int_lit         : src0.eta:13:10..13:11
            method          : src0.eta:16:1..19:2
            mdecl           : src0.eta:16:1..16:10
            id g            : src0.eta:16:1..16:2
            decl            : src0.eta:16:3..16:9
            id x            : src0.eta:16:3..16:4
            block           : src0.eta:16:11..19:2
            assign          : src0.eta:17:3..17:12
            uexpr           : src0.eta:17:7..17:12
            uop             : src0.eta:17:7..17:8
            bool_lit        : src0.eta:17:8..17:12
            id x            : src0.eta:17:3..17:4
            assign          : src0.eta:18:3..18:13
            uexpr           : src0.eta:18:7..18:13
            uop             : src0.eta:18:7..18:8
            bool_lit        : src0.eta:18:8..18:13
            id x            : src0.eta:18:3..18:4

        "#]]
    }
}

#[test]
fn var_decls() {
    parsetest! {
        ".eta",
        indoc![r#"
            main(args: int[][]) {
                b: bool
                n: int
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl main (params (: args (array (array int)))) (returns))
                  (block
                    (decls (: b bool))
                    (decls (: n int))))))
            program         : src0.eta:1:1..4:2
            method          : src0.eta:1:1..4:2
            mdecl           : src0.eta:1:1..1:20
            id main         : src0.eta:1:1..1:5
            decl            : src0.eta:1:6..1:19
            arr_t           : src0.eta:1:17..1:19
            arr_t           : src0.eta:1:15..1:17
            id args         : src0.eta:1:6..1:10
            block           : src0.eta:1:21..4:2
            decls           : src0.eta:2:5..2:12
            decl            : src0.eta:2:5..2:12
            id b            : src0.eta:2:5..2:6
            decls           : src0.eta:3:5..3:11
            decl            : src0.eta:3:5..3:11
            id n            : src0.eta:3:5..3:6

        "#]]
    }
}

#[test]
fn while_dangling() {
    parsetest! {
        ".eta",
        indoc![r#"
            f(x: int) {
                while x > 0 if x > 5 x = x - 2 else x = x - 1
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl f (params (: x int)) (returns))
                  (block
                    (while (> x 0)
                      (if (> x 5)
                        (assign (targets x) (values (- x 2)))
                        (assign (targets x) (values (- x 1)))))))))
            program         : src0.eta:1:1..3:2
            method          : src0.eta:1:1..3:2
            mdecl           : src0.eta:1:1..1:10
            id f            : src0.eta:1:1..1:2
            decl            : src0.eta:1:3..1:9
            id x            : src0.eta:1:3..1:4
            block           : src0.eta:1:11..3:2
            while           : src0.eta:2:5..2:50
            bexpr           : src0.eta:2:11..2:16
            binop           : src0.eta:2:13..2:14
            id x            : src0.eta:2:11..2:12
            int_lit         : src0.eta:2:15..2:16
            if              : src0.eta:2:17..2:50
            bexpr           : src0.eta:2:20..2:25
            binop           : src0.eta:2:22..2:23
            id x            : src0.eta:2:20..2:21
            int_lit         : src0.eta:2:24..2:25
            assign          : src0.eta:2:26..2:35
            bexpr           : src0.eta:2:30..2:35
            binop           : src0.eta:2:32..2:33
            id x            : src0.eta:2:30..2:31
            int_lit         : src0.eta:2:34..2:35
            id x            : src0.eta:2:26..2:27
            assign          : src0.eta:2:41..2:50
            bexpr           : src0.eta:2:45..2:50
            binop           : src0.eta:2:47..2:48
            id x            : src0.eta:2:45..2:46
            int_lit         : src0.eta:2:49..2:50
            id x            : src0.eta:2:41..2:42

        "#]]
    }
}

#[test]
fn while_no_braces() {
    parsetest! {
        ".eta",
        indoc![r#"
            countdown(n: int) {
                while n > 0
                    n = n - 1
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl countdown (params (: n int)) (returns))
                  (block
                    (while (> n 0)
                      (assign (targets n) (values (- n 1))))))))
            program         : src0.eta:1:1..4:2
            method          : src0.eta:1:1..4:2
            mdecl           : src0.eta:1:1..1:18
            id countdown    : src0.eta:1:1..1:10
            decl            : src0.eta:1:11..1:17
            id n            : src0.eta:1:11..1:12
            block           : src0.eta:1:19..4:2
            while           : src0.eta:2:5..3:18
            bexpr           : src0.eta:2:11..2:16
            binop           : src0.eta:2:13..2:14
            id n            : src0.eta:2:11..2:12
            int_lit         : src0.eta:2:15..2:16
            assign          : src0.eta:3:9..3:18
            bexpr           : src0.eta:3:13..3:18
            binop           : src0.eta:3:15..3:16
            id n            : src0.eta:3:13..3:14
            int_lit         : src0.eta:3:17..3:18
            id n            : src0.eta:3:9..3:10

        "#]]
    }
}

#[test]
fn return_nothing() {
    parsetest! {
        ".eta",
        indoc![r#"
            main() {
                return
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl main (params) (returns))
                  (block
                    (return)))))
            program         : src0.eta:1:1..3:2
            method          : src0.eta:1:1..3:2
            mdecl           : src0.eta:1:1..1:7
            id main         : src0.eta:1:1..1:5
            block           : src0.eta:1:8..3:2
            return          : src0.eta:2:5..3:1

        "#]]
    }
}

#[test]
fn garbage_definition_recovery() {
    parsetest! {
        ".eta",
        indoc![r#"
            use io
            )))
        "#],
        expect![[r#"
            src0.eta: parse(recovered)
            (program
              (uses
                (use io))
              (defs
                (error)))
            program         : src0.eta:1:1..2:4
            use             : src0.eta:1:1..1:7
            id io           : src0.eta:1:5..1:7

            Error: unexpected token `)`
               ╭─[ src0.eta:2:1 ]
               │
             2 │ )))
               │ ┬  
               │ ╰── expected one of "use", ";", or "identifier"
            ───╯
        "#]]
    }
}

#[test]
fn method_trailing_junk_recovery() {
    parsetest! {
        ".eta",
        indoc![r#"
            size():int { return 0 }
            }}}
        "#],
        expect![[r#"
            src0.eta: parse(recovered)
            (program
              (uses)
              (defs
                (method (decl size (params) (returns int))
                  (block
                    (return 0)))
                (error)))
            program         : src0.eta:1:1..2:4
            method          : src0.eta:1:1..1:24
            mdecl           : src0.eta:1:1..1:11
            id size         : src0.eta:1:1..1:5
            block           : src0.eta:1:12..1:24
            return          : src0.eta:1:14..1:22
            int_lit         : src0.eta:1:21..1:22

            Error: unexpected token `}`
               ╭─[ src0.eta:2:1 ]
               │
             2 │ }}}
               │ ┬  
               │ ╰── expected one of "use", or "identifier"
            ───╯
        "#]]
    }
}

#[test]
fn malformed_statement_recovery() {
    parsetest! {
        ".eta",
        indoc![r#"
            main() { + + }
        "#],
        expect![[r#"
            src0.eta: parse(recovered)
            (program
              (uses)
              (defs
                (method (decl main (params) (returns))
                  (block
                    (error)))))
            program         : src0.eta:1:1..1:15
            method          : src0.eta:1:1..1:15
            mdecl           : src0.eta:1:1..1:7
            id main         : src0.eta:1:1..1:5
            block           : src0.eta:1:8..1:15

            Error: unexpected token `+`
               ╭─[ src0.eta:1:10 ]
               │
             1 │ main() { + + }
               │          ┬  
               │          ╰── expected one of "while", "if", "return", "_", "{", "}", or "identifier"
            ───╯
        "#]]
    }
}

#[test]
fn missing_rhs_recovery() {
    parsetest! {
        ".eta",
        indoc![r#"
            main() { x:int = }
        "#],
        expect![[r#"
            src0.eta: parse(recovered)
            (program
              (uses)
              (defs
                (method (decl main (params) (returns))
                  (block
                    (assign (targets (: x int)) (values (error)))))))
            program         : src0.eta:1:1..1:19
            method          : src0.eta:1:1..1:19
            mdecl           : src0.eta:1:1..1:7
            id main         : src0.eta:1:1..1:5
            block           : src0.eta:1:8..1:19
            assign          : src0.eta:1:10..1:18
            decl            : src0.eta:1:10..1:15
            id x            : src0.eta:1:10..1:11

            Error: unexpected token `}`
               ╭─[ src0.eta:1:18 ]
               │
             1 │ main() { x:int = }
               │                  ┬  
               │                  ╰── expected one of "length", "(", "{", "!", "-", "bool literal", "char literal", "string literal", "identifier", or "int literal"
            ───╯
        "#]]
    }
}

#[test]
fn dangling_binop_recovery() {
    parsetest! {
        ".eta",
        indoc![r#"
            main() { x:int = a[3 + ] }
        "#],
        expect![[r#"
            src0.eta: parse(recovered)
            (program
              (uses)
              (defs
                (method (decl main (params) (returns))
                  (block
                    (assign (targets (: x int)) (values (index a (+ 3 (error)))))))))
            program         : src0.eta:1:1..1:27
            method          : src0.eta:1:1..1:27
            mdecl           : src0.eta:1:1..1:7
            id main         : src0.eta:1:1..1:5
            block           : src0.eta:1:8..1:27
            assign          : src0.eta:1:10..1:25
            expr_idx        : src0.eta:1:18..1:25
            id a            : src0.eta:1:18..1:19
            bexpr           : src0.eta:1:20..1:24
            binop           : src0.eta:1:22..1:23
            int_lit         : src0.eta:1:20..1:21
            decl            : src0.eta:1:10..1:15
            id x            : src0.eta:1:10..1:11

            Error: unexpected token `]`
               ╭─[ src0.eta:1:24 ]
               │
             1 │ main() { x:int = a[3 + ] }
               │                        ┬  
               │                        ╰── expected one of "length", "(", "{", "!", "-", "bool literal", "char literal", "string literal", "identifier", or "int literal"
            ───╯
        "#]]
    }
}
