//! Diagnostics and recovery.

#![allow(unused_mut, unused_variables, unused_assignments)]

use expect_test::expect;
use indoc::indoc;

#[macro_use]
pub mod util;

#[test]
fn sized_dimension_after_unsized() {
    parsetest! {
        ".eta",
        indoc![r#"
            f() {
                a: int[][3]
            }
        "#],
        expect![[r#"
            src0.eta: parse(recovered)
            (program
              (uses)
              (defs
                (method (decl f (params) (returns))
                  (block
                    (decls (: a (array (array int) 3)))))))
            program   > f() {
                      >     a: int[][3]
                      > }
            method    > f() {
                      >     a: int[][3]
                      > }
            mdecl     > f()
            id        > f
            block     > {
                      >     a: int[][3]
                      > }
            decls     > a: int[][3]
            decl      > a: int[][3]
            arr_t     > int[][3]
            arr_t     > int[]
            int_t     > int
            int_lit   > 3
            id        > a

            Error: an array size cannot follow an unsized dimension
               ╭─[ src0.eta:2:13 ]
               │
             2 │     a: int[][3]
               │           ─┬─┬─  
               │            ╰───── this one is not
               │              │   
               │              ╰─── this dimension is sized
               │ 
               │ Note: sizes must always come first: [] does not have a size so [size] doesn't know how to allocate
            ───╯
        "#]]
    }
}

/// `f() + 1 = 3` parses fine as an expression; it is only an error once you ask what it
/// means. Validating after the parse names the problem instead of reporting a stray `=`.
#[test]
fn assign_to_non_lvalue() {
    parsetest! {
        ".eta",
        indoc![r#"
            f() {
                g() + 1 = 3
            }
        "#],
        expect![[r#"
            src0.eta: parse(recovered)
            (program
              (uses)
              (defs
                (method (decl f (params) (returns))
                  (block
                    (assign (targets _) (values 3))))))
            program   > f() {
                      >     g() + 1 = 3
                      > }
            method    > f() {
                      >     g() + 1 = 3
                      > }
            mdecl     > f()
            id        > f
            block     > {
                      >     g() + 1 = 3
                      > }
            assign    > g() + 1 = 3
            int_lit   > 3
            discard   > g() + 1

            Error: cannot assign to a binary expression
               ╭─[ src0.eta:2:5 ]
               │
             2 │     g() + 1 = 3
               │     ───┬───  
               │        ╰───── not a valid assignment target
               │ 
               │ Note: assign to a variable, an array element, or `_`
            ───╯
        "#]]
    }
}

/// The diagnostic a table-driven parser cannot produce: *which* brace never came back.
#[test]
fn unclosed_block_points_at_the_opener() {
    parsetest! {
        ".eta",
        indoc![r#"
            f() {
                x: int = 1
        "#],
        expect![[r#"
            src0.eta: parse(recovered)
            (program
              (uses)
              (defs
                (method (decl f (params) (returns))
                  (block
                    (assign (targets (: x int)) (values 1))))))
            program   > f() {
                      > x: int = 1
            method    > f() {
                      > x: int = 1
            mdecl     > f()
            id        > f
            block     > {
                      > x: int = 1
            assign    > x: int = 1
            int_lit   > 1
            decl      > x: int
            int_t     > int
            id        > x

            Error: unclosed block
               ╭─[ src0.eta:1:5 ]
               │
             1 │ f() {
               │     ┬  
               │     ╰── this block is never closed
               │ 
               │ Note: expected `}` to close it
            ───╯
        "#]]
    }
}

/// A missing separator should not end the argument list -- one typo, one diagnostic.
#[test]
fn missing_comma_keeps_the_list_together() {
    parsetest! {
        ".eta",
        indoc![r#"
            f() {
                g(1 2, 3)
            }
        "#],
        expect![[r#"
            src0.eta: parse(recovered)
            (program
              (uses)
              (defs
                (method (decl f (params) (returns))
                  (block
                    (call g 1 2 3)))))
            program   > f() {
                      >     g(1 2, 3)
                      > }
            method    > f() {
                      >     g(1 2, 3)
                      > }
            mdecl     > f()
            id        > f
            block     > {
                      >     g(1 2, 3)
                      > }
            pcall     > g(1 2, 3)
            id        > g
            int_lit   > 1
            int_lit   > 2
            int_lit   > 3

            Error: expected `,`, found an integer literal
               ╭─[ src0.eta:2:9 ]
               │
             2 │     g(1 2, 3)
               │         ┬  
               │         ╰── expected `,`
            ───╯
        "#]]
    }
}

/// A stray separator is deleted rather than swallowing the element after it.
#[test]
fn stray_comma_in_argument_list() {
    parsetest! {
        ".eta",
        indoc![r#"
            f() {
                g(1,, 2)
            }
        "#],
        expect![[r#"
            src0.eta: parse(recovered)
            (program
              (uses)
              (defs
                (method (decl f (params) (returns))
                  (block
                    (call g 1 2)))))
            program   > f() {
                      >     g(1,, 2)
                      > }
            method    > f() {
                      >     g(1,, 2)
                      > }
            mdecl     > f()
            id        > f
            block     > {
                      >     g(1,, 2)
                      > }
            pcall     > g(1,, 2)
            id        > g
            int_lit   > 1
            int_lit   > 2

            Error: expected an expression, found `,`
               ╭─[ src0.eta:2:9 ]
               │
             2 │     g(1,, 2)
               │         ┬  
               │         ╰── expected an expression
            ───╯
        "#]]
    }
}

/// `return` may only be last. Parsing the rest of the block anyway lets the message point
/// at both halves instead of failing at the `}`.
#[test]
fn statement_after_return() {
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

/// A method body in a `.eti` gets named, and the interface still parses.
#[test]
fn body_in_interface() {
    parsetest! {
        ".eti",
        indoc![r#"
            f(): int {
                return 1
            }
            g(): bool
        "#],
        expect![[r#"
            src0.eti: parse(recovered)
            (interface
              (decl f (params) (returns int))
              (decl g (params) (returns bool)))
            interfce  > f(): int {
                      >     return 1
                      > }
                      > g(): bool
            mdecl     > f(): int
            id        > f
            int_t     > int
            mdecl     > g(): bool
            id        > g
            bool_t    > bool

            Error: a method in an interface cannot have a body
               ╭─[ src0.eti:1:10 ]
               │
             1 │ ╭─▶ f(): int {
               ┆ ┆   
             3 │ ├─▶ }
               │ │       
               │ ╰─────── remove this body
               │
               ├─[ src0.eti:1:10 ]
               │
             1 │ f(): int {
               │ ────┬───  
               │     ╰───── declared here
               │ 
               │ Note: interfaces can only declare methods; only `.eta` files can implement them
            ───╯
        "#]]
    }
}

/// A `.eta` method without a body, with the declaration labelled.
#[test]
fn missing_method_body() {
    parsetest! {
        ".eta",
        indoc![r#"
            f(): int
            g() {
            }
        "#],
        expect![[r#"
            src0.eta: parse(recovered)
            (program
              (uses)
              (defs
                (method (decl f (params) (returns int))
                  (error))
                (method (decl g (params) (returns))
                  (block))))
            program   > f(): int
                      > g() {
                      > }
            method    > f(): int
            mdecl     > f(): int
            id        > f
            int_t     > int
            error     > 
            method    > g() {
                      > }
            mdecl     > g()
            id        > g
            block     > {
                      > }

            Error: this method has no body
               ╭─[ src0.eta:1:9 ]
               │
             1 │ f(): int
               │ ────┬───│ 
               │     ╰───── declared here
               │         │ 
               │         ╰─ expected `{`
               │ 
               │ Note: only interface (`.eti`) files declare methods without a body.
            ───╯
        "#]]
    }
}

/// A misplaced `use` is kept in the tree, so the rest of the file still gets read.
#[test]
fn use_after_definition() {
    parsetest! {
        ".eta",
        indoc![r#"
            f() {
            }
            use io
            g() {
            }
        "#],
        expect![[r#"
            src0.eta: parse(recovered)
            (program
              (uses
                (use io))
              (defs
                (method (decl f (params) (returns))
                  (block))
                (method (decl g (params) (returns))
                  (block))))
            program   > f() {
                      > }
                      > use io
                      > g() {
                      > }
            use       > use io
            id        > io
            method    > f() {
                      > }
            mdecl     > f()
            id        > f
            block     > {
                      > }
            method    > g() {
                      > }
            mdecl     > g()
            id        > g
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

/// Globals are declared unsized. The LR grammar had a `GlobDeclTypeKind_Sized` rule for
/// this that was never referenced, so `x: int[4]` used to be accepted.
#[test]
fn sized_global_array() {
    parsetest! {
        ".eta",
        indoc![r#"
            x: int[4]
            f() {
            }
        "#],
        expect![[r#"
            src0.eta: parse(recovered)
            (program
              (uses)
              (defs
                (global x (array int 4))
                (method (decl f (params) (returns))
                  (block))))
            program   > x: int[4]
                      > f() {
                      > }
            gdecl     > x: int[4]
            arr_t     > int[4]
            int_t     > int
            int_lit   > 4
            id        > x
            method    > f() {
                      > }
            mdecl     > f()
            id        > f
            block     > {
                      > }

            Error: a global array cannot be given a size
               ╭─[ src0.eta:1:7 ]
               │
             1 │ x: int[4]
               │       ─┬─  
               │        ╰─── size not allowed here
               │ 
               │ Note: globals are declared unsized: `x: int[]`
            ───╯
        "#]]
    }
}

/// Nothing that could start a definition, so one diagnostic covers the whole file rather
/// than one per identifier.
#[test]
fn prose_is_one_diagnostic() {
    parsetest! {
        ".eta",
        indoc![r#"
            what a beautiful, invalid program!
        "#],
        expect![[r#"
            src0.eta: parse(recovered)
            (program
              (uses)
              (defs
                (error)))
            program   > what a beautiful, invalid program!
            error     > what a beautiful, invalid program!

            Error: expected a method or global definition, found an identifier
               ╭─[ src0.eta:1:1 ]
               │
             1 │ what a beautiful, invalid program!
               │ ──┬─  
               │   ╰─── expected a method or global definition
            ───╯
        "#]]
    }
}

/// Junk inside a call stops at that call's `)`, not at the end of the file: the enclosing
/// construct pushes its own anchors before descending.
#[test]
fn junk_inside_a_call_stays_inside_it() {
    parsetest! {
        ".eta",
        indoc![r#"
            f() {
                g(] )
                h()
            }
        "#],
        expect![[r#"
            src0.eta: parse(recovered)
            (program
              (uses)
              (defs
                (method (decl f (params) (returns))
                  (block
                    (call g)
                    (call h)))))
            program   > f() {
                      >     g(] )
                      >     h()
                      > }
            method    > f() {
                      >     g(] )
                      >     h()
                      > }
            mdecl     > f()
            id        > f
            block     > {
                      >     g(] )
                      >     h()
                      > }
            pcall     > g(] )
            id        > g
            pcall     > h()
            id        > h

            Error: expected an expression, found `]`
               ╭─[ src0.eta:2:7 ]
               │
             2 │     g(] )
               │       ┬  
               │       ╰── expected an expression
            ───╯
        "#]]
    }
}

/// `if {` is a missing condition, not an array literal that ate the body.
#[test]
fn if_with_no_condition() {
    parsetest! {
        ".eta",
        indoc![r#"
            f() {
                if {
                    g()
                }
            }
        "#],
        expect![[r#"
            src0.eta: parse(recovered)
            (program
              (uses)
              (defs
                (method (decl f (params) (returns))
                  (block
                    (if (error)
                      (block
                        (call g)))))))
            program   > f() {
                      >     if {
                      >         g()
                      >     }
                      > }
            method    > f() {
                      >     if {
                      >         g()
                      >     }
                      > }
            mdecl     > f()
            id        > f
            block     > {
                      >     if {
                      >         g()
                      >     }
                      > }
            if        > if {
                      >     g()
                      > }
            error     > 
            block     > {
                      >     g()
                      > }
            pcall     > g()
            id        > g

            Error: expected an expression, found `{`
               ╭─[ src0.eta:2:8 ]
               │
             2 │     if {
               │        ┬  
               │        ╰── expected an expression
            ───╯
        "#]]
    }
}

/// `else` binds to the nearest `if`, without the `Stmt_Open`/`Stmt_Closed` split.
#[test]
fn dangling_else_binds_innermost() {
    parsetest! {
        ".eta",
        indoc![r#"
            f(a: bool, b: bool) {
                if a if b g() else h()
            }
        "#],
        expect![[r#"
            src0.eta: parse(clean)
            (program
              (uses)
              (defs
                (method (decl f (params (: a bool) (: b bool)) (returns))
                  (block
                    (if a
                      (if b
                        (call g)
                        (call h)))))))
            program   > f(a: bool, b: bool) {
                      >     if a if b g() else h()
                      > }
            method    > f(a: bool, b: bool) {
                      >     if a if b g() else h()
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
                      >     if a if b g() else h()
                      > }
            if        > if a if b g() else h()
            id        > a
            if        > if b g() else h()
            id        > b
            pcall     > g()
            id        > g
            pcall     > h()
            id        > h

        "#]]
    }
}

/// `_` is only meaningful on the left of an assignment.
#[test]
fn bare_discard() {
    parsetest! {
        ".eta",
        indoc![r#"
            f() {
                _
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
                      >     _
                      > }
            method    > f() {
                      >     _
                      > }
            mdecl     > f()
            id        > f
            block     > {
                      >     _
                      > }
            error     > _

            Error: `_` is only allowed on the left of an assignment
               ╭─[ src0.eta:2:5 ]
               │
             2 │     _
               │     ┬  
               │     ╰── nothing is being discarded here
            ───╯
        "#]]
    }
}
