#![allow(unused_assignments)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use expect_test::expect;
use indoc::indoc;

#[macro_use]
pub mod util;

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
            program   > +++
            error     > +++

            Error: expected a method or global definition, found `+`
               ╭─[ src0.eta:1:1 ]
               │
             1 │ +++
               │ ┬  
               │ ╰── expected a method or global definition
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
            program   > compare(a: int, b: int) {
                      >     c1: bool = a < b
                      >     c2: bool = a <= b
                      > ...
                      >     c5: bool = a == b
                      >     c6: bool = a != b
                      > }
            method    > compare(a: int, b: int) {
                      >     c1: bool = a < b
                      >     c2: bool = a <= b
                      > ...
                      >     c5: bool = a == b
                      >     c6: bool = a != b
                      > }
            mdecl     > compare(a: int, b: int)
            id        > compare
            decl      > a: int
            int_t     > int
            id        > a
            decl      > b: int
            int_t     > int
            id        > b
            block     > {
                      >     c1: bool = a < b
                      >     c2: bool = a <= b
                      > ...
                      >     c5: bool = a == b
                      >     c6: bool = a != b
                      > }
            assign    > c1: bool = a < b
            bexpr     > a < b
            binop     > <
            id        > a
            id        > b
            decl      > c1: bool
            bool_t    > bool
            id        > c1
            assign    > c2: bool = a <= b
            bexpr     > a <= b
            binop     > <=
            id        > a
            id        > b
            decl      > c2: bool
            bool_t    > bool
            id        > c2
            assign    > c3: bool = a > b
            bexpr     > a > b
            binop     > >
            id        > a
            id        > b
            decl      > c3: bool
            bool_t    > bool
            id        > c3
            assign    > c4: bool = a >= b
            bexpr     > a >= b
            binop     > >=
            id        > a
            id        > b
            decl      > c4: bool
            bool_t    > bool
            id        > c4
            assign    > c5: bool = a == b
            bexpr     > a == b
            binop     > ==
            id        > a
            id        > b
            decl      > c5: bool
            bool_t    > bool
            id        > c5
            assign    > c6: bool = a != b
            bexpr     > a != b
            binop     > !=
            id        > a
            id        > b
            decl      > c6: bool
            bool_t    > bool
            id        > c6

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
            program   > f() {
                      >     a: int[][] = {{1, 2}, {3, 4}}
                      >     x: int = a[0][1]
                      >     a[1][0] = 5
                      > }
            method    > f() {
                      >     a: int[][] = {{1, 2}, {3, 4}}
                      >     x: int = a[0][1]
                      >     a[1][0] = 5
                      > }
            mdecl     > f()
            id        > f
            block     > {
                      >     a: int[][] = {{1, 2}, {3, 4}}
                      >     x: int = a[0][1]
                      >     a[1][0] = 5
                      > }
            assign    > a: int[][] = {{1, 2}, {3, 4}}
            arr_lit   > {{1, 2}, {3, 4}}
            arr_lit   > {1, 2}
            int_lit   > 1
            int_lit   > 2
            arr_lit   > {3, 4}
            int_lit   > 3
            int_lit   > 4
            decl      > a: int[][]
            arr_t     > int[][]
            arr_t     > int[]
            int_t     > int
            id        > a
            assign    > x: int = a[0][1]
            expr_idx  > a[0][1]
            expr_idx  > a[0]
            id        > a
            int_lit   > 0
            int_lit   > 1
            decl      > x: int
            int_t     > int
            id        > x
            assign    > a[1][0] = 5
            int_lit   > 5
            expr_idx  > a[1][0]
            expr_idx  > a[1]
            id        > a
            int_lit   > 1
            int_lit   > 0

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
            program   > f() {
                      >     a: int[][]
                      >     b: int[i][]
                      >     d: int[i][j]
                      > }
            method    > f() {
                      >     a: int[][]
                      >     b: int[i][]
                      >     d: int[i][j]
                      > }
            mdecl     > f()
            id        > f
            block     > {
                      >     a: int[][]
                      >     b: int[i][]
                      >     d: int[i][j]
                      > }
            decls     > a: int[][]
            decl      > a: int[][]
            arr_t     > int[][]
            arr_t     > int[]
            int_t     > int
            id        > a
            decls     > b: int[i][]
            decl      > b: int[i][]
            arr_t     > int[i][]
            arr_t     > int[i]
            int_t     > int
            id        > i
            id        > b
            decls     > d: int[i][j]
            decl      > d: int[i][j]
            arr_t     > int[i][j]
            arr_t     > int[i]
            int_t     > int
            id        > i
            id        > j
            id        > d

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
            program   > foo() {
                      >     a: int[] = {72,101,108,108,111}
                      >     a: int[] = "Hello"
                      > }
            method    > foo() {
                      >     a: int[] = {72,101,108,108,111}
                      >     a: int[] = "Hello"
                      > }
            mdecl     > foo()
            id        > foo
            block     > {
                      >     a: int[] = {72,101,108,108,111}
                      >     a: int[] = "Hello"
                      > }
            assign    > a: int[] = {72,101,108,108,111}
            arr_lit   > {72,101,108,108,111}
            int_lit   > 72
            int_lit   > 101
            int_lit   > 108
            int_lit   > 108
            int_lit   > 111
            decl      > a: int[]
            arr_t     > int[]
            int_t     > int
            id        > a
            assign    > a: int[] = "Hello"
            string_lit> "Hello"
            decl      > a: int[]
            arr_t     > int[]
            int_t     > int
            id        > a

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
            program   > foo() {
                      >     n: int = gcd(10, 2)
                      >     a: int[n]
                      > ...
                      >       a[n] = n
                      >     }
                      > }
            method    > foo() {
                      >     n: int = gcd(10, 2)
                      >     a: int[n]
                      > ...
                      >       a[n] = n
                      >     }
                      > }
            mdecl     > foo()
            id        > foo
            block     > {
                      >     n: int = gcd(10, 2)
                      >     a: int[n]
                      > ...
                      >       a[n] = n
                      >     }
                      > }
            assign    > n: int = gcd(10, 2)
            pcall     > gcd(10, 2)
            id        > gcd
            int_lit   > 10
            int_lit   > 2
            decl      > n: int
            int_t     > int
            id        > n
            decls     > a: int[n]
            decl      > a: int[n]
            arr_t     > int[n]
            int_t     > int
            id        > n
            id        > a
            while     > while (n > 0) {
                      >   n = n - 1
                      >   a[n] = n
                      > }
            bexpr     > n > 0
            binop     > >
            id        > n
            int_lit   > 0
            block     > {
                      >   n = n - 1
                      >   a[n] = n
                      > }
            assign    > n = n - 1
            bexpr     > n - 1
            binop     > -
            id        > n
            int_lit   > 1
            id        > n
            assign    > a[n] = n
            id        > n
            expr_idx  > a[n]
            id        > a
            id        > n

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
                (error)))
            program   > ===============================================================================
                      > = This is a beautiful document heading, not a xi program, but it still lexes! =
                      > ===============================================================================
            error     > ===============================================================================
                      > = This is a beautiful document heading, not a xi program, but it still lexes! =
                      > ===============================================================================

            Error: expected a method or global definition, found `==`
               ╭─[ src0.eta:1:1 ]
               │
             1 │ ===============================================================================
               │ ─┬  
               │  ╰── expected a method or global definition
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
            program   > f(a: bool, b: bool, c: bool): bool {
                      >     d: bool = a & b | c & !a
                      >     e: bool = (a | b) & (b | c)
                      >     return d == e
                      > }
            method    > f(a: bool, b: bool, c: bool): bool {
                      >     d: bool = a & b | c & !a
                      >     e: bool = (a | b) & (b | c)
                      >     return d == e
                      > }
            mdecl     > f(a: bool, b: bool, c: bool): bool
            id        > f
            decl      > a: bool
            bool_t    > bool
            id        > a
            decl      > b: bool
            bool_t    > bool
            id        > b
            decl      > c: bool
            bool_t    > bool
            id        > c
            bool_t    > bool
            block     > {
                      >     d: bool = a & b | c & !a
                      >     e: bool = (a | b) & (b | c)
                      >     return d == e
                      > }
            assign    > d: bool = a & b | c & !a
            bexpr     > a & b | c & !a
            binop     > |
            bexpr     > a & b
            binop     > &
            id        > a
            id        > b
            bexpr     > c & !a
            binop     > &
            id        > c
            uexpr     > !a
            uop       > !
            id        > a
            decl      > d: bool
            bool_t    > bool
            id        > d
            assign    > e: bool = (a | b) & (b | c)
            bexpr     > (a | b) & (b | c)
            binop     > &
            bexpr     > a | b
            binop     > |
            id        > a
            id        > b
            bexpr     > b | c
            binop     > |
            id        > b
            id        > c
            decl      > e: bool
            bool_t    > bool
            id        > e
            return    > return d == e
            bexpr     > d == e
            binop     > ==
            id        > d
            id        > e

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
            program   > f(a: int, b: int, c: int): bool {
                      >     return a < b < c
                      > }
            method    > f(a: int, b: int, c: int): bool {
                      >     return a < b < c
                      > }
            mdecl     > f(a: int, b: int, c: int): bool
            id        > f
            decl      > a: int
            int_t     > int
            id        > a
            decl      > b: int
            int_t     > int
            id        > b
            decl      > c: int
            int_t     > int
            id        > c
            bool_t    > bool
            block     > {
                      >     return a < b < c
                      > }
            return    > return a < b < c
            bexpr     > a < b < c
            binop     > <
            bexpr     > a < b
            binop     > <
            id        > a
            id        > b
            id        > c

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
            program   > classify(n: int): int {
                      >     r: int = 0
                      >     if n < 0 r = -1
                      > ...
                      >     else r = 1
                      >     return r
                      > }
            method    > classify(n: int): int {
                      >     r: int = 0
                      >     if n < 0 r = -1
                      > ...
                      >     else r = 1
                      >     return r
                      > }
            mdecl     > classify(n: int): int
            id        > classify
            decl      > n: int
            int_t     > int
            id        > n
            int_t     > int
            block     > {
                      >     r: int = 0
                      >     if n < 0 r = -1
                      > ...
                      >     else r = 1
                      >     return r
                      > }
            assign    > r: int = 0
            int_lit   > 0
            decl      > r: int
            int_t     > int
            id        > r
            if        > if n < 0 r = -1
                      > else if n == 0 r = 0
                      > else r = 1
            bexpr     > n < 0
            binop     > <
            id        > n
            int_lit   > 0
            assign    > r = -1
            uexpr     > -1
            uop       > -
            int_lit   > 1
            id        > r
            if        > if n == 0 r = 0
                      > else r = 1
            bexpr     > n == 0
            binop     > ==
            id        > n
            int_lit   > 0
            assign    > r = 0
            int_lit   > 0
            id        > r
            assign    > r = 1
            int_lit   > 1
            id        > r
            return    > return r
            id        > r

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
            interfce  > add(a: int, b: int): int
                      > subtract(a: int, b: int): int
                      > divide(a: int, b: int): int, int
                      > noop()
                      > identity(x: int[]): int[]
                      > matrix(m: int[][]): int[][]
            mdecl     > add(a: int, b: int): int
            id        > add
            decl      > a: int
            int_t     > int
            id        > a
            decl      > b: int
            int_t     > int
            id        > b
            int_t     > int
            mdecl     > subtract(a: int, b: int): int
            id        > subtract
            decl      > a: int
            int_t     > int
            id        > a
            decl      > b: int
            int_t     > int
            id        > b
            int_t     > int
            mdecl     > divide(a: int, b: int): int, int
            id        > divide
            decl      > a: int
            int_t     > int
            id        > a
            decl      > b: int
            int_t     > int
            id        > b
            int_t     > int
            int_t     > int
            mdecl     > noop()
            id        > noop
            mdecl     > identity(x: int[]): int[]
            id        > identity
            decl      > x: int[]
            arr_t     > int[]
            int_t     > int
            id        > x
            arr_t     > int[]
            int_t     > int
            mdecl     > matrix(m: int[][]): int[][]
            id        > matrix
            decl      > m: int[][]
            arr_t     > int[][]
            arr_t     > int[]
            int_t     > int
            id        > m
            arr_t     > int[][]
            arr_t     > int[]
            int_t     > int

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
            program   > f(a: bool, b: bool) {
                      >     x: int = 0
                      >     if a if b x = 1 else x = 2
                      > }
            method    > f(a: bool, b: bool) {
                      >     x: int = 0
                      >     if a if b x = 1 else x = 2
                      > }
            mdecl     > f(a: bool, b: bool)
            id        > f
            decl      > a: bool
            bool_t    > bool
            id        > a
            decl      > b: bool
            bool_t    > bool
            id        > b
            block     > {
                      >     x: int = 0
                      >     if a if b x = 1 else x = 2
                      > }
            assign    > x: int = 0
            int_lit   > 0
            decl      > x: int
            int_t     > int
            id        > x
            if        > if a if b x = 1 else x = 2
            id        > a
            if        > if b x = 1 else x = 2
            id        > b
            assign    > x = 1
            int_lit   > 1
            id        > x
            assign    > x = 2
            int_lit   > 2
            id        > x

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
            program   > f(): int {
                      >     x: int = ((1 + 2) * (3 + 4)) / ((5 - 6) * (7 - 8))
                      >     return x
                      > }
            method    > f(): int {
                      >     x: int = ((1 + 2) * (3 + 4)) / ((5 - 6) * (7 - 8))
                      >     return x
                      > }
            mdecl     > f(): int
            id        > f
            int_t     > int
            block     > {
                      >     x: int = ((1 + 2) * (3 + 4)) / ((5 - 6) * (7 - 8))
                      >     return x
                      > }
            assign    > x: int = ((1 + 2) * (3 + 4)) / ((5 - 6) * (7 - 8))
            bexpr     > ((1 + 2) * (3 + 4)) / ((5 - 6) * (7 - 8))
            binop     > /
            bexpr     > (1 + 2) * (3 + 4)
            binop     > *
            bexpr     > 1 + 2
            binop     > +
            int_lit   > 1
            int_lit   > 2
            bexpr     > 3 + 4
            binop     > +
            int_lit   > 3
            int_lit   > 4
            bexpr     > (5 - 6) * (7 - 8)
            binop     > *
            bexpr     > 5 - 6
            binop     > -
            int_lit   > 5
            int_lit   > 6
            bexpr     > 7 - 8
            binop     > -
            int_lit   > 7
            int_lit   > 8
            decl      > x: int
            int_t     > int
            id        > x
            return    > return x
            id        > x

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
            program   > f() {
                      >     a: int[] = {}
                      >     b: bool[] = {}
                      > }
            method    > f() {
                      >     a: int[] = {}
                      >     b: bool[] = {}
                      > }
            mdecl     > f()
            id        > f
            block     > {
                      >     a: int[] = {}
                      >     b: bool[] = {}
                      > }
            assign    > a: int[] = {}
            arr_lit   > {}
            decl      > a: int[]
            arr_t     > int[]
            int_t     > int
            id        > a
            assign    > b: bool[] = {}
            arr_lit   > {}
            decl      > b: bool[]
            arr_t     > bool[]
            bool_t    > bool
            id        > b

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
            program   > f() {
                      > }
            method    > f() {
                      > }
            mdecl     > f()
            id        > f
            block     > {
                      > }

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
                    (error)
                    (error)))))
            program   > main(args: int[][]) {
                      >     _[0] = true
                      > }
            method    > main(args: int[][]) {
                      >     _[0] = true
                      > }
            mdecl     > main(args: int[][])
            id        > main
            decl      > args: int[][]
            arr_t     > int[][]
            arr_t     > int[]
            int_t     > int
            id        > args
            block     > {
                      >     _[0] = true
                      > }
            error     > _
            error     > [0] = true

            Error: `_` is only allowed on the left of an assignment
               ╭─[ src0.eta:2:5 ]
               │
             2 │     _[0] = true
               │     ┬  
               │     ╰── nothing is being discarded here
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
            program   > f() {
                      >     {1,2,3}[0] = 5
                      > }
            method    > f() {
                      >     {1,2,3}[0] = 5
                      > }
            mdecl     > f()
            id        > f
            block     > {
                      >     {1,2,3}[0] = 5
                      > }
            block     > {1,2,3}
            error     > 1,2,3
            error     > [0] = 5

            Error: expected a statement, found an integer literal
               ╭─[ src0.eta:2:6 ]
               │
             2 │     {1,2,3}[0] = 5
               │      ┬  
               │      ╰── expected a statement
            ───╯
            Error: expected a statement, found `[`
               ╭─[ src0.eta:2:12 ]
               │
             2 │     {1,2,3}[0] = 5
               │            ┬  
               │            ╰── expected a statement
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
                    (error)))))
            program   > f(x: int) {
                      >     (x) = 5
                      > }
            method    > f(x: int) {
                      >     (x) = 5
                      > }
            mdecl     > f(x: int)
            id        > f
            decl      > x: int
            int_t     > int
            id        > x
            block     > {
                      >     (x) = 5
                      > }
            error     > (x) = 5

            Error: expected a statement, found `(`
               ╭─[ src0.eta:2:5 ]
               │
             2 │     (x) = 5
               │     ┬  
               │     ╰── expected a statement
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
            program   > f() {
                      >     "hello"[0] = 5
                      > }
            method    > f() {
                      >     "hello"[0] = 5
                      > }
            mdecl     > f()
            id        > f
            block     > {
                      >     "hello"[0] = 5
                      > }
            error     > "hello"[0] = 5

            Error: expected a statement, found a string literal
               ╭─[ src0.eta:2:5 ]
               │
             2 │     "hello"[0] = 5
               │     ───┬───  
               │        ╰───── expected a statement
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
            src0.eta: parse(recovered)
            (program
              (uses)
              (defs))
            program   > 

            Error: a program must contain at least one definition
               ╭─[ src0.eta:1:9 ]
               │
             1 │ 
               │         │ 
               │         ╰─ expected a method or global definition
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
            src0.eti: parse(recovered)
            (interface)
            interfce  > 

            Error: an interface must declare at least one method
               ╭─[ src0.eti:1:9 ]
               │
             1 │ 
               │         │ 
               │         ╰─ expected a method declaration
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
            src0.eta: parse(recovered)
            (program
              (uses
                (use io))
              (defs))
            program   > use io;
            use       > use io;
            id        > io

            Error: a program must contain at least one definition
               ╭─[ src0.eta:1:8 ]
               │
             1 │ use io;
               │        │ 
               │        ╰─ expected a method or global definition
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
                  (block))))
            program   > use io
                      > 
                      > f() {
            use       > use io
            id        > io
            method    > f() {
            mdecl     > f()
            id        > f
            block     > {

            Error: unclosed block
               ╭─[ src0.eta:3:5 ]
               │
             3 │ f() {
               │     ┬  
               │     ╰── this block is never closed
               │ 
               │ Note: expected `}` to close it
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
              (decl f (params) (returns)))
            interfce  > f():
            mdecl     > f():
            id        > f

            Error: expected a type, found end of file
               ╭─[ src0.eti:1:5 ]
               │
             1 │ f():
               │     │ 
               │     ╰─ expected a type
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
            program   > f() {
                      >     1 + 2
                      > }
            method    > f() {
                      >     1 + 2
                      > }
            mdecl     > f()
            id        > f
            block     > {
                      >     1 + 2
                      > }
            error     > 1 + 2

            Error: expected a statement, found an integer literal
               ╭─[ src0.eta:2:5 ]
               │
             2 │     1 + 2
               │     ┬  
               │     ╰── expected a statement
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
            program   > if true {}
            error     > if true {}

            Error: expected a method or global definition, found `if`
               ╭─[ src0.eta:1:1 ]
               │
             1 │ if true {}
               │ ─┬  
               │  ╰── expected a method or global definition
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
            program   > f() {
                      >     length: int = 5
                      > }
            method    > f() {
                      >     length: int = 5
                      > }
            mdecl     > f()
            id        > f
            block     > {
                      >     length: int = 5
                      > }
            error     > length: int = 5

            Error: expected a statement, found `length`
               ╭─[ src0.eta:2:5 ]
               │
             2 │     length: int = 5
               │     ───┬──  
               │        ╰──── expected a statement
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
                    (return 1)
                    (assign (targets (: x int)) (values 2))))))
            program   > f(): int {
                      >     return 1
                      >     x: int = 2
                      > }
            method    > f(): int {
                      >     return 1
                      >     x: int = 2
                      > }
            mdecl     > f(): int
            id        > f
            int_t     > int
            block     > {
                      >     return 1
                      >     x: int = 2
                      > }
            return    > return 1
            int_lit   > 1
            assign    > x: int = 2
            int_lit   > 2
            decl      > x: int
            int_t     > int
            id        > x

            Error: this statement is unreachable
               ╭─[ src0.eta:3:5 ]
               │
             3 │     x: int = 2
               │     ─────┬────  
               │          ╰────── unreachable
               │
               ├─[ src0.eta:3:5 ]
               │
             2 │     return 1
               │     ────┬───  
               │         ╰───── the block already returned here
               │ 
               │ Note: `return` must be the last statement in a block
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
            program   > return
            error     > return

            Error: expected a method or global definition, found `return`
               ╭─[ src0.eta:1:1 ]
               │
             1 │ return
               │ ───┬──  
               │    ╰──── expected a method or global definition
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
            src0.eta: parse(recovered)
            (program
              (uses
                (use io))
              (defs
                (method (decl f (params) (returns))
                  (block))))
            program   > f() {
                      > }
                      > use io
            use       > use io
            id        > io
            method    > f() {
                      > }
            mdecl     > f()
            id        > f
            block     > {
                      > }

            Error: `use` must come before every definition
               ╭─[ src0.eta:3:1 ]
               │
             3 │ use io
               │ ───┬──  
               │    ╰──── `use` is not allowed here
               │ 
               │ Note: move this `use` to the top of the file
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
            program   > while true {}
            error     > while true {}

            Error: expected a method or global definition, found `while`
               ╭─[ src0.eta:1:1 ]
               │
             1 │ while true {}
               │ ──┬──  
               │   ╰──── expected a method or global definition
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
            program   > use io
                      > 
                      > main(args: int[][]) {
                      > ...
                      >   c3po: int = 'x' + 47;
                      >   r2d2: int = c3po // No Han Solo
                      > }
            use       > use io
            id        > io
            method    > main(args: int[][]) {
                      >   print("Hello, Worl\x{64}!\n")
                      >   c3po: int = 'x' + 47;
                      >   r2d2: int = c3po // No Han Solo
                      > }
            mdecl     > main(args: int[][])
            id        > main
            decl      > args: int[][]
            arr_t     > int[][]
            arr_t     > int[]
            int_t     > int
            id        > args
            block     > {
                      >   print("Hello, Worl\x{64}!\n")
                      >   c3po: int = 'x' + 47;
                      >   r2d2: int = c3po // No Han Solo
                      > }
            pcall     > print("Hello, Worl\x{64}!\n")
            id        > print
            string_lit> "Hello, Worl\x{64}!\n"
            assign    > c3po: int = 'x' + 47
            bexpr     > 'x' + 47
            binop     > +
            char_lit  > 'x'
            int_lit   > 47
            decl      > c3po: int
            int_t     > int
            id        > c3po
            assign    > r2d2: int = c3po
            id        > c3po
            decl      > r2d2: int
            int_t     > int
            id        > r2d2

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
            program   > foo(): bool, int {
                      >   expr: int = 1 - 2 * 3 * -4 *
                      >   5pred: bool = true & true | false;
                      > ...
                      >   b: int[i][]
                      >   b[0] = {1, 0}
                      > }
            method    > foo(): bool, int {
                      >   expr: int = 1 - 2 * 3 * -4 *
                      >   5pred: bool = true & true | false;
                      > ...
                      >   if (pred) { expr = 59 }
                      >   return pred, expr;
                      > }
            mdecl     > foo(): bool, int
            id        > foo
            bool_t    > bool
            int_t     > int
            block     > {
                      >   expr: int = 1 - 2 * 3 * -4 *
                      >   5pred: bool = true & true | false;
                      > ...
                      >   if (pred) { expr = 59 }
                      >   return pred, expr;
                      > }
            assign    > expr: int = 1 - 2 * 3 * -4 *
                      > 5
            bexpr     > 1 - 2 * 3 * -4 *
                      > 5
            binop     > -
            int_lit   > 1
            bexpr     > 2 * 3 * -4 *
                      > 5
            binop     > *
            bexpr     > 2 * 3 * -4
            binop     > *
            bexpr     > 2 * 3
            binop     > *
            int_lit   > 2
            int_lit   > 3
            uexpr     > -4
            uop       > -
            int_lit   > 4
            int_lit   > 5
            decl      > expr: int
            int_t     > int
            id        > expr
            assign    > pred: bool = true & true | false
            bexpr     > true & true | false
            binop     > |
            bexpr     > true & true
            binop     > &
            bool_lit  > true
            bool_lit  > true
            bool_lit  > false
            decl      > pred: bool
            bool_t    > bool
            id        > pred
            if        > if (expr <= 47) { }
                      > else pred = !pred
            bexpr     > expr <= 47
            binop     > <=
            id        > expr
            int_lit   > 47
            block     > { }
            assign    > pred = !pred
            uexpr     > !pred
            uop       > !
            id        > pred
            id        > pred
            if        > if (pred) { expr = 59 }
            id        > pred
            block     > { expr = 59 }
            assign    > expr = 59
            int_lit   > 59
            id        > expr
            return    > return pred, expr;
            id        > pred
            id        > expr
            method    > bar() {
                      >   _, i: int = foo()
                      >   b: int[i][]
                      >   b[0] = {1, 0}
                      > }
            mdecl     > bar()
            id        > bar
            block     > {
                      >   _, i: int = foo()
                      >   b: int[i][]
                      >   b[0] = {1, 0}
                      > }
            assign    > _, i: int = foo()
            pcall     > foo()
            id        > foo
            discard   > _
            decl      > i: int
            int_t     > int
            id        > i
            decls     > b: int[i][]
            decl      > b: int[i][]
            arr_t     > int[i][]
            arr_t     > int[i]
            int_t     > int
            id        > i
            id        > b
            assign    > b[0] = {1, 0}
            arr_lit   > {1, 0}
            int_lit   > 1
            int_lit   > 0
            expr_idx  > b[0]
            id        > b
            int_lit   > 0

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
                (error)))
            program   > +------------------------------------+
                      > | What a beautiful, invalid program! |
                      > +------------------------------------+
            error     > +------------------------------------+
                      > | What a beautiful, invalid program! |
                      > +------------------------------------+

            Error: expected a method or global definition, found `+`
               ╭─[ src0.eta:1:1 ]
               │
             1 │ +------------------------------------+
               │ ┬  
               │ ╰── expected a method or global definition
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
            program   > x'y(a_b: int, c'd: int): int {
                      >     x'_1: int = a_b + c'd
                      >     return x'_1
                      > }
            method    > x'y(a_b: int, c'd: int): int {
                      >     x'_1: int = a_b + c'd
                      >     return x'_1
                      > }
            mdecl     > x'y(a_b: int, c'd: int): int
            id        > x'y
            decl      > a_b: int
            int_t     > int
            id        > a_b
            decl      > c'd: int
            int_t     > int
            id        > c'd
            int_t     > int
            block     > {
                      >     x'_1: int = a_b + c'd
                      >     return x'_1
                      > }
            assign    > x'_1: int = a_b + c'd
            bexpr     > a_b + c'd
            binop     > +
            id        > a_b
            id        > c'd
            decl      > x'_1: int
            int_t     > int
            id        > x'_1
            return    > return x'_1
            id        > x'_1

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
            program   > f(a: int, b: int, c: int, d: bool, e: bool): bool {
                      >     return d | e & a + b * c < a - b / c == d != e
                      > }
            method    > f(a: int, b: int, c: int, d: bool, e: bool): bool {
                      >     return d | e & a + b * c < a - b / c == d != e
                      > }
            mdecl     > f(a: int, b: int, c: int, d: bool, e: bool): bool
            id        > f
            decl      > a: int
            int_t     > int
            id        > a
            decl      > b: int
            int_t     > int
            id        > b
            decl      > c: int
            int_t     > int
            id        > c
            decl      > d: bool
            bool_t    > bool
            id        > d
            decl      > e: bool
            bool_t    > bool
            id        > e
            bool_t    > bool
            block     > {
                      >     return d | e & a + b * c < a - b / c == d != e
                      > }
            return    > return d | e & a + b * c < a - b / c == d != e
            bexpr     > d | e & a + b * c < a - b / c == d != e
            binop     > |
            id        > d
            bexpr     > e & a + b * c < a - b / c == d != e
            binop     > &
            id        > e
            bexpr     > a + b * c < a - b / c == d != e
            binop     > !=
            bexpr     > a + b * c < a - b / c == d
            binop     > ==
            bexpr     > a + b * c < a - b / c
            binop     > <
            bexpr     > a + b * c
            binop     > +
            id        > a
            bexpr     > b * c
            binop     > *
            id        > b
            id        > c
            bexpr     > a - b / c
            binop     > -
            id        > a
            bexpr     > b / c
            binop     > /
            id        > b
            id        > c
            id        > d
            id        > e

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
            program   > f(): int {
                      >     x: int = g()[0]
                      >     y: int = g(1, 2)[h()]
                      >     return x + y
                      > }
            method    > f(): int {
                      >     x: int = g()[0]
                      >     y: int = g(1, 2)[h()]
                      >     return x + y
                      > }
            mdecl     > f(): int
            id        > f
            int_t     > int
            block     > {
                      >     x: int = g()[0]
                      >     y: int = g(1, 2)[h()]
                      >     return x + y
                      > }
            assign    > x: int = g()[0]
            expr_idx  > g()[0]
            pcall     > g()
            id        > g
            int_lit   > 0
            decl      > x: int
            int_t     > int
            id        > x
            assign    > y: int = g(1, 2)[h()]
            expr_idx  > g(1, 2)[h()]
            pcall     > g(1, 2)
            id        > g
            int_lit   > 1
            int_lit   > 2
            pcall     > h()
            id        > h
            decl      > y: int
            int_t     > int
            id        > y
            return    > return x + y
            bexpr     > x + y
            binop     > +
            id        > x
            id        > y

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
            program   > main(args: int[][]) {
                      >     g()[0] = 5
                      >     g(arg)[x] = y
                      > }
            method    > main(args: int[][]) {
                      >     g()[0] = 5
                      >     g(arg)[x] = y
                      > }
            mdecl     > main(args: int[][])
            id        > main
            decl      > args: int[][]
            arr_t     > int[][]
            arr_t     > int[]
            int_t     > int
            id        > args
            block     > {
                      >     g()[0] = 5
                      >     g(arg)[x] = y
                      > }
            assign    > g()[0] = 5
            int_lit   > 5
            expr_idx  > g()[0]
            pcall     > g()
            id        > g
            int_lit   > 0
            assign    > g(arg)[x] = y
            id        > y
            expr_idx  > g(arg)[x]
            pcall     > g(arg)
            id        > g
            id        > arg
            id        > x

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
            program   > gcd(a:int, b:int):int {
                      >   while (a != 0) {
                      >     if (a<b) b = b - a
                      > ...
                      >   }
                      >   return b
                      > }
            method    > gcd(a:int, b:int):int {
                      >   while (a != 0) {
                      >     if (a<b) b = b - a
                      > ...
                      >   }
                      >   return b
                      > }
            mdecl     > gcd(a:int, b:int):int
            id        > gcd
            decl      > a:int
            int_t     > int
            id        > a
            decl      > b:int
            int_t     > int
            id        > b
            int_t     > int
            block     > {
                      >   while (a != 0) {
                      >     if (a<b) b = b - a
                      > ...
                      >   }
                      >   return b
                      > }
            while     > while (a != 0) {
                      >   if (a<b) b = b - a
                      >   else a = a - b
                      > }
            bexpr     > a != 0
            binop     > !=
            id        > a
            int_lit   > 0
            block     > {
                      >   if (a<b) b = b - a
                      >   else a = a - b
                      > }
            if        > if (a<b) b = b - a
                      > else a = a - b
            bexpr     > a<b
            binop     > <
            id        > a
            id        > b
            assign    > b = b - a
            bexpr     > b - a
            binop     > -
            id        > b
            id        > a
            id        > b
            assign    > a = a - b
            bexpr     > a - b
            binop     > -
            id        > a
            id        > b
            id        > a
            return    > return b
            id        > b

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
            program   > f(x:int, y:bool, z:int[]) {
                      >   x = 0
                      >   y = true
                      > ...
                      > g1():int {
                      >   return 0
                      > }
            method    > f(x:int, y:bool, z:int[]) {
                      >   x = 0
                      >   y = true
                      > ...
                      >   x = 0 + 0
                      >   x = (0)
                      > }
            mdecl     > f(x:int, y:bool, z:int[])
            id        > f
            decl      > x:int
            int_t     > int
            id        > x
            decl      > y:bool
            bool_t    > bool
            id        > y
            decl      > z:int[]
            arr_t     > int[]
            int_t     > int
            id        > z
            block     > {
                      >   x = 0
                      >   y = true
                      > ...
                      >   x = 0 + 0
                      >   x = (0)
                      > }
            assign    > x = 0
            int_lit   > 0
            id        > x
            assign    > y = true
            bool_lit  > true
            id        > y
            assign    > x = x
            id        > x
            id        > x
            assign    > z = {1, 2, 3}
            arr_lit   > {1, 2, 3}
            int_lit   > 1
            int_lit   > 2
            int_lit   > 3
            id        > z
            assign    > x = g1()
            pcall     > g1()
            id        > g1
            id        > x
            assign    > x = z[0]
            expr_idx  > z[0]
            id        > z
            int_lit   > 0
            id        > x
            assign    > x = -1
            uexpr     > -1
            uop       > -
            int_lit   > 1
            id        > x
            assign    > x = 0 + 0
            bexpr     > 0 + 0
            binop     > +
            int_lit   > 0
            int_lit   > 0
            id        > x
            assign    > x = (0)
            int_lit   > 0
            id        > x
            gdecl     > x: int;
            int_t     > int
            id        > x
            method    > g1():int {
                      >   return 0
                      > }
            mdecl     > g1():int
            id        > g1
            int_t     > int
            block     > {
                      >   return 0
                      > }
            return    > return 0
            int_lit   > 0

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
            program   > f(x:int[], y:int) {
                      >   x[0] = 0
                      >   x[y] = 0
                      > ...
                      >   return 0
                      > }
                      > booleanboi :bool;
            method    > f(x:int[], y:int) {
                      >   x[0] = 0
                      >   x[y] = 0
                      > ...
                      >   x[0+0] = 0
                      >   x[(0)] = 0
                      > }
            mdecl     > f(x:int[], y:int)
            id        > f
            decl      > x:int[]
            arr_t     > int[]
            int_t     > int
            id        > x
            decl      > y:int
            int_t     > int
            id        > y
            block     > {
                      >   x[0] = 0
                      >   x[y] = 0
                      > ...
                      >   x[0+0] = 0
                      >   x[(0)] = 0
                      > }
            assign    > x[0] = 0
            int_lit   > 0
            expr_idx  > x[0]
            id        > x
            int_lit   > 0
            assign    > x[y] = 0
            int_lit   > 0
            expr_idx  > x[y]
            id        > x
            id        > y
            assign    > x[{1,2,3}[0]] = 0
            int_lit   > 0
            expr_idx  > x[{1,2,3}[0]]
            id        > x
            expr_idx  > {1,2,3}[0]
            arr_lit   > {1,2,3}
            int_lit   > 1
            int_lit   > 2
            int_lit   > 3
            int_lit   > 0
            assign    > x[g()] = 0
            int_lit   > 0
            expr_idx  > x[g()]
            id        > x
            pcall     > g()
            id        > g
            assign    > x[x[0]] = 0
            int_lit   > 0
            expr_idx  > x[x[0]]
            id        > x
            expr_idx  > x[0]
            id        > x
            int_lit   > 0
            assign    > x[-1] = 0
            int_lit   > 0
            expr_idx  > x[-1]
            id        > x
            uexpr     > -1
            uop       > -
            int_lit   > 1
            assign    > x[0+0] = 0
            int_lit   > 0
            expr_idx  > x[0+0]
            id        > x
            bexpr     > 0+0
            binop     > +
            int_lit   > 0
            int_lit   > 0
            assign    > x[(0)] = 0
            int_lit   > 0
            expr_idx  > x[(0)]
            id        > x
            int_lit   > 0
            method    > g():int {
                      >   return 0
                      > }
            mdecl     > g():int
            id        > g
            int_t     > int
            block     > {
                      >   return 0
                      > }
            return    > return 0
            int_lit   > 0
            gdecl     > booleanboi :bool;
            bool_t    > bool
            id        > booleanboi

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
            program   > f(x:int[][], y:int, z:int) {
                      >   x[0][1] = 0
                      >   x[y][z] = 0
                      > ...
                      > g2():int {
                      >   return 0
                      > }
            method    > f(x:int[][], y:int, z:int) {
                      >   x[0][1] = 0
                      >   x[y][z] = 0
                      > ...
                      >   x[0+0][0-0] = 0
                      >   x[(0)][(1)] = 0
                      > }
            mdecl     > f(x:int[][], y:int, z:int)
            id        > f
            decl      > x:int[][]
            arr_t     > int[][]
            arr_t     > int[]
            int_t     > int
            id        > x
            decl      > y:int
            int_t     > int
            id        > y
            decl      > z:int
            int_t     > int
            id        > z
            block     > {
                      >   x[0][1] = 0
                      >   x[y][z] = 0
                      > ...
                      >   x[0+0][0-0] = 0
                      >   x[(0)][(1)] = 0
                      > }
            assign    > x[0][1] = 0
            int_lit   > 0
            expr_idx  > x[0][1]
            expr_idx  > x[0]
            id        > x
            int_lit   > 0
            int_lit   > 1
            assign    > x[y][z] = 0
            int_lit   > 0
            expr_idx  > x[y][z]
            expr_idx  > x[y]
            id        > x
            id        > y
            id        > z
            assign    > x[{1,2,3}[0]][{1,2,3}[1]] = 0
            int_lit   > 0
            expr_idx  > x[{1,2,3}[0]][{1,2,3}[1]]
            expr_idx  > x[{1,2,3}[0]]
            id        > x
            expr_idx  > {1,2,3}[0]
            arr_lit   > {1,2,3}
            int_lit   > 1
            int_lit   > 2
            int_lit   > 3
            int_lit   > 0
            expr_idx  > {1,2,3}[1]
            arr_lit   > {1,2,3}
            int_lit   > 1
            int_lit   > 2
            int_lit   > 3
            int_lit   > 1
            assign    > x[g1()][g2()] = 0
            int_lit   > 0
            expr_idx  > x[g1()][g2()]
            expr_idx  > x[g1()]
            id        > x
            pcall     > g1()
            id        > g1
            pcall     > g2()
            id        > g2
            assign    > x[x[0][1]][x[2][3]] = 0
            int_lit   > 0
            expr_idx  > x[x[0][1]][x[2][3]]
            expr_idx  > x[x[0][1]]
            id        > x
            expr_idx  > x[0][1]
            expr_idx  > x[0]
            id        > x
            int_lit   > 0
            int_lit   > 1
            expr_idx  > x[2][3]
            expr_idx  > x[2]
            id        > x
            int_lit   > 2
            int_lit   > 3
            assign    > x[-1][-2] = 0
            int_lit   > 0
            expr_idx  > x[-1][-2]
            expr_idx  > x[-1]
            id        > x
            uexpr     > -1
            uop       > -
            int_lit   > 1
            uexpr     > -2
            uop       > -
            int_lit   > 2
            assign    > x[0+0][0-0] = 0
            int_lit   > 0
            expr_idx  > x[0+0][0-0]
            expr_idx  > x[0+0]
            id        > x
            bexpr     > 0+0
            binop     > +
            int_lit   > 0
            int_lit   > 0
            bexpr     > 0-0
            binop     > -
            int_lit   > 0
            int_lit   > 0
            assign    > x[(0)][(1)] = 0
            int_lit   > 0
            expr_idx  > x[(0)][(1)]
            expr_idx  > x[(0)]
            id        > x
            int_lit   > 0
            int_lit   > 1
            gdecl     > heyitsaboolean :bool = false;
            bool_t    > bool
            id        > heyitsaboolean
            bool_lit  > false
            method    > g1():int {
                      >   return 0
                      > }
            mdecl     > g1():int
            id        > g1
            int_t     > int
            block     > {
                      >   return 0
                      > }
            return    > return 0
            int_lit   > 0
            method    > g2():int {
                      >   return 0
                      > }
            mdecl     > g2():int
            id        > g2
            int_t     > int
            block     > {
                      >   return 0
                      > }
            return    > return 0
            int_lit   > 0

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
            program   > f1() {
                      >   x:int
                      > }
                      > ...
                      >   x11:int;
                      >   x12:int
                      > }
            method    > f1() {
                      >   x:int
                      > }
            mdecl     > f1()
            id        > f1
            block     > {
                      >   x:int
                      > }
            decls     > x:int
            decl      > x:int
            int_t     > int
            id        > x
            gdecl     > thebigglobarray :int[];
            arr_t     > int[]
            int_t     > int
            id        > thebigglobarray
            method    > f2() {
                      >   x:int;
                      > }
            mdecl     > f2()
            id        > f2
            block     > {
                      >   x:int;
                      > }
            decls     > x:int
            decl      > x:int
            int_t     > int
            id        > x
            method    > f3() {
                      >   x01:int;
                      >   x02:int
                      > ...
                      >   x11:int;
                      >   x12:int
                      > }
            mdecl     > f3()
            id        > f3
            block     > {
                      >   x01:int;
                      >   x02:int
                      > ...
                      >   x11:int;
                      >   x12:int
                      > }
            decls     > x01:int
            decl      > x01:int
            int_t     > int
            id        > x01
            decls     > x02:int
            decl      > x02:int
            int_t     > int
            id        > x02
            decls     > x03:int
            decl      > x03:int
            int_t     > int
            id        > x03
            decls     > x04:int
            decl      > x04:int
            int_t     > int
            id        > x04
            decls     > x05:int
            decl      > x05:int
            int_t     > int
            id        > x05
            decls     > x06:int
            decl      > x06:int
            int_t     > int
            id        > x06
            decls     > x07:int
            decl      > x07:int
            int_t     > int
            id        > x07
            decls     > x08:int
            decl      > x08:int
            int_t     > int
            id        > x08
            decls     > x09:int
            decl      > x09:int
            int_t     > int
            id        > x09
            decls     > x10:int
            decl      > x10:int
            int_t     > int
            id        > x10
            decls     > x11:int
            decl      > x11:int
            int_t     > int
            id        > x11
            decls     > x12:int
            decl      > x12:int
            int_t     > int
            id        > x12

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
            program   > f1(): int {
                      >   return 0;
                      > }
                      > ...
                      >   x12:int
                      >   return 0;
                      > }
            method    > f1(): int {
                      >   return 0;
                      > }
            mdecl     > f1(): int
            id        > f1
            int_t     > int
            block     > {
                      >   return 0;
                      > }
            return    > return 0;
            int_lit   > 0
            method    > f2(): int {
                      >   x:int
                      >   return 0
                      > }
            mdecl     > f2(): int
            id        > f2
            int_t     > int
            block     > {
                      >   x:int
                      >   return 0
                      > }
            decls     > x:int
            decl      > x:int
            int_t     > int
            id        > x
            return    > return 0
            int_lit   > 0
            gdecl     > arrayisquitedeep :int[][][][][][];
            arr_t     > int[][][][][][]
            arr_t     > int[][][][][]
            arr_t     > int[][][][]
            arr_t     > int[][][]
            arr_t     > int[][]
            arr_t     > int[]
            int_t     > int
            id        > arrayisquitedeep
            method    > f3(): int {
                      >   x:int
                      >   return 0;
                      > }
            mdecl     > f3(): int
            id        > f3
            int_t     > int
            block     > {
                      >   x:int
                      >   return 0;
                      > }
            decls     > x:int
            decl      > x:int
            int_t     > int
            id        > x
            return    > return 0;
            int_lit   > 0
            method    > f4(): int {
                      >   x:int;
                      >   return 0
                      > }
            mdecl     > f4(): int
            id        > f4
            int_t     > int
            block     > {
                      >   x:int;
                      >   return 0
                      > }
            decls     > x:int
            decl      > x:int
            int_t     > int
            id        > x
            return    > return 0
            int_lit   > 0
            method    > f5(): int {
                      >   x:int;
                      >   return 0;
                      > }
            mdecl     > f5(): int
            id        > f5
            int_t     > int
            block     > {
                      >   x:int;
                      >   return 0;
                      > }
            decls     > x:int
            decl      > x:int
            int_t     > int
            id        > x
            return    > return 0;
            int_lit   > 0
            method    > f6(): int {
                      >   x01:int;
                      >   x02:int
                      > ...
                      >   x12:int
                      >   return 0
                      > }
            mdecl     > f6(): int
            id        > f6
            int_t     > int
            block     > {
                      >   x01:int;
                      >   x02:int
                      > ...
                      >   x12:int
                      >   return 0
                      > }
            decls     > x01:int
            decl      > x01:int
            int_t     > int
            id        > x01
            decls     > x02:int
            decl      > x02:int
            int_t     > int
            id        > x02
            decls     > x03:int
            decl      > x03:int
            int_t     > int
            id        > x03
            decls     > x04:int
            decl      > x04:int
            int_t     > int
            id        > x04
            decls     > x05:int
            decl      > x05:int
            int_t     > int
            id        > x05
            decls     > x06:int
            decl      > x06:int
            int_t     > int
            id        > x06
            decls     > x07:int
            decl      > x07:int
            int_t     > int
            id        > x07
            decls     > x08:int
            decl      > x08:int
            int_t     > int
            id        > x08
            decls     > x09:int
            decl      > x09:int
            int_t     > int
            id        > x09
            decls     > x10:int
            decl      > x10:int
            int_t     > int
            id        > x10
            decls     > x11:int
            decl      > x11:int
            int_t     > int
            id        > x11
            decls     > x12:int
            decl      > x12:int
            int_t     > int
            id        > x12
            return    > return 0
            int_lit   > 0
            method    > f7(): int {
                      >   x01:int;
                      >   x02:int
                      > ...
                      >   x12:int
                      >   return 0;
                      > }
            mdecl     > f7(): int
            id        > f7
            int_t     > int
            block     > {
                      >   x01:int;
                      >   x02:int
                      > ...
                      >   x12:int
                      >   return 0;
                      > }
            decls     > x01:int
            decl      > x01:int
            int_t     > int
            id        > x01
            decls     > x02:int
            decl      > x02:int
            int_t     > int
            id        > x02
            decls     > x03:int
            decl      > x03:int
            int_t     > int
            id        > x03
            decls     > x04:int
            decl      > x04:int
            int_t     > int
            id        > x04
            decls     > x05:int
            decl      > x05:int
            int_t     > int
            id        > x05
            decls     > x06:int
            decl      > x06:int
            int_t     > int
            id        > x06
            decls     > x07:int
            decl      > x07:int
            int_t     > int
            id        > x07
            decls     > x08:int
            decl      > x08:int
            int_t     > int
            id        > x08
            decls     > x09:int
            decl      > x09:int
            int_t     > int
            id        > x09
            decls     > x10:int
            decl      > x10:int
            int_t     > int
            id        > x10
            decls     > x11:int
            decl      > x11:int
            int_t     > int
            id        > x11
            decls     > x12:int
            decl      > x12:int
            int_t     > int
            id        > x12
            return    > return 0;
            int_lit   > 0

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
            src0.eta: parse(recovered)
            (program
              (uses)
              (defs
                (global x (array int) Spur(2))
                (method (decl foo (params (: x int)) (returns int))
                  (block
                    (return x)))))
            program   > x :int[] = "whoops";
                      > 
                      > foo(x :int) :int
                      > {
                      >    return x;
                      > }
            gdecl     > x :int[] = "whoops";
            arr_t     > int[]
            int_t     > int
            id        > x
            string_lit> "whoops"
            method    > foo(x :int) :int
                      > {
                      >    return x;
                      > }
            mdecl     > foo(x :int) :int
            id        > foo
            decl      > x :int
            int_t     > int
            id        > x
            int_t     > int
            block     > {
                      >    return x;
                      > }
            return    > return x;
            id        > x

            Error: a global array cannot be initialised where it is declared
               ╭─[ src0.eta:1:10 ]
               │
             1 │ x :int[] = "whoops";
               │    ──┬── ─────┬────  
               │      ╰─────────────── this is an array type
               │               │      
               │               ╰────── initialisation happens here
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
            src0.eta: parse(recovered)
            (program
              (uses)
              (defs
                (global x (array int 4))
                (method (decl foo (params (: x int)) (returns int))
                  (block
                    (return x)))))
            program   > x :int[4];
                      > 
                      > foo(x :int) :int
                      > {
                      >    return x;
                      > }
            gdecl     > x :int[4];
            arr_t     > int[4]
            int_t     > int
            int_lit   > 4
            id        > x
            method    > foo(x :int) :int
                      > {
                      >    return x;
                      > }
            mdecl     > foo(x :int) :int
            id        > foo
            decl      > x :int
            int_t     > int
            id        > x
            int_t     > int
            block     > {
                      >    return x;
                      > }
            return    > return x;
            id        > x

            Error: a global array cannot be given a size
               ╭─[ src0.eta:1:7 ]
               │
             1 │ x :int[4];
               │       ─┬─  
               │        ╰─── size not allowed here
               │ 
               │ Note: globals are declared unsized: `x: int[]`
            ───╯
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
            src0.eta: parse(recovered)
            (program
              (uses)
              (defs
                (global x (array bool) (array true false))
                (method (decl foo (params (: x int)) (returns int))
                  (block
                    (return x)))))
            program   > x :bool[] = {true, false,};
                      > 
                      > foo(x :int) :int
                      > {
                      >    return x;
                      > }
            gdecl     > x :bool[] = {true, false,};
            arr_t     > bool[]
            bool_t    > bool
            id        > x
            arr_lit   > {true, false,}
            bool_lit  > true
            bool_lit  > false
            method    > foo(x :int) :int
                      > {
                      >    return x;
                      > }
            mdecl     > foo(x :int) :int
            id        > foo
            decl      > x :int
            int_t     > int
            id        > x
            int_t     > int
            block     > {
                      >    return x;
                      > }
            return    > return x;
            id        > x

            Error: a global array cannot be initialised where it is declared
               ╭─[ src0.eta:1:11 ]
               │
             1 │ x :bool[] = {true, false,};
               │    ───┬── ────────┬───────  
               │       ╰───────────────────── this is an array type
               │                   │         
               │                   ╰───────── initialisation happens here
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
            program   > count: int = 42
                      > flag: bool = true
                      > data: int[]
                      > ...
                      > 
                      > flag2: bool = true;
                      > m: int;
            gdecl     > count: int = 42
            int_t     > int
            id        > count
            int_lit   > 42
            gdecl     > flag: bool = true
            bool_t    > bool
            id        > flag
            bool_lit  > true
            gdecl     > data: int[]
            arr_t     > int[]
            int_t     > int
            id        > data
            gdecl     > n: int
            int_t     > int
            id        > n
            gdecl     > flag2: bool = true;
            bool_t    > bool
            id        > flag2
            bool_lit  > true
            gdecl     > m: int;
            int_t     > int
            id        > m
            method    > reset() {
                      >     count = 0
                      >     flag = false
                      > }
            mdecl     > reset()
            id        > reset
            block     > {
                      >     count = 0
                      >     flag = false
                      > }
            assign    > count = 0
            int_lit   > 0
            id        > count
            assign    > flag = false
            bool_lit  > false
            id        > flag
            gdecl     > count: int = 42
            int_t     > int
            id        > count
            int_lit   > 42
            gdecl     > flag: bool = true
            bool_t    > bool
            id        > flag
            bool_lit  > true
            gdecl     > data: int[]
            arr_t     > int[]
            int_t     > int
            id        > data
            gdecl     > n: int
            int_t     > int
            id        > n
            gdecl     > flag2: bool = true;
            bool_t    > bool
            id        > flag2
            bool_lit  > true
            gdecl     > m: int;
            int_t     > int
            id        > m

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
            program   > x: int = 1
                      > f() {}
                      > y: int = 2
                      > g() {}
                      > z: bool = true
            gdecl     > x: int = 1
            int_t     > int
            id        > x
            int_lit   > 1
            method    > f() {}
            mdecl     > f()
            id        > f
            block     > {}
            gdecl     > y: int = 2
            int_t     > int
            id        > y
            int_lit   > 2
            method    > g() {}
            mdecl     > g()
            id        > g
            block     > {}
            gdecl     > z: bool = true
            bool_t    > bool
            id        > z
            bool_lit  > true

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
            program   > ops(a: int, b: int): int, int {
                      >     h: int = a *>> b
                      >     m: int = a % b
                      >     return h, m
                      > }
            method    > ops(a: int, b: int): int, int {
                      >     h: int = a *>> b
                      >     m: int = a % b
                      >     return h, m
                      > }
            mdecl     > ops(a: int, b: int): int, int
            id        > ops
            decl      > a: int
            int_t     > int
            id        > a
            decl      > b: int
            int_t     > int
            id        > b
            int_t     > int
            int_t     > int
            block     > {
                      >     h: int = a *>> b
                      >     m: int = a % b
                      >     return h, m
                      > }
            assign    > h: int = a *>> b
            bexpr     > a *>> b
            binop     > *>>
            id        > a
            id        > b
            decl      > h: int
            int_t     > int
            id        > h
            assign    > m: int = a % b
            bexpr     > a % b
            binop     > %
            id        > a
            id        > b
            decl      > m: int
            int_t     > int
            id        > m
            return    > return h, m
            id        > h
            id        > m

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
            program   > f(x: int) {
                      >     if x > 0 x = x - 1
                      >     if x == 0 x = 0
                      > }
            method    > f(x: int) {
                      >     if x > 0 x = x - 1
                      >     if x == 0 x = 0
                      > }
            mdecl     > f(x: int)
            id        > f
            decl      > x: int
            int_t     > int
            id        > x
            block     > {
                      >     if x > 0 x = x - 1
                      >     if x == 0 x = 0
                      > }
            if        > if x > 0 x = x - 1
            bexpr     > x > 0
            binop     > >
            id        > x
            int_lit   > 0
            assign    > x = x - 1
            bexpr     > x - 1
            binop     > -
            id        > x
            int_lit   > 1
            id        > x
            if        > if x == 0 x = 0
            bexpr     > x == 0
            binop     > ==
            id        > x
            int_lit   > 0
            assign    > x = 0
            int_lit   > 0
            id        > x

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
            program   > sort(a: int[]) {
                      >   i:int = 0
                      >   n:int = length(a)
                      > ...
                      >       i = i+1
                      >   }
                      > }
            method    > sort(a: int[]) {
                      >   i:int = 0
                      >   n:int = length(a)
                      > ...
                      >       i = i+1
                      >   }
                      > }
            mdecl     > sort(a: int[])
            id        > sort
            decl      > a: int[]
            arr_t     > int[]
            int_t     > int
            id        > a
            block     > {
                      >   i:int = 0
                      >   n:int = length(a)
                      > ...
                      >       i = i+1
                      >   }
                      > }
            assign    > i:int = 0
            int_lit   > 0
            decl      > i:int
            int_t     > int
            id        > i
            assign    > n:int = length(a)
            length    > length(a)
            id        > a
            decl      > n:int
            int_t     > int
            id        > n
            while     > while (i < n) {
                      >     j:int = i
                      >     while (j > 0) {
                      > ...
                      >     }
                      >     i = i+1
                      > }
            bexpr     > i < n
            binop     > <
            id        > i
            id        > n
            block     > {
                      >     j:int = i
                      >     while (j > 0) {
                      > ...
                      >     }
                      >     i = i+1
                      > }
            assign    > j:int = i
            id        > i
            decl      > j:int
            int_t     > int
            id        > j
            while     > while (j > 0) {
                      >   if (a[j-1] > a[j]) {
                      >       swap:int = a[j]
                      > ...
                      >   }
                      >   j = j-1
                      > }
            bexpr     > j > 0
            binop     > >
            id        > j
            int_lit   > 0
            block     > {
                      >   if (a[j-1] > a[j]) {
                      >       swap:int = a[j]
                      > ...
                      >   }
                      >   j = j-1
                      > }
            if        > if (a[j-1] > a[j]) {
                      >     swap:int = a[j]
                      >     a[j] = a[j-1]
                      >     a[j-1] = swap
                      > }
            bexpr     > a[j-1] > a[j]
            binop     > >
            expr_idx  > a[j-1]
            id        > a
            bexpr     > j-1
            binop     > -
            id        > j
            int_lit   > 1
            expr_idx  > a[j]
            id        > a
            id        > j
            block     > {
                      >     swap:int = a[j]
                      >     a[j] = a[j-1]
                      >     a[j-1] = swap
                      > }
            assign    > swap:int = a[j]
            expr_idx  > a[j]
            id        > a
            id        > j
            decl      > swap:int
            int_t     > int
            id        > swap
            assign    > a[j] = a[j-1]
            expr_idx  > a[j-1]
            id        > a
            bexpr     > j-1
            binop     > -
            id        > j
            int_lit   > 1
            expr_idx  > a[j]
            id        > a
            id        > j
            assign    > a[j-1] = swap
            id        > swap
            expr_idx  > a[j-1]
            id        > a
            bexpr     > j-1
            binop     > -
            id        > j
            int_lit   > 1
            assign    > j = j-1
            bexpr     > j-1
            binop     > -
            id        > j
            int_lit   > 1
            id        > j
            assign    > i = i+1
            bexpr     > i+1
            binop     > +
            id        > i
            int_lit   > 1
            id        > i

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
            interfce  > print(str: int[])     // Print a string to standard output.
                      > println(str: int[])   // Print a string to standard output, followed by a newline.
                      > readln() : int[]      // Read from standard input until a newline.
                      > getchar() : int       // Read a single character from standard input.
                      >                       // Returns -1 if the end of input has been reached.
                      > eof() : bool
            mdecl     > print(str: int[])
            id        > print
            decl      > str: int[]
            arr_t     > int[]
            int_t     > int
            id        > str
            mdecl     > println(str: int[])
            id        > println
            decl      > str: int[]
            arr_t     > int[]
            int_t     > int
            id        > str
            mdecl     > readln() : int[]
            id        > readln
            arr_t     > int[]
            int_t     > int
            mdecl     > getchar() : int
            id        > getchar
            int_t     > int
            mdecl     > eof() : bool
            id        > eof
            bool_t    > bool

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
            program   > main(args: int[][]) {
                      >     x = length(arr)
                      > }
            method    > main(args: int[][]) {
                      >     x = length(arr)
                      > }
            mdecl     > main(args: int[][])
            id        > main
            decl      > args: int[][]
            arr_t     > int[][]
            arr_t     > int[]
            int_t     > int
            id        > args
            block     > {
                      >     x = length(arr)
                      > }
            assign    > x = length(arr)
            length    > length(arr)
            id        > arr
            id        > x

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
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl main (params (: args (array (array int)))) (returns))
                  (block
                    (assign (targets (: x int)) (values (error)))))))
            program   > main(args: int[][]) {
                      >     x:int = '\q'
                      > }
            method    > main(args: int[][]) {
                      >     x:int = '\q'
                      > }
            mdecl     > main(args: int[][])
            id        > main
            decl      > args: int[][]
            arr_t     > int[][]
            arr_t     > int[]
            int_t     > int
            id        > args
            block     > {
                      >     x:int = '\q'
                      > }
            assign    > x:int = '\q'
            error     > 
            decl      > x:int
            int_t     > int
            id        > x

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
            program   > foo() {
                      >     a: int[][]
                      >     b: int[3][4]
                      > ...
                      >     c[0] = b[0]; c[1] = b[1]; c[2] = b[2]
                      >     d: int[][] = {{1, 0}, {0, 1}}
                      > }
            method    > foo() {
                      >     a: int[][]
                      >     b: int[3][4]
                      > ...
                      >     c[0] = b[0]; c[1] = b[1]; c[2] = b[2]
                      >     d: int[][] = {{1, 0}, {0, 1}}
                      > }
            mdecl     > foo()
            id        > foo
            block     > {
                      >     a: int[][]
                      >     b: int[3][4]
                      > ...
                      >     c[0] = b[0]; c[1] = b[1]; c[2] = b[2]
                      >     d: int[][] = {{1, 0}, {0, 1}}
                      > }
            decls     > a: int[][]
            decl      > a: int[][]
            arr_t     > int[][]
            arr_t     > int[]
            int_t     > int
            id        > a
            decls     > b: int[3][4]
            decl      > b: int[3][4]
            arr_t     > int[3][4]
            arr_t     > int[3]
            int_t     > int
            int_lit   > 3
            int_lit   > 4
            id        > b
            assign    > a = b
            id        > b
            id        > a
            decls     > c: int[3][]
            decl      > c: int[3][]
            arr_t     > int[3][]
            arr_t     > int[3]
            int_t     > int
            int_lit   > 3
            id        > c
            assign    > c[0] = b[0]
            expr_idx  > b[0]
            id        > b
            int_lit   > 0
            expr_idx  > c[0]
            id        > c
            int_lit   > 0
            assign    > c[1] = b[1]
            expr_idx  > b[1]
            id        > b
            int_lit   > 1
            expr_idx  > c[1]
            id        > c
            int_lit   > 1
            assign    > c[2] = b[2]
            expr_idx  > b[2]
            id        > b
            int_lit   > 2
            expr_idx  > c[2]
            id        > c
            int_lit   > 2
            assign    > d: int[][] = {{1, 0}, {0, 1}}
            arr_lit   > {{1, 0}, {0, 1}}
            arr_lit   > {1, 0}
            int_lit   > 1
            int_lit   > 0
            arr_lit   > {0, 1}
            int_lit   > 0
            int_lit   > 1
            decl      > d: int[][]
            arr_t     > int[][]
            arr_t     > int[]
            int_t     > int
            id        > d

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
            program   > triple(x: int): int, int, int {
                      >     a: int = x + 1
                      >     b: int = x + 2
                      >     return x, a, b
                      > }
            method    > triple(x: int): int, int, int {
                      >     a: int = x + 1
                      >     b: int = x + 2
                      >     return x, a, b
                      > }
            mdecl     > triple(x: int): int, int, int
            id        > triple
            decl      > x: int
            int_t     > int
            id        > x
            int_t     > int
            int_t     > int
            int_t     > int
            block     > {
                      >     a: int = x + 1
                      >     b: int = x + 2
                      >     return x, a, b
                      > }
            assign    > a: int = x + 1
            bexpr     > x + 1
            binop     > +
            id        > x
            int_lit   > 1
            decl      > a: int
            int_t     > int
            id        > a
            assign    > b: int = x + 2
            bexpr     > x + 2
            binop     > +
            id        > x
            int_lit   > 2
            decl      > b: int
            int_t     > int
            id        > b
            return    > return x, a, b
            id        > x
            id        > a
            id        > b

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
            program   > use io
                      > use conv;
                      > 
                      > main(args: int[][]) {
                      >     println(unparseInt(42))
                      > }
            use       > use io
            id        > io
            use       > use conv;
            id        > conv
            method    > main(args: int[][]) {
                      >     println(unparseInt(42))
                      > }
            mdecl     > main(args: int[][])
            id        > main
            decl      > args: int[][]
            arr_t     > int[][]
            arr_t     > int[]
            int_t     > int
            id        > args
            block     > {
                      >     println(unparseInt(42))
                      > }
            pcall     > println(unparseInt(42))
            id        > println
            pcall     > unparseInt(42)
            id        > unparseInt
            int_lit   > 42

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
            program   > main(args: int[][]) {
                      >     x:int, z, y:int = 1, 2
                      >     a:int, b:bool = 3, true
                      > }
            method    > main(args: int[][]) {
                      >     x:int, z, y:int = 1, 2
                      >     a:int, b:bool = 3, true
                      > }
            mdecl     > main(args: int[][])
            id        > main
            decl      > args: int[][]
            arr_t     > int[][]
            arr_t     > int[]
            int_t     > int
            id        > args
            block     > {
                      >     x:int, z, y:int = 1, 2
                      >     a:int, b:bool = 3, true
                      > }
            assign    > x:int, z, y:int = 1, 2
            int_lit   > 1
            int_lit   > 2
            decl      > x:int
            int_t     > int
            id        > x
            id        > z
            decl      > y:int
            int_t     > int
            id        > y
            assign    > a:int, b:bool = 3, true
            int_lit   > 3
            bool_lit  > true
            decl      > a:int
            int_t     > int
            id        > a
            decl      > b:bool
            bool_t    > bool
            id        > b

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
            program   > foo() {
                      >     _, _, _ = g()
                      >     _ = h()
                      > }
            method    > foo() {
                      >     _, _, _ = g()
                      >     _ = h()
                      > }
            mdecl     > foo()
            id        > foo
            block     > {
                      >     _, _, _ = g()
                      >     _ = h()
                      > }
            assign    > _, _, _ = g()
            pcall     > g()
            id        > g
            discard   > _
            discard   > _
            discard   > _
            assign    > _ = h()
            pcall     > h()
            id        > h
            discard   > _

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
            program   > n: int = -1
                      > x: int = -9223372036854775808
                      > f() {}
            gdecl     > n: int = -1
            int_t     > int
            id        > n
            int_lit   > 1
            gdecl     > x: int = -9223372036854775808
            int_t     > int
            id        > x
            int_lit   > 9223372036854775808
            method    > f() {}
            mdecl     > f()
            id        > f
            block     > {}

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
            program   > f() {
                      >     x: int = 1
                      >     {
                      > ...
                      >         x = x + y
                      >     }
                      > }
            method    > f() {
                      >     x: int = 1
                      >     {
                      > ...
                      >         x = x + y
                      >     }
                      > }
            mdecl     > f()
            id        > f
            block     > {
                      >     x: int = 1
                      >     {
                      > ...
                      >         x = x + y
                      >     }
                      > }
            assign    > x: int = 1
            int_lit   > 1
            decl      > x: int
            int_t     > int
            id        > x
            block     > {
                      >     y: int = 2
                      >     x = x + y
                      > }
            assign    > y: int = 2
            int_lit   > 2
            decl      > y: int
            int_t     > int
            id        > y
            assign    > x = x + y
            bexpr     > x + y
            binop     > +
            id        > x
            id        > y
            id        > x

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
            program   > f() {
                      >     {}
                      >     {{{}}}
                      > }
            method    > f() {
                      >     {}
                      >     {{{}}}
                      > }
            mdecl     > f()
            id        > f
            block     > {
                      >     {}
                      >     {{{}}}
                      > }
            block     > {}
            block     > {{{}}}
            block     > {{}}
            block     > {}

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
            program   > f() {
                      >     x = 5
                      > }
            method    > f() {
                      >     x = 5
                      > }
            mdecl     > f()
            id        > f
            block     > {
                      >     x = 5
                      > }
            assign    > x = 5
            int_lit   > 5
            id        > x

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
            program   > x: int = 1
                      > y: bool = false
            gdecl     > x: int = 1
            int_t     > int
            id        > x
            int_lit   > 1
            gdecl     > y: bool = false
            bool_t    > bool
            id        > y
            bool_lit  > false

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
            program   > f(a: int, b: int): int {
                      >     c: int = (a + b) * (a - b)
                      >     return c
                      > }
            method    > f(a: int, b: int): int {
                      >     c: int = (a + b) * (a - b)
                      >     return c
                      > }
            mdecl     > f(a: int, b: int): int
            id        > f
            decl      > a: int
            int_t     > int
            id        > a
            decl      > b: int
            int_t     > int
            id        > b
            int_t     > int
            block     > {
                      >     c: int = (a + b) * (a - b)
                      >     return c
                      > }
            assign    > c: int = (a + b) * (a - b)
            bexpr     > (a + b) * (a - b)
            binop     > *
            bexpr     > a + b
            binop     > +
            id        > a
            id        > b
            bexpr     > a - b
            binop     > -
            id        > a
            id        > b
            decl      > c: int
            int_t     > int
            id        > c
            return    > return c
            id        > c

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
            program   > f() {
                      >     return 1
                      > }
            method    > f() {
                      >     return 1
                      > }
            mdecl     > f()
            id        > f
            block     > {
                      >     return 1
                      > }
            return    > return 1
            int_lit   > 1

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
            program   > doNothing(x: int) {
                      >     x = 0
                      >     return
                      > }
            method    > doNothing(x: int) {
                      >     x = 0
                      >     return
                      > }
            mdecl     > doNothing(x: int)
            id        > doNothing
            decl      > x: int
            int_t     > int
            id        > x
            block     > {
                      >     x = 0
                      >     return
                      > }
            assign    > x = 0
            int_lit   > 0
            id        > x
            return    > return

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
            program   > ratadd(p1:int, q1:int, p2:int, q2:int) : int, int {
                      >     g:int = gcd(q1,q2)
                      >     p3:int = p1*(q2/g) + p2*(q1/g)
                      >     return p3, q1/g*q2
                      > }
            method    > ratadd(p1:int, q1:int, p2:int, q2:int) : int, int {
                      >     g:int = gcd(q1,q2)
                      >     p3:int = p1*(q2/g) + p2*(q1/g)
                      >     return p3, q1/g*q2
                      > }
            mdecl     > ratadd(p1:int, q1:int, p2:int, q2:int) : int, int
            id        > ratadd
            decl      > p1:int
            int_t     > int
            id        > p1
            decl      > q1:int
            int_t     > int
            id        > q1
            decl      > p2:int
            int_t     > int
            id        > p2
            decl      > q2:int
            int_t     > int
            id        > q2
            int_t     > int
            int_t     > int
            block     > {
                      >     g:int = gcd(q1,q2)
                      >     p3:int = p1*(q2/g) + p2*(q1/g)
                      >     return p3, q1/g*q2
                      > }
            assign    > g:int = gcd(q1,q2)
            pcall     > gcd(q1,q2)
            id        > gcd
            id        > q1
            id        > q2
            decl      > g:int
            int_t     > int
            id        > g
            assign    > p3:int = p1*(q2/g) + p2*(q1/g)
            bexpr     > p1*(q2/g) + p2*(q1/g)
            binop     > +
            bexpr     > p1*(q2/g)
            binop     > *
            id        > p1
            bexpr     > q2/g
            binop     > /
            id        > q2
            id        > g
            bexpr     > p2*(q1/g)
            binop     > *
            id        > p2
            bexpr     > q1/g
            binop     > /
            id        > q1
            id        > g
            decl      > p3:int
            int_t     > int
            id        > p3
            return    > return p3, q1/g*q2
            id        > p3
            bexpr     > q1/g*q2
            binop     > *
            bexpr     > q1/g
            binop     > /
            id        > q1
            id        > g
            id        > q2

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
            program   > foo() {
                      >     p:int, q:int = ratadd(2, 5, 1, 3)
                      >     _, q':int = ratadd(1, 2, 1, 3)
                      > }
            method    > foo() {
                      >     p:int, q:int = ratadd(2, 5, 1, 3)
                      >     _, q':int = ratadd(1, 2, 1, 3)
                      > }
            mdecl     > foo()
            id        > foo
            block     > {
                      >     p:int, q:int = ratadd(2, 5, 1, 3)
                      >     _, q':int = ratadd(1, 2, 1, 3)
                      > }
            assign    > p:int, q:int = ratadd(2, 5, 1, 3)
            pcall     > ratadd(2, 5, 1, 3)
            id        > ratadd
            int_lit   > 2
            int_lit   > 5
            int_lit   > 1
            int_lit   > 3
            decl      > p:int
            int_t     > int
            id        > p
            decl      > q:int
            int_t     > int
            id        > q
            assign    > _, q':int = ratadd(1, 2, 1, 3)
            pcall     > ratadd(1, 2, 1, 3)
            id        > ratadd
            int_lit   > 1
            int_lit   > 2
            int_lit   > 1
            int_lit   > 3
            discard   > _
            decl      > q':int
            int_t     > int
            id        > q'

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
            program   > f() {
                      >   x:int;
                      >   x;=;2
                      > }
            method    > f() {
                      >   x:int;
                      >   x;=;2
                      > }
            mdecl     > f()
            id        > f
            block     > {
                      >   x:int;
                      >   x;=;2
                      > }
            decls     > x:int
            decl      > x:int
            int_t     > int
            id        > x
            error     > x
            error     > =
            error     > 2

            Error: a variable is not a statement
               ╭─[ src0.eta:3:3 ]
               │
             3 │   x;=;2
               │   ┬  
               │   ╰── expected an assignment or a procedure call
               │ 
               │ Note: add `= ...` to assign to it
            ───╯
            Error: expected a statement, found `=`
               ╭─[ src0.eta:3:5 ]
               │
             3 │   x;=;2
               │     ┬  
               │     ╰── expected a statement
            ───╯
            Error: expected a statement, found an integer literal
               ╭─[ src0.eta:3:7 ]
               │
             3 │   x;=;2
               │       ┬  
               │       ╰── expected a statement
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
            src0.eta: parse(recovered)
            (program
              (uses
                (use A))
              (defs))
            program   > use A
            use       > use A
            id        > A

            Error: a program must contain at least one definition
               ╭─[ src0.eta:1:6 ]
               │
             1 │ use A
               │      │ 
               │      ╰─ expected a method or global definition
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
            src0.eta: parse(recovered)
            (program
              (uses
                (use A))
              (defs))
            program   > use A;
            use       > use A;
            id        > A

            Error: a program must contain at least one definition
               ╭─[ src0.eta:1:7 ]
               │
             1 │ use A;
               │       │ 
               │       ╰─ expected a method or global definition
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
                    (decls (: x (array (array bool) 1)))))))
            program   > f() {
                      >   x:bool[][1]
                      > }
            method    > f() {
                      >   x:bool[][1]
                      > }
            mdecl     > f()
            id        > f
            block     > {
                      >   x:bool[][1]
                      > }
            decls     > x:bool[][1]
            decl      > x:bool[][1]
            arr_t     > bool[][1]
            arr_t     > bool[]
            bool_t    > bool
            int_lit   > 1
            id        > x

            Error: an array size cannot follow an unsized dimension
               ╭─[ src0.eta:2:11 ]
               │
             2 │   x:bool[][1]
               │         ─┬─┬─  
               │          ╰───── this one is not
               │            │   
               │            ╰─── this dimension is sized
               │ 
               │ Note: sizes must always come first: [] does not have a size so [size] doesn't know how to allocate
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
                    (decls (: x (array (array (array bool) 1))))))))
            program   > f() {
                      >   x:bool[][1][]
                      > }
            method    > f() {
                      >   x:bool[][1][]
                      > }
            mdecl     > f()
            id        > f
            block     > {
                      >   x:bool[][1][]
                      > }
            decls     > x:bool[][1][]
            decl      > x:bool[][1][]
            arr_t     > bool[][1][]
            arr_t     > bool[][1]
            arr_t     > bool[]
            bool_t    > bool
            int_lit   > 1
            id        > x

            Error: an array size cannot follow an unsized dimension
               ╭─[ src0.eta:2:11 ]
               │
             2 │   x:bool[][1][]
               │         ─┬─┬─  
               │          ╰───── this one is not
               │            │   
               │            ╰─── this dimension is sized
               │ 
               │ Note: sizes must always come first: [] does not have a size so [size] doesn't know how to allocate
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
            program   > f() {
                      >   __ = g()
                      > }
                      > ...
                      > g():int {
                      >   return 0
                      > }
            method    > f() {
                      >   __ = g()
                      > }
            mdecl     > f()
            id        > f
            block     > {
                      >   __ = g()
                      > }
            error     > _
            assign    > _ = g()
            pcall     > g()
            id        > g
            discard   > _
            method    > g():int {
                      >   return 0
                      > }
            mdecl     > g():int
            id        > g
            int_t     > int
            block     > {
                      >   return 0
                      > }
            return    > return 0
            int_lit   > 0

            Error: `_` is only allowed on the left of an assignment
               ╭─[ src0.eta:2:3 ]
               │
             2 │   __ = g()
               │   ┬  
               │   ╰── nothing is being discarded here
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
            program   > identity(x: int): int {
                      >     return x
                      > }
            method    > identity(x: int): int {
                      >     return x
                      > }
            mdecl     > identity(x: int): int
            id        > identity
            decl      > x: int
            int_t     > int
            id        > x
            int_t     > int
            block     > {
                      >     return x
                      > }
            return    > return x
            id        > x

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
            program   > f() {
                      >     a: int[3] = {1, 2, 3}
                      > }
            method    > f() {
                      >     a: int[3] = {1, 2, 3}
                      > }
            mdecl     > f()
            id        > f
            block     > {
                      >     a: int[3] = {1, 2, 3}
                      > }
            assign    > a: int[3] = {1, 2, 3}
            arr_lit   > {1, 2, 3}
            int_lit   > 1
            int_lit   > 2
            int_lit   > 3
            decl      > a: int[3]
            arr_t     > int[3]
            int_t     > int
            int_lit   > 3
            id        > a

        "#]]
    }
}

#[test]
fn spec1() {
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
            program   > foo() {
                      >   x = x + 1
                      >   s = {1, 2, 3}
                      >   b = !b
                      > }
            method    > foo() {
                      >   x = x + 1
                      >   s = {1, 2, 3}
                      >   b = !b
                      > }
            mdecl     > foo()
            id        > foo
            block     > {
                      >   x = x + 1
                      >   s = {1, 2, 3}
                      >   b = !b
                      > }
            assign    > x = x + 1
            bexpr     > x + 1
            binop     > +
            id        > x
            int_lit   > 1
            id        > x
            assign    > s = {1, 2, 3}
            arr_lit   > {1, 2, 3}
            int_lit   > 1
            int_lit   > 2
            int_lit   > 3
            id        > s
            assign    > b = !b
            uexpr     > !b
            uop       > !
            id        > b
            id        > b

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
            program   > foo() {
                      >     s: int[] = "Hello" + {13, 10}
                      > }
            method    > foo() {
                      >     s: int[] = "Hello" + {13, 10}
                      > }
            mdecl     > foo()
            id        > foo
            block     > {
                      >     s: int[] = "Hello" + {13, 10}
                      > }
            assign    > s: int[] = "Hello" + {13, 10}
            bexpr     > "Hello" + {13, 10}
            binop     > +
            string_lit> "Hello"
            arr_lit   > {13, 10}
            int_lit   > 13
            int_lit   > 10
            decl      > s: int[]
            arr_t     > int[]
            int_t     > int
            id        > s

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
            program   > f(): int {
                      >     x: int = "Hello"[0]
                      >     return x
                      > }
            method    > f(): int {
                      >     x: int = "Hello"[0]
                      >     return x
                      > }
            mdecl     > f(): int
            id        > f
            int_t     > int
            block     > {
                      >     x: int = "Hello"[0]
                      >     return x
                      > }
            assign    > x: int = "Hello"[0]
            expr_idx  > "Hello"[0]
            string_lit> "Hello"
            int_lit   > 0
            decl      > x: int
            int_t     > int
            id        > x
            return    > return x
            id        > x

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
            program   > f() {
                      >     a: int = 1
                      >     b: int = 2
                      >     a, b = swap(a, b)
                      > }
            method    > f() {
                      >     a: int = 1
                      >     b: int = 2
                      >     a, b = swap(a, b)
                      > }
            mdecl     > f()
            id        > f
            block     > {
                      >     a: int = 1
                      >     b: int = 2
                      >     a, b = swap(a, b)
                      > }
            assign    > a: int = 1
            int_lit   > 1
            decl      > a: int
            int_t     > int
            id        > a
            assign    > b: int = 2
            int_lit   > 2
            decl      > b: int
            int_t     > int
            id        > b
            assign    > a, b = swap(a, b)
            pcall     > swap(a, b)
            id        > swap
            id        > a
            id        > b
            id        > a
            id        > b

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
            program   > f() {
                      >     a: int[] = {2,}
                      >     b: int[] = {1, 2, 3,}
                      > }
            method    > f() {
                      >     a: int[] = {2,}
                      >     b: int[] = {1, 2, 3,}
                      > }
            mdecl     > f()
            id        > f
            block     > {
                      >     a: int[] = {2,}
                      >     b: int[] = {1, 2, 3,}
                      > }
            assign    > a: int[] = {2,}
            arr_lit   > {2,}
            int_lit   > 2
            decl      > a: int[]
            arr_t     > int[]
            int_t     > int
            id        > a
            assign    > b: int[] = {1, 2, 3,}
            arr_lit   > {1, 2, 3,}
            int_lit   > 1
            int_lit   > 2
            int_lit   > 3
            decl      > b: int[]
            arr_t     > int[]
            int_t     > int
            id        > b

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
            program   > f(x: int, b: bool): int, bool {
                      >     return ---x, !!!b
                      > }
            method    > f(x: int, b: bool): int, bool {
                      >     return ---x, !!!b
                      > }
            mdecl     > f(x: int, b: bool): int, bool
            id        > f
            decl      > x: int
            int_t     > int
            id        > x
            decl      > b: bool
            bool_t    > bool
            id        > b
            int_t     > int
            bool_t    > bool
            block     > {
                      >     return ---x, !!!b
                      > }
            return    > return ---x, !!!b
            uexpr     > ---x
            uop       > -
            uexpr     > --x
            uop       > -
            uexpr     > -x
            uop       > -
            id        > x
            uexpr     > !!!b
            uop       > !
            uexpr     > !!b
            uop       > !
            uexpr     > !b
            uop       > !
            id        > b

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
            program   > f(x: int, b: bool) {
                      >     y: int = - -x
                      >     c: bool = !!b
                      > }
            method    > f(x: int, b: bool) {
                      >     y: int = - -x
                      >     c: bool = !!b
                      > }
            mdecl     > f(x: int, b: bool)
            id        > f
            decl      > x: int
            int_t     > int
            id        > x
            decl      > b: bool
            bool_t    > bool
            id        > b
            block     > {
                      >     y: int = - -x
                      >     c: bool = !!b
                      > }
            assign    > y: int = - -x
            uexpr     > - -x
            uop       > -
            uexpr     > -x
            uop       > -
            id        > x
            decl      > y: int
            int_t     > int
            id        > y
            assign    > c: bool = !!b
            uexpr     > !!b
            uop       > !
            uexpr     > !b
            uop       > !
            id        > b
            decl      > c: bool
            bool_t    > bool
            id        > c

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
            program   > f(x:int, y:int[]) {
                      >   x = -0
                      >   x = -'a'
                      > ...
                      >   x = !true
                      >   x = !false
                      > }
            method    > f(x:int, y:int[]) {
                      >   x = -0
                      >   x = -'a'
                      > ...
                      >   x = (0)
                      >   x = -9223372036854775808
                      > }
            mdecl     > f(x:int, y:int[])
            id        > f
            decl      > x:int
            int_t     > int
            id        > x
            decl      > y:int[]
            arr_t     > int[]
            int_t     > int
            id        > y
            block     > {
                      >   x = -0
                      >   x = -'a'
                      > ...
                      >   x = (0)
                      >   x = -9223372036854775808
                      > }
            assign    > x = -0
            uexpr     > -0
            uop       > -
            int_lit   > 0
            id        > x
            assign    > x = -'a'
            uexpr     > -'a'
            uop       > -
            char_lit  > 'a'
            id        > x
            assign    > x = -x
            uexpr     > -x
            uop       > -
            id        > x
            id        > x
            assign    > x = -i()
            uexpr     > -i()
            uop       > -
            pcall     > i()
            id        > i
            id        > x
            assign    > x = -y[0]
            uexpr     > -y[0]
            uop       > -
            expr_idx  > y[0]
            id        > y
            int_lit   > 0
            id        > x
            assign    > x = - -1
            uexpr     > - -1
            uop       > -
            uexpr     > -1
            uop       > -
            int_lit   > 1
            id        > x
            assign    > x = (0)
            int_lit   > 0
            id        > x
            assign    > x = -9223372036854775808
            uexpr     > -9223372036854775808
            uop       > -
            int_lit   > 9223372036854775808
            id        > x
            method    > i(): int {
                      >   return 0
                      > }
            mdecl     > i(): int
            id        > i
            int_t     > int
            block     > {
                      >   return 0
                      > }
            return    > return 0
            int_lit   > 0
            method    > g(x:bool) {
                      >   x = !true
                      >   x = !false
                      > }
            mdecl     > g(x:bool)
            id        > g
            decl      > x:bool
            bool_t    > bool
            id        > x
            block     > {
                      >   x = !true
                      >   x = !false
                      > }
            assign    > x = !true
            uexpr     > !true
            uop       > !
            bool_lit  > true
            id        > x
            assign    > x = !false
            uexpr     > !false
            uop       > !
            bool_lit  > false
            id        > x

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
            program   > main(args: int[][]) {
                      >     b: bool
                      >     n: int
                      > }
            method    > main(args: int[][]) {
                      >     b: bool
                      >     n: int
                      > }
            mdecl     > main(args: int[][])
            id        > main
            decl      > args: int[][]
            arr_t     > int[][]
            arr_t     > int[]
            int_t     > int
            id        > args
            block     > {
                      >     b: bool
                      >     n: int
                      > }
            decls     > b: bool
            decl      > b: bool
            bool_t    > bool
            id        > b
            decls     > n: int
            decl      > n: int
            int_t     > int
            id        > n

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
            program   > f(x: int) {
                      >     while x > 0 if x > 5 x = x - 2 else x = x - 1
                      > }
            method    > f(x: int) {
                      >     while x > 0 if x > 5 x = x - 2 else x = x - 1
                      > }
            mdecl     > f(x: int)
            id        > f
            decl      > x: int
            int_t     > int
            id        > x
            block     > {
                      >     while x > 0 if x > 5 x = x - 2 else x = x - 1
                      > }
            while     > while x > 0 if x > 5 x = x - 2 else x = x - 1
            bexpr     > x > 0
            binop     > >
            id        > x
            int_lit   > 0
            if        > if x > 5 x = x - 2 else x = x - 1
            bexpr     > x > 5
            binop     > >
            id        > x
            int_lit   > 5
            assign    > x = x - 2
            bexpr     > x - 2
            binop     > -
            id        > x
            int_lit   > 2
            id        > x
            assign    > x = x - 1
            bexpr     > x - 1
            binop     > -
            id        > x
            int_lit   > 1
            id        > x

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
            program   > countdown(n: int) {
                      >     while n > 0
                      >         n = n - 1
                      > }
            method    > countdown(n: int) {
                      >     while n > 0
                      >         n = n - 1
                      > }
            mdecl     > countdown(n: int)
            id        > countdown
            decl      > n: int
            int_t     > int
            id        > n
            block     > {
                      >     while n > 0
                      >         n = n - 1
                      > }
            while     > while n > 0
                      > n = n - 1
            bexpr     > n > 0
            binop     > >
            id        > n
            int_lit   > 0
            assign    > n = n - 1
            bexpr     > n - 1
            binop     > -
            id        > n
            int_lit   > 1
            id        > n

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
            program   > main() {
                      >     return
                      > }
            method    > main() {
                      >     return
                      > }
            mdecl     > main()
            id        > main
            block     > {
                      >     return
                      > }
            return    > return

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
            program   > use io
                      > )))
            use       > use io
            id        > io
            error     > )))

            Error: expected a method or global definition, found `)`
               ╭─[ src0.eta:2:1 ]
               │
             2 │ )))
               │ ┬  
               │ ╰── expected a method or global definition
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
            program   > size():int { return 0 }
                      > }}}
            method    > size():int { return 0 }
            mdecl     > size():int
            id        > size
            int_t     > int
            block     > { return 0 }
            return    > return 0
            int_lit   > 0
            error     > }}}

            Error: expected a method or global definition, found `}`
               ╭─[ src0.eta:2:1 ]
               │
             2 │ }}}
               │ ┬  
               │ ╰── expected a method or global definition
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
            program   > main() { + + }
            method    > main() { + + }
            mdecl     > main()
            id        > main
            block     > { + + }
            error     > + +

            Error: expected a statement, found `+`
               ╭─[ src0.eta:1:10 ]
               │
             1 │ main() { + + }
               │          ┬  
               │          ╰── expected a statement
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
            program   > main() { x:int = }
            method    > main() { x:int = }
            mdecl     > main()
            id        > main
            block     > { x:int = }
            assign    > x:int =
            error     > 
            decl      > x:int
            int_t     > int
            id        > x

            Error: expected an expression, found `}`
               ╭─[ src0.eta:1:18 ]
               │
             1 │ main() { x:int = }
               │                  ┬  
               │                  ╰── expected an expression
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
            program   > main() { x:int = a[3 + ] }
            method    > main() { x:int = a[3 + ] }
            mdecl     > main()
            id        > main
            block     > { x:int = a[3 + ] }
            assign    > x:int = a[3 + ]
            expr_idx  > a[3 + ]
            id        > a
            bexpr     > 3 +
            binop     > +
            int_lit   > 3
            error     > ]
            decl      > x:int
            int_t     > int
            id        > x

            Error: expected an expression, found `]`
               ╭─[ src0.eta:1:24 ]
               │
             1 │ main() { x:int = a[3 + ] }
               │                        ┬  
               │                        ╰── expected an expression
            ───╯
        "#]]
    }
}
