//! The Eta grammar

use etac_ast::ast::*;
use etac_errors::guarantee::ErrorGuaranteed;
use etac_lexer::Tok;
use etac_lexer::token::{Token, TokenKind};
use etac_source::Span;

use crate::parser::{Expected, List, MAX_HEIGHT, Parser};
use crate::token_set::{TokenSet, ts};

// ---- first / follow sets ----

/// Tokens that can begin a literal.
const LIT_FIRST: TokenSet = ts![int_lit, bool_lit, char_lit, str_lit, '{' /* arr_lit */];
/// Tokens that can begin an expression.
const EXPR_FIRST: TokenSet = ts![ident, length, '(', -, !].union(LIT_FIRST);
/// Tokens that can begin a type.
const TYPE_FIRST: TokenSet = ts![int, bool];
/// Tokens that can begin a statement.
/// Deliberately no `(`
const STMT_FIRST: TokenSet = ts![if, while, return, '{', ident, _];
/// Tokens that can begin a top-level item.
const ITEM_FIRST: TokenSet = ts![ident, use];
/// A declaration starts with an identifier; the `:` is resolved by lookahead.
const DECL_FIRST: TokenSet = ts![ident];

/// Where a statement can end.
const STMT_FOLLOW: TokenSet = ts![;, '}', else].union(STMT_FIRST);

// ---- top level ----

/// ```text
/// program ::= use* definition+
/// ```
pub fn program(p: &mut Parser) -> Program {
    let m = p.start();
    let mut uses = Vec::new();
    let mut definitions = Vec::new();

    // use*
    while p.at(Tok![use]) {
        uses.extend(use_stmt(p));
    }

    while !p.at_eof() {
        let start = p.pos();

        if p.at(Tok![use]) {
            // `use`s must come first: report an error, add it anyway, and carry on.
            let at = p.cur_span();
            let u = use_stmt(p);
            let span = u.as_ref().map_or(at, |u| u.span);
            let d = p
                .dcx()
                .new_err(span, "`use` must come before every definition")
                .with_message("`use` is not allowed here")
                .with_note("move this `use` to the top of the file");
            let _ = p.report(d);
            uses.extend(u);
        } else {
            definitions.push(definition(p));
        }

        force_progress(p, start);
    }

    if definitions.is_empty() {
        let d = p
            .dcx()
            .new_err(p.gap_span(), "a program must contain at least one definition")
            .with_message("expected a method or global definition");
        let _ = p.report(d);
    }

    Program {
        span: p.finish(m),
        uses,
        definitions,
    }
}

/// ```text
/// interface ::= method_decl+
/// ```
pub fn interface(p: &mut Parser) -> Interface {
    let m = p.start();
    let mut items = Vec::new();

    while !p.at_eof() {
        let start = p.pos();
        items.push(interface_item(p));
        force_progress(p, start);
    }

    if items.is_empty() {
        let d = p
            .dcx()
            .new_err(p.gap_span(), "an interface must declare at least one method")
            .with_message("expected a method declaration");
        let _ = p.report(d);
    }

    Interface {
        span: p.finish(m),
        items,
    }
}

/// ```text
/// use ::= "use" ident ";"?
/// ```
///
/// `None` when there was no name to import.
fn use_stmt(p: &mut Parser) -> Option<Use> {
    let m = p.start();
    p.bump(Tok![use]);
    let Some(interface) = ident(p) else {
        p.skip_until(ITEM_FIRST.union(ts![;]));
        p.eat(Tok![;]);
        return None;
    };
    p.eat(Tok![;]);
    Some(Use {
        span: p.finish(m),
        interface,
    })
}

/// Skip to the next plausible top-level item: `use`, or an identifier that is
/// followed by `(` or `:`. We don't want to sync on each identifier to avoid
/// spamming errors when there's text.
fn skip_to_item(p: &mut Parser, also: TokenSet) -> Span {
    let m = p.start();
    while !p.at_eof() {
        if p.at(Tok![use]) {
            break;
        }
        if p.at_any(also) {
            break;
        }
        if p.matches([ts![ident], ts!['(', :]]) {
            break;
        }
        p.bump_any();
    }
    p.finish(m)
}

/// ```text
/// definition ::= method | glob_decl
/// ```
fn definition(p: &mut Parser) -> DefinitionKind {
    // `ident (` is a method
    // `ident :` is a global
    if p.at(Tok![ident]) {
        match p.nth(1) {
            Tok!['('] => return DefinitionKind::Method(method(p)),
            Tok![:] => return DefinitionKind::GlobDecl(glob_decl(p)),
            _ => {}
        }
    }
    let reported = p.err_expected(Expected::Definition);
    DefinitionKind::Error(Error {
        span: skip_to_item(p, TokenSet::EMPTY),
        reported,
    })
}

/// ```text
/// interface_item ::= method_decl ";"?
/// ```
fn interface_item(p: &mut Parser) -> InterfaceItemKind {
    // success case
    if p.matches([ts![ident], ts!['(']]) {
        let decl = method_decl(p);

        // report the body but parse it anyway
        if p.at(Tok!['{']) {
            let body = block(p);
            let d = p
                .dcx()
                .new_err(body.span, "a method in an interface cannot have a body")
                .with_message("remove this body")
                .with_label(decl.span, "declared here")
                .with_note("interfaces can only declare methods; only `.eta` files can implement them");
            let _ = p.report(d);
        }

        p.eat(Tok![;]);
        return InterfaceItemKind::MethodDecl(decl);
    }

    // nice error cases
    if p.matches([ts![ident], ts![:]]) {
        let g = glob_decl(p);
        let d = p
            .dcx()
            .new_err(g.span, "an interface cannot declare a global")
            .with_message("not allowed in an interface")
            .with_note("move this global into the `.eta` file that defines it");
        let reported = p.report(d);
        return InterfaceItemKind::Error(Error { span: g.span, reported });
    }
    if p.at(Tok![use]) {
        let at = p.cur_span();
        let u = use_stmt(p);
        let span = u.as_ref().map_or(at, |u| u.span);
        let d = p
            .dcx()
            .new_err(span, "an interface cannot `use` another interface")
            .with_message("not allowed in an interface");
        let reported = p.report(d);
        return InterfaceItemKind::Error(Error { span, reported });
    }

    // non-nice errors
    let reported = p.err_expected(Expected::InterfaceItem);
    InterfaceItemKind::Error(Error {
        span: skip_to_item(p, TokenSet::EMPTY),
        reported,
    })
}

/// ```text
/// method ::= method_decl block
/// ```
fn method(p: &mut Parser) -> Method {
    let m = p.start();
    let decl = method_decl(p);
    let body = if p.at(Tok!['{']) {
        MethodBodyKind::Block(block(p))
    } else {
        let d = p
            .dcx()
            .new_err(p.gap_span(), "this method has no body")
            .with_message(format!("expected {}", (Tok!['{'])))
            .with_label(decl.span, "declared here")
            .with_note("a method can be used before it is defined, so there is no need to forward declare it")
            .with_note("only interface (`.eti`) files declare methods without a body");
        let reported = p.report(d);
        MethodBodyKind::Error(Error {
            span: skip_to_item(p, ts!['{']),
            reported,
        })
    };
    Method {
        span: p.finish(m),
        decl,
        body,
    }
}

/// ```text
/// method_decl ::= ident "(" decl,* ")" (":" type,+)?
/// ```
fn method_decl(p: &mut Parser) -> MethodDecl {
    let m = p.start();
    let id = ident_at(p);

    let params = if p.at(Tok!['(']) {
        let open = p.bump(Tok!['(']);
        let saved = p.enter_recovery(ts![')'].union(TokenSet::of(Tok![,])));
        let (params, _) = p.separated(
            /* sepr */ Tok![,],
            /* close */ ts![')'],
            /* first */ DECL_FIRST,
            /* what */ Expected::Param,
            /* kind */ List::Any,
            /* elem */ |p| Some(decl(p)),
        );
        p.leave_recovery(saved);
        p.expect_close(Tok![')'], open, "parameter list");
        params
    } else {
        let _ = p.expect(Tok!['(']);
        Vec::new()
    };

    let ret_types = if p.eat(Tok![:]) {
        // A return list ends at the body, or at whatever starts the next item.
        p.separated(
            /* sepr */ Tok![,],
            /* close */ ts!['{'].union(ITEM_FIRST),
            /* first */ TYPE_FIRST,
            /* what */ Expected::Type,
            /* kind */ List::NonEmpty,
            /* elem */ |p| Some(ty(p, Sizes::Allowed)),
        )
        .0
    } else {
        Vec::new()
    };

    MethodDecl {
        span: p.finish(m),
        id,
        params,
        ret_types,
    }
}

/// ```text
/// glob_decl ::= ident ":" type ("=" glob_decl_value)? ";"?
/// ```
fn glob_decl(p: &mut Parser) -> GlobDecl {
    let m = p.start();
    let id = ident_at(p);
    p.bump(Tok![:]);

    let tm = p.start();
    let typ = ty(p, Sizes::Forbidden);
    let type_span = p.finish(tm);

    let val = if p.at(Tok![=]) {
        let vm = p.start();
        p.bump(Tok![=]);
        let v = glob_decl_value(p);
        let init_span = p.finish(vm);

        // report this error but save it anyway.
        if matches!(typ, TypeKind::Array(_)) {
            let d = p
                .dcx()
                .new_err(init_span, "a global array cannot be initialised where it is declared")
                .with_message("initialisation happens here")
                .with_label(type_span, "this is an array type");
            let _ = p.report(d);
        }
        v
    } else {
        None
    };

    p.eat(Tok![;]);
    GlobDecl {
        span: p.finish(m),
        id,
        typ,
        val,
    }
}

/// ```text
/// glob_decl_value ::= literal | "-" int_literal
/// ```
fn glob_decl_value(p: &mut Parser) -> Option<GlobDeclValue> {
    let m = p.start();
    if p.at(Tok![-]) {
        p.bump(Tok![-]);
        if p.at(Tok![int_lit]) {
            let value = int_lit_at(p);
            return Some(GlobDeclValue::Neg {
                span: p.finish(m),
                value,
            });
        }
        let _ = p.err_expected(Expected::Tok(Tok![int_lit]));
        return None;
    }
    if p.at_any(LIT_FIRST) {
        let (lit, _) = lit_at(p);
        check_int_range(p, &lit);
        return Some(GlobDeclValue::Literal(lit));
    }
    let _ = p.err_and_recover(Expected::GlobalInit, ITEM_FIRST.union(ts![;]));
    None
}

// ---- statements ----

/// ```text
/// block ::= "{" (stmt ";"?)* "}"
/// ```
fn block(p: &mut Parser) -> Block {
    let m = p.start();
    let open = p.bump(Tok!['{']);

    let saved = p.enter_recovery(STMT_FOLLOW);
    let mut stmts: Vec<StmtKind> = Vec::new();
    let mut returned: Option<Span> = None;
    let mut reported_return_error = false;

    while !p.at(Tok!['}']) && !p.at_eof() {
        // `;` is optional after every statement, stray => empty statement
        if p.eat(Tok![;]) {
            continue;
        }

        let before = p.pos();
        let s = stmt(p);

        // `return` is allowed only in final position.
        match (&s, returned, reported_return_error) {
            (StmtKind::Return(r), None, false) => returned = Some(r.span),
            (_, Some(ret_span), false) => {
                let d = p
                    .dcx()
                    .new_err(stmt_span(&s), "this statement is unreachable")
                    .with_message("unreachable")
                    .with_label(ret_span, "the block already returned here")
                    .with_note("`return` must be the last statement in a block");
                let _ = p.report(d);
                reported_return_error = true; // once per block is plenty
            }
            _ => {}
        }

        stmts.push(s);
        p.eat(Tok![;]);
        force_progress(p, before);
    }

    p.leave_recovery(saved);
    p.expect_close(Tok!['}'], open, "block");
    Block {
        span: p.finish(m),
        stmts,
    }
}

/// ```text
/// stmt ::= if | while | return | block | assign | decls | call
/// ```
fn stmt(p: &mut Parser) -> StmtKind {
    if let Err(reported) = p.enter() {
        return StmtKind::Error(Error {
            span: p.skip_until(STMT_FOLLOW),
            reported,
        });
    }
    let out = match p.cur() {
        Tok![if] => StmtKind::If(if_stmt(p)),
        Tok![while] => StmtKind::While(while_stmt(p)),
        Tok![return] => StmtKind::Return(return_stmt(p)),
        Tok!['{'] => StmtKind::Block(block(p)),
        k if STMT_FIRST.contains(k) => assign_or_decl(p),
        _ => StmtKind::Error(p.err_and_recover(Expected::Stmt, STMT_FOLLOW)),
    };
    p.leave();
    out
}

/// ```text
/// if ::= "if" expr stmt ("else" stmt)?
/// ```
///
/// `else` binds to the nearest `if`
///
/// An `else if` is an `if` nested in an `else`, but a chain of them is parsed in a loop: recursing through
/// [`stmt`] for every branch would run a long chain into the recursion cap, and one branch per case is how such
/// chains are written.
fn if_stmt(p: &mut Parser) -> IfStmt {
    // Each `if` of the chain, outermost first. The `else` that would complete it is not known yet.
    let mut arms = Vec::new();
    let mut else_branch = None;
    loop {
        let m = p.start();
        p.bump(Tok![if]);
        let cond = condition(p);
        let then_branch = stmt(p);
        arms.push((m, cond, then_branch));

        if !p.at(Tok![else]) {
            break;
        }
        if !p.nth_at(1, Tok![if]) {
            p.bump(Tok![else]);
            else_branch = Some(Box::new(stmt(p)));
            break;
        }
        if !p.enter_link() {
            else_branch = Some(Box::new(StmtKind::Error(else_if_too_deep(p))));
            break;
        }
        p.bump(Tok![else]);
    }
    p.leave_links(arms.len() as u32 - 1);

    // Nest them, from the last `if` outwards. Every `if` runs to the end of the chain.
    let mut nested: Option<IfStmt> = None;
    for (m, cond, then_branch) in arms.into_iter().rev() {
        let else_branch = match nested.take() {
            Some(inner) => Some(Box::new(StmtKind::If(inner))),
            None => else_branch.take(),
        };
        nested = Some(IfStmt {
            span: p.finish(m),
            cond: Box::new(cond),
            then_branch: Box::new(then_branch),
            else_branch,
        });
    }
    nested.expect("the loop pushes at least one `if`")
}

/// Report an `else if` that would nest the tree past [`MAX_HEIGHT`], and read the rest of its chain without
/// building it. Returns the error node standing in for what was read.
fn else_if_too_deep(p: &mut Parser) -> Error {
    let m = p.start();
    let d = p
        .dcx()
        .new_err(p.cur_span(), "this chain of `else if` is too long")
        .with_message(format!("statements are nested more than {MAX_HEIGHT} deep here"))
        .with_note("every `else if` nests inside the one before it; split the chain up");
    let reported = p.report(d);

    while p.eat(Tok![else]) {
        if !p.at(Tok![if]) {
            stmt(p);
            break;
        }
        p.bump(Tok![if]);
        condition(p);
        stmt(p);
    }
    Error {
        span: p.finish(m),
        reported,
    }
}

/// ```text
/// while ::= "while" expr stmt
/// ```
fn while_stmt(p: &mut Parser) -> WhileStmt {
    let m = p.start();
    p.bump(Tok![while]);
    let cond = condition(p);
    let body = stmt(p);
    WhileStmt {
        span: p.finish(m),
        cond: Box::new(cond),
        body: Box::new(body),
    }
}

/// The condition of an `if` or `while`.
///
/// `{` is formally ambiguous: it opens both an array literal and the body block
///
/// Array is never a useful condition, so a `{` after `if` means the body, and the
/// condition is simply missing. Reading the body as an array literal will cascade
/// and is unlikely to be what the user meant.
fn condition(p: &mut Parser) -> ExprKind {
    if p.at(Tok!['{']) {
        let reported = p.err_expected(Expected::Expr);
        return ExprKind::Error(Error {
            span: p.gap_span(),
            reported,
        });
    }
    let saved = p.enter_recovery(ts!['{']);
    let e = expr(p);
    p.leave_recovery(saved);
    e
}

/// ```text
/// return ::= "return" expr,* ";"?
/// ```
fn return_stmt(p: &mut Parser) -> ReturnStmt {
    let m = p.start();
    p.bump(Tok![return]);
    let values = if p.at_any(EXPR_FIRST) {
        p.separated(
            /* sepr */ Tok![,],
            /* close */ STMT_FOLLOW,
            /* first */ EXPR_FIRST,
            /* what */ Expected::Expr,
            /* kind */ List::NonEmpty,
            /* elem */ |p| Some(expr(p)),
        )
        .0
    } else {
        Vec::new()
    };
    p.eat(Tok![;]);
    ReturnStmt {
        span: p.finish(m),
        values,
    }
}

/// One item on the left of a `=`, before we know whether there is a `=` at all.
enum TargetItem {
    Decl(Decl),
    Discard(Span),
    Expr(ExprKind),
}

/// ```text
/// assign ::= target,+ "=" expr,+
/// decls  ::= decl,+
/// call   ::= proc_call
/// ```
///
/// All three start the same way, parse the prefix, then let existence of `=` decide.
fn assign_or_decl(p: &mut Parser) -> StmtKind {
    let m = p.start();

    let saved = p.enter_recovery(ts![=].union(TokenSet::of(Tok![,])));
    let mut items = Vec::new();
    loop {
        let before = p.pos();
        items.push(target_item(p));
        force_progress(p, before);
        if !p.eat(Tok![,]) {
            break;
        }
    }
    p.leave_recovery(saved);

    if p.at(Tok![=]) {
        p.bump(Tok![=]);
        let (mut values, missing) = p.separated(
            /* sepr */ Tok![,],
            /* close */ STMT_FOLLOW,
            /* first */ EXPR_FIRST,
            /* what */ Expected::Expr,
            /* kind */ List::NonEmpty,
            /* elem */ |p| Some(expr(p)),
        );
        // `x: int =` with nothing after it, which the list reported.
        // Standing an error node in for the missing value keeps the arity right, so the type checker reports nothing
        // extra.
        if values.is_empty() {
            let reported = missing.unwrap_or_else(|| p.err_expected(Expected::Expr));
            values.push(ExprKind::Error(Error {
                span: p.gap_span(),
                reported,
            }));
        }
        let targets = items.into_iter().map(|i| into_target(p, i)).collect();
        return StmtKind::Assign(AssignStmt {
            span: p.finish(m),
            targets,
            values,
        });
    }

    // No `=`: either every item was a declaration, or this is a bare procedure call.
    if items.iter().all(|i| matches!(i, TargetItem::Decl(_))) {
        let decls = items
            .into_iter()
            .map(|i| match i {
                TargetItem::Decl(d) => d,
                _ => unreachable!("just checked"),
            })
            .collect();
        return StmtKind::Decls(MultiDeclStmt {
            span: p.finish(m),
            decls,
        });
    }

    if items.len() == 1 {
        return match items.pop().expect("len == 1") {
            TargetItem::Expr(ExprKind::Call(c)) => StmtKind::Call(c),
            TargetItem::Expr(ExprKind::Error(e)) => StmtKind::Error(e),
            TargetItem::Expr(e) => {
                let span = expr_span(&e);
                let d = p
                    .dcx()
                    .new_err(span, format!("{} is not a statement", expr_noun(&e)))
                    .with_message("expected an assignment or a procedure call")
                    .with_note("add `= ...` to assign to it");
                let reported = p.report(d);
                StmtKind::Error(Error { span, reported })
            }
            TargetItem::Discard(span) => {
                let d = p
                    .dcx()
                    .new_err(span, "`_` is only allowed on the left of an assignment")
                    .with_message("nothing is being discarded here");
                let reported = p.report(d);
                StmtKind::Error(Error { span, reported })
            }
            TargetItem::Decl(_) => unreachable!("handled above"),
        };
    }

    // A mixed list with no `=`: `x: int, y` or `f(), g()`.
    let reported = p.err_expected(Expected::Tok(Tok![=]));
    StmtKind::Error(Error {
        span: p.finish(m),
        reported,
    })
}

fn target_item(p: &mut Parser) -> TargetItem {
    if p.at(Tok![_]) {
        return TargetItem::Discard(p.bump(Tok![_]));
    }
    // `ident :` is a declaration; anything else is an expression that then has to turn
    // out to be assignable.
    if p.matches([ts![ident], ts![:]]) {
        return TargetItem::Decl(decl(p));
    }
    TargetItem::Expr(expr(p))
}

/// Check after the fact that an expression can be assigned to.
fn into_target(p: &mut Parser, item: TargetItem) -> TargetKind {
    match item {
        TargetItem::Decl(d) => TargetKind::Decl(d),
        TargetItem::Discard(s) => TargetKind::Discard(s),
        TargetItem::Expr(e) => match e {
            ExprKind::Id(i) => TargetKind::LValue(LValueKind::Id(i)),
            ExprKind::Index(ix) => TargetKind::LValue(LValueKind::Index(ix)),
            // Already reported by whoever produced it.
            ExprKind::Error(e) => TargetKind::Error(e),
            other => {
                let span = expr_span(&other);
                let d = p
                    .dcx()
                    .new_err(span, format!("cannot assign to {}", expr_noun(&other)))
                    .with_message("not a valid assignment target")
                    .with_note("assign to a variable, an array element, or `_`");
                let reported = p.report(d);
                TargetKind::Error(Error { span, reported })
            }
        },
    }
}

/// ```text
/// decl ::= ident ":" type
/// ```
fn decl(p: &mut Parser) -> Decl {
    let m = p.start();
    let id = ident_at(p);
    let _ = p.expect(Tok![:]);
    let typ = ty(p, Sizes::Allowed);
    Decl {
        span: p.finish(m),
        id,
        typ,
    }
}

// ---- types ----

/// Whether `[expr]` dimensions are allowed.
/// Globals must be declared unsized `x: int[]`, never `x: int[4]` because there is nowhere to run the allocation.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Sizes {
    Allowed,
    Forbidden,
}

/// ```text
/// type ::= ("int" | "bool") ("[" expr? "]")*
/// ```
///
/// The first dimension written is the outermost array: `int[3][4]` is an array of 3 `int[4]`s.
fn ty(p: &mut Parser, sizes: Sizes) -> TypeKind {
    let m = p.start();
    let mut typ = match p.cur() {
        Tok![int] => TypeKind::Int(p.bump(Tok![int])),
        Tok![bool] => TypeKind::Bool(p.bump(Tok![bool])),
        _ => {
            let stop = STMT_FOLLOW.union(ts![=, ')', ,]);
            return TypeKind::Error(p.err_and_recover(Expected::Type, stop));
        }
    };

    // The dimensions as written, outermost first.
    let mut dims: Vec<Option<Box<ExprKind>>> = Vec::new();
    let mut first_unsized: Option<Span> = None;
    while p.at(Tok!['[']) {
        let dm = p.start();
        let open = p.bump(Tok!['[']);

        // Each dimension nests the type one level deeper.
        if dims.len() as u32 == MAX_HEIGHT {
            let d = p
                .dcx()
                .new_err(open, "this array type has too many dimensions")
                .with_message(format!("this is dimension {}", MAX_HEIGHT + 1))
                .with_note(format!("an array type can have at most {MAX_HEIGHT} dimensions"));
            let reported = p.report(d);
            // Read the rest of the dimensions without keeping them.
            let mut open = open;
            loop {
                if !p.at(Tok![']']) {
                    let saved = p.enter_recovery(ts![']']);
                    expr(p);
                    p.leave_recovery(saved);
                }
                if !p.expect_close(Tok![']'], open, "array dimension") || !p.at(Tok!['[']) {
                    break;
                }
                open = p.bump(Tok!['[']);
            }
            return TypeKind::Error(Error {
                span: p.finish(m),
                reported,
            });
        }

        let size = if p.at(Tok![']']) {
            None
        } else {
            let saved = p.enter_recovery(ts![']']);
            let e = expr(p);
            p.leave_recovery(saved);
            Some(Box::new(e))
        };
        p.expect_close(Tok![']'], open, "array dimension");
        let dim_span = p.finish(dm);

        match (&size, first_unsized) {
            (Some(_), _) if sizes == Sizes::Forbidden => {
                let d = p
                    .dcx()
                    .new_err(dim_span, "a global array cannot be given a size")
                    .with_message("size not allowed here")
                    .with_note("globals are declared unsized: `x: int[]`");
                let _ = p.report(d);
            }
            (Some(_), Some(unsized_span)) => {
                let d = p
                    .dcx()
                    .new_err(dim_span, "an array size cannot follow an unsized dimension")
                    .with_message("this dimension is sized")
                    .with_label(unsized_span, "this one is not")
                    .with_note(
                        "sizes must always come first: [] does not have a size so [size] doesn't know how to allocate",
                    );
                let _ = p.report(d);
            }
            (None, None) => first_unsized = Some(dim_span),
            _ => {}
        }

        dims.push(size);
    }

    // Wrap from the inside out: the last dimension written is the one right around the element type. An inner
    // array's own text is not contiguous (the `int` and `[4]` of `int[3][4]`), so every level spans the whole type.
    let span = p.finish(m);
    for size in dims.into_iter().rev() {
        typ = TypeKind::Array(ArrType {
            span,
            of: Box::new(typ),
            size,
        });
    }
    typ
}

// ---- expressions ----

/// Left and right binding power, plus the operator the token maps to.
/// Every operator is left-associative, so `rbp == lbp + 1`
#[inline]
const fn infix(k: TokenKind) -> Option<(u8, u8, BinOpKind)> {
    Some(match k {
        Tok![|] => (1, 2, BinOpKind::LOr),
        Tok![&] => (3, 4, BinOpKind::LAnd),
        Tok![==] => (5, 6, BinOpKind::Eq),
        Tok![!=] => (5, 6, BinOpKind::Neq),
        Tok![<] => (7, 8, BinOpKind::Lt),
        Tok![<=] => (7, 8, BinOpKind::Le),
        Tok![>] => (7, 8, BinOpKind::Gt),
        Tok![>=] => (7, 8, BinOpKind::Ge),
        Tok![+] => (9, 10, BinOpKind::Add),
        Tok![-] => (9, 10, BinOpKind::Sub),
        Tok![*] => (11, 12, BinOpKind::Mul),
        Tok![*>>] => (11, 12, BinOpKind::HighMul),
        Tok![/] => (11, 12, BinOpKind::Div),
        Tok![%] => (11, 12, BinOpKind::Mod),
        _ => return None,
    })
}

/// Prefix `-` and `!` bind tighter than any infix operator, so `-a * b` is `(-a) * b`.
/// Indexing binds tighter still and lives in [`primary`]'s loop.
const PREFIX_BP: u8 = 13;

/// ```text
/// expr ::= unop expr | expr binop expr | primary
/// ```
pub fn expr(p: &mut Parser) -> ExprKind {
    expr_h(p).0
}

/// An expression and the height of its tree, for the productions that nest it in a bigger expression.
///
/// The height is how [`MAX_HEIGHT`] is enforced: a leaf is 1 high, and every node is one higher than its tallest
/// child. Each production that loops checks the height it has reached, and so no expression is ever taller.
fn expr_h(p: &mut Parser) -> (ExprKind, u32) {
    expr_bp(p, 0, false)
}

/// `negated` says the expression is the operand of a unary `-`, which is the one place the literal
/// `9223372036854775808` may be written: see [`check_int_range`].
fn expr_bp(p: &mut Parser, min_bp: u8, negated: bool) -> (ExprKind, u32) {
    if let Err(reported) = p.enter() {
        let span = p.skip_until(STMT_FOLLOW);
        return (ExprKind::Error(Error { span, reported }), 1);
    }
    let m = p.start();

    let (mut lhs, mut height) = if p.at_any(ts![-, !]) {
        let op = unary_op(p);
        let (operand, height) = expr_bp(p, PREFIX_BP, matches!(op.kind, UOpKind::Neg));
        let unary = ExprKind::Unary(UnaryExpr {
            span: p.finish(m),
            op,
            operand: Box::new(operand),
        });
        (unary, height + 1)
    } else {
        primary(p, negated)
    };

    while let Some((lbp, rbp, kind)) = infix(p.cur()) {
        if lbp < min_bp {
            break;
        }
        let op = BinOp {
            span: p.cur_span(),
            kind,
        };
        let op_span = op.span;
        p.bump_any();
        let (rhs, rhs_height) = expr_bp(p, rbp, false);
        height = height.max(rhs_height) + 1;
        lhs = ExprKind::Binary(BinaryExpr {
            span: p.finish(m),
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        });

        if height > MAX_HEIGHT {
            let reported = too_tall(p, op_span);
            // Read the rest of the chain without building on it: an operand at a time, whatever the operators.
            while infix(p.cur()).is_some() {
                p.bump_any();
                expr_bp(p, PREFIX_BP, false);
            }
            lhs = ExprKind::Error(Error {
                span: p.finish(m),
                reported,
            });
            height = 1;
        }
    }

    p.leave();
    (lhs, height)
}

/// Report an expression that grew past [`MAX_HEIGHT`], at the operator or index that took it there.
fn too_tall(p: &mut Parser, at: Span) -> ErrorGuaranteed {
    let d = p
        .dcx()
        .new_err(at, "this expression is too large")
        .with_message(format!("the expression is more than {MAX_HEIGHT} operations deep here"))
        .with_note("split it up, storing the parts in variables");
    p.report(d)
}

fn unary_op(p: &mut Parser) -> UOp {
    let kind = match p.cur() {
        Tok![-] => UOpKind::Neg,
        Tok![!] => UOpKind::Not,
        k => unreachable!("unary_op on {k}"),
    };
    UOp {
        span: p.bump_any().1,
        kind,
    }
}

/// ```text
/// primary ::= literal | ident | proc_call | length_call | "(" expr ")" | primary "[" expr "]"
/// ```
fn primary(p: &mut Parser, negated: bool) -> (ExprKind, u32) {
    let m = p.start();

    let (mut e, mut height) = match p.cur() {
        Tok![ident] if p.nth_at(1, Tok!['(']) => {
            let (call, height) = proc_call(p);
            (ExprKind::Call(call), height)
        }
        Tok![ident] => (ExprKind::Id(ident_at(p)), 1),
        Tok![length] => {
            let (call, height) = length_call(p);
            (ExprKind::Length(call), height)
        }
        Tok!['('] => {
            let open = p.bump(Tok!['(']);
            let saved = p.enter_recovery(ts![')']);
            let inner = expr_h(p);
            p.leave_recovery(saved);
            p.expect_close(Tok![')'], open, "parenthesised expression");
            inner // parentheses group; they leave no node behind
        }
        k if LIT_FIRST.contains(k) => {
            let (lit, height) = lit_at(p);
            if !negated {
                check_int_range(p, &lit);
            }
            (ExprKind::Lit(lit), height)
        }
        _ => (ExprKind::Error(p.err_and_recover(Expected::Expr, STMT_FOLLOW)), 1),
    };

    // indexing binds tighter than every operator, and chains.
    while p.at(Tok!['[']) {
        let im = p.start();
        let (index, index_height) = index_suffix(p);
        let index_span = p.finish(im);
        height = height.max(index_height) + 1;
        e = ExprKind::Index(ExprIndex {
            span: p.finish(m),
            array: Box::new(e),
            index: Box::new(index),
        });

        if height > MAX_HEIGHT {
            let reported = too_tall(p, index_span);
            // Read the rest of the chain without building on it.
            while p.at(Tok!['[']) {
                index_suffix(p);
            }
            e = ExprKind::Error(Error {
                span: p.finish(m),
                reported,
            });
            height = 1;
        }
    }
    (e, height)
}

/// The `[expr]` after something being indexed.
fn index_suffix(p: &mut Parser) -> (ExprKind, u32) {
    let open = p.bump(Tok!['[']);
    let saved = p.enter_recovery(ts![']']);
    let index = expr_h(p);
    p.leave_recovery(saved);
    p.expect_close(Tok![']'], open, "index");
    index
}

/// ```text
/// proc_call ::= ident "(" expr,* ")"
/// ```
fn proc_call(p: &mut Parser) -> (ProcCall, u32) {
    let m = p.start();
    let id = ident_at(p);
    let open = p.bump(Tok!['(']);
    let saved = p.enter_recovery(ts![')'].union(TokenSet::of(Tok![,])));
    let mut tallest = 0;
    let (args, _) = p.separated(
        /* sepr */ Tok![,],
        /* close */ ts![')'],
        /* first */ EXPR_FIRST,
        /* what */ Expected::Expr,
        /* kind */ List::Any,
        /* elem */ |p| Some(expr_under(p, &mut tallest)),
    );
    p.leave_recovery(saved);
    p.expect_close(Tok![')'], open, "argument list");
    let call = ProcCall {
        span: p.finish(m),
        id,
        args,
    };
    (call, tallest + 1)
}

/// An expression that is one of several children of a node: `tallest` keeps the height of the tallest so far.
fn expr_under(p: &mut Parser, tallest: &mut u32) -> ExprKind {
    let (e, height) = expr_h(p);
    *tallest = (*tallest).max(height);
    e
}

/// ```text
/// length_call ::= "length" "(" expr ")"
/// ```
fn length_call(p: &mut Parser) -> (LengthCall, u32) {
    let m = p.start();
    p.bump(Tok![length]);

    if !p.at(Tok!['(']) {
        let Err(reported) = p.expect(Tok!['(']) else {
            unreachable!("`expect` reports a token that is not there")
        };
        let call = LengthCall {
            span: p.finish(m),
            param: Box::new(ExprKind::Error(Error {
                span: p.gap_span(),
                reported,
            })),
        };
        return (call, 2);
    }
    let open = p.bump(Tok!['(']);
    let saved = p.enter_recovery(ts![')']);
    let (param, height) = expr_h(p);
    p.leave_recovery(saved);
    p.expect_close(Tok![')'], open, "argument list");
    let call = LengthCall {
        span: p.finish(m),
        param: Box::new(param),
    };
    (call, height + 1)
}

// ---- literals & identifiers ----

/// Report an integer literal that is not an `int`.
///
/// The lexer lets `9223372036854775808` through, one more than the largest `int`, because `-9223372036854775808` is
/// the smallest. Call this for a literal anywhere but directly under that `-`.
fn check_int_range(p: &mut Parser, lit: &LitKind) {
    if let LitKind::Int(int) = lit
        && int.value > i64::MAX as u64
    {
        let d = p
            .dcx()
            .new_err(int.span, "integer literal out of range")
            .with_message("this is one more than the largest `int`")
            .with_note(format!(
                "{} can only be written with a `-` directly in front of it, as the smallest `int`",
                int.value
            ));
        let _ = p.report(d);
    }
}

/// Consume an identifier the caller has already checked for.
fn ident_at(p: &mut Parser) -> Ident {
    debug_assert!(p.at(Tok![ident]));
    let (tok, span) = p.bump_any();
    match tok {
        Token::Identifier(sym) => Ident { span, sym },
        other => unreachable!("ident_at on {other:?}"),
    }
}

/// Consume an identifier, reporting if there isn't one.
fn ident(p: &mut Parser) -> Option<Ident> {
    if p.at(Tok![ident]) {
        Some(ident_at(p))
    } else {
        let _ = p.err_expected(Expected::Ident);
        None
    }
}

/// A literal and the height of its tree: 1, unless it is an array literal with elements.
fn lit_at(p: &mut Parser) -> (LitKind, u32) {
    if p.at(Tok!['{']) {
        let (array, height) = array_lit(p);
        return (LitKind::Array(ArrayLitKind::Array(array)), height);
    }
    let lit = match p.cur() {
        Tok![int_lit] => LitKind::Int(int_lit_at(p)),
        Tok![bool_lit] => {
            let (tok, span) = p.bump_any();
            let Token::BoolLiteral(value) = tok else { unreachable!() };
            LitKind::Bool(BoolLit { span, value })
        }
        Tok![char_lit] => {
            let (tok, span) = p.bump_any();
            let Token::CharLiteral(value) = tok else { unreachable!() };
            LitKind::Char(CharLit { span, value })
        }
        Tok![str_lit] => {
            let (tok, span) = p.bump_any();
            let Token::StrLiteral(value) = tok else { unreachable!() };
            LitKind::Array(ArrayLitKind::String(StringLit { span, value }))
        }
        k => unreachable!("lit_at on {k}"),
    };
    (lit, 1)
}

fn int_lit_at(p: &mut Parser) -> IntLit {
    debug_assert!(p.at(Tok![int_lit]));
    let (tok, span) = p.bump_any();
    let Token::IntLiteral(value) = tok else { unreachable!() };
    IntLit { span, value }
}

/// ```text
/// array_lit ::= "{" expr,* ","? "}"
/// ```
fn array_lit(p: &mut Parser) -> (ArrayLit, u32) {
    let m = p.start();
    let open = p.bump(Tok!['{']);
    let saved = p.enter_recovery(ts!['}'].union(TokenSet::of(Tok![,])));
    let mut tallest = 0;
    let (value, _) = p.separated(
        /* sepr */ Tok![,],
        /* close */ ts!['}'],
        /* first */ EXPR_FIRST,
        /* what */ Expected::Expr,
        /* kind */ List::Trailing,
        /* elem */ |p| Some(expr_under(p, &mut tallest)),
    );
    p.leave_recovery(saved);
    p.expect_close(Tok!['}'], open, "array literal");
    let array = ArrayLit {
        span: p.finish(m),
        value,
    };
    (array, tallest + 1)
}

// ---- helpers ----

/// Every list loop calls this. A production that consumes nothing turns its enclosing
/// loop into a hang, worst case we'll skip one extra token for the guarantee that we
/// never hang
#[inline]
fn force_progress(p: &mut Parser, before: usize) {
    if p.pos() == before && !p.at_eof() {
        p.bump_any();
    }
}

fn expr_noun(e: &ExprKind) -> &'static str {
    match e {
        ExprKind::Id(_) => "a variable",
        ExprKind::Lit(_) => "a literal",
        ExprKind::Index(_) => "an array element",
        ExprKind::Call(_) => "a procedure call",
        ExprKind::Length(_) => "a `length` call",
        ExprKind::Unary(_) => "a unary expression",
        ExprKind::Binary(_) => "a binary expression",
        ExprKind::Error(_) => "this",
    }
}

pub fn expr_span(e: &ExprKind) -> Span {
    match e {
        ExprKind::Id(n) => n.span,
        ExprKind::Lit(l) => lit_span(l),
        ExprKind::Index(n) => n.span,
        ExprKind::Call(n) => n.span,
        ExprKind::Length(n) => n.span,
        ExprKind::Unary(n) => n.span,
        ExprKind::Binary(n) => n.span,
        ExprKind::Error(e) => e.span,
    }
}

pub fn lit_span(l: &LitKind) -> Span {
    match l {
        LitKind::Int(n) => n.span,
        LitKind::Bool(n) => n.span,
        LitKind::Char(n) => n.span,
        LitKind::Array(ArrayLitKind::String(n)) => n.span,
        LitKind::Array(ArrayLitKind::Array(n)) => n.span,
    }
}

pub fn stmt_span(s: &StmtKind) -> Span {
    match s {
        StmtKind::Assign(n) => n.span,
        StmtKind::Decls(n) => n.span,
        StmtKind::If(n) => n.span,
        StmtKind::While(n) => n.span,
        StmtKind::Return(n) => n.span,
        StmtKind::Call(n) => n.span,
        StmtKind::Block(n) => n.span,
        StmtKind::Error(e) => e.span,
    }
}
