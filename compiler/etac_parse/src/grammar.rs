//! The Eta grammar

use etac_ast::ast::*;
use etac_lexer::token::{Token, TokenKind};
use etac_lexer::Tok;
use etac_source::Span;

use crate::parser::{Expected, List, Parser};
use crate::token_set::{ts, TokenSet};

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
            // use's must come first, report and error, add it, and carry on.
            let at = p.cur_span();
            let u = use_stmt(p);
            let span = u.as_ref().map_or(at, |u| u.span);
            let d = p
                .dcx()
                .err(span, "`use` must come before every definition")
                .with_primary_label("`use` is not allowed here")
                .with_note("move this `use` to the top of the file");
            p.report(d);
            uses.extend(u);
        } else {
            definitions.push(definition(p));
        }

        force_progress(p, start);
    }

    if definitions.is_empty() {
        let d = p
            .dcx()
            .err(p.gap_span(), "a program must contain at least one definition")
            .with_primary_label("expected a method or global definition");
        p.report(d);
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
            .err(p.gap_span(), "an interface must declare at least one method")
            .with_primary_label("expected a method declaration");
        p.report(d);
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
    p.err_expected(Expected::Definition);
    DefinitionKind::Error(Error(skip_to_item(p, TokenSet::EMPTY)))
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
                .err(body.span, "a method in an interface cannot have a body")
                .with_primary_label("remove this body")
                .with_secondary_label(decl.span, "declared here")
                .with_note("interfaces can only declare methods; only `.eta` files can implement them");
            p.report(d);
        }

        p.eat(Tok![;]);
        return InterfaceItemKind::MethodDecl(decl);
    }

    // nice error cases
    if p.matches([ts![ident], ts![:]]) {
        let g = glob_decl(p);
        let d = p
            .dcx()
            .err(g.span, "an interface cannot declare a global")
            .with_primary_label("not allowed in an interface")
            .with_note("move this global into the `.eta` file that defines it");
        p.report(d);
        return InterfaceItemKind::Error(Error(g.span));
    }
    if p.at(Tok![use]) {
        let at = p.cur_span();
        let u = use_stmt(p);
        let span = u.as_ref().map_or(at, |u| u.span);
        let d = p
            .dcx()
            .err(span, "an interface cannot `use` another interface")
            .with_primary_label("not allowed in an interface");
        p.report(d);
        return InterfaceItemKind::Error(Error(span));
    }

    // non-nice errors
    p.err_expected(Expected::InterfaceItem);
    InterfaceItemKind::Error(Error(skip_to_item(p, TokenSet::EMPTY)))
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
            .err(p.gap_span(), "this method has no body")
            .with_primary_label(format!("expected {}", (Tok!['{'])))
            .with_secondary_label(decl.span, "declared here")
            .with_note("eta is not declare before use, there is no need to foward declare")
            .with_note("only interface (`.eti`) files declare methods without a body.");
        p.report(d);
        MethodBodyKind::Error(Error(skip_to_item(p, ts!['{'])))
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
        let params = p.separated(
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
        p.expect(Tok!['(']);
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
                .err(init_span, "a global array cannot be initialised where it is declared")
                .with_primary_label("initialisation happens here")
                .with_secondary_label(type_span, "this is an array type");
            p.report(d);
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
        p.err_expected(Expected::Tok(Tok![int_lit]));
        return None;
    }
    if p.at_any(LIT_FIRST) {
        return Some(GlobDeclValue::Literal(lit_at(p)));
    }
    p.err_and_recover(Expected::GlobalInit, ITEM_FIRST.union(ts![;]));
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
                    .err(stmt_span(&s), "this statement is unreachable")
                    .with_primary_label("unreachable")
                    .with_secondary_label(ret_span, "the block already returned here")
                    .with_note("`return` must be the last statement in a block");
                p.report(d);
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
    if !p.enter() {
        return StmtKind::Error(Error(p.skip_until(STMT_FOLLOW)));
    }
    let out = match p.cur() {
        Tok![if] => StmtKind::If(if_stmt(p)),
        Tok![while] => StmtKind::While(while_stmt(p)),
        Tok![return] => StmtKind::Return(return_stmt(p)),
        Tok!['{'] => StmtKind::Block(block(p)),
        k if STMT_FIRST.contains(k) => assign_or_decl(p),
        _ => StmtKind::Error(Error(p.err_and_recover(Expected::Stmt, STMT_FOLLOW))),
    };
    p.leave();
    out
}

/// ```text
/// if ::= "if" expr stmt ("else" stmt)?
/// ```
///
/// `else` binds to the nearest `if`
fn if_stmt(p: &mut Parser) -> IfStmt {
    let m = p.start();
    p.bump(Tok![if]);
    let cond = condition(p);
    let then_branch = stmt(p);
    let else_branch = if p.eat(Tok![else]) {
        Some(Box::new(stmt(p)))
    } else {
        None
    };
    IfStmt {
        span: p.finish(m),
        cond: Box::new(cond),
        then_branch: Box::new(then_branch),
        else_branch,
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
        p.err_expected(Expected::Expr);
        return ExprKind::Error(Error(p.gap_span()));
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
/// All three start the same way, parse the prefix, then let existance of `=` decide.
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
        let mut values = p.separated(
            /* sepr */ Tok![,],
            /* close */ STMT_FOLLOW,
            /* first */ EXPR_FIRST,
            /* what */ Expected::Expr,
            /* kind */ List::NonEmpty,
            /* elem */ |p| Some(expr(p)),
        );
        // `x: int =` with nothing after it.
        // Standing an error node in for the missing value keeps the arity right, so the type checker reports nothing
        // extra.
        if values.is_empty() {
            values.push(ExprKind::Error(Error(p.gap_span())));
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
                    .err(span, format!("{} is not a statement", expr_noun(&e)))
                    .with_primary_label("expected an assignment or a procedure call")
                    .with_note("add `= ...` to assign to it");
                p.report(d);
                StmtKind::Error(Error(span))
            }
            TargetItem::Discard(span) => {
                let d = p
                    .dcx()
                    .err(span, "`_` is only allowed on the left of an assignment")
                    .with_primary_label("nothing is being discarded here");
                p.report(d);
                StmtKind::Error(Error(span))
            }
            TargetItem::Decl(_) => unreachable!("handled above"),
        };
    }

    // A mixed list with no `=`: `x: int, y` or `f(), g()`.
    p.err_expected(Expected::Tok(Tok![=]));
    StmtKind::Error(Error(p.finish(m)))
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
            ExprKind::Call(c) => TargetKind::LValue(LValueKind::ProcCall(c)),
            // Already reported by whoever produced it.
            ExprKind::Error(e) => TargetKind::Discard(e.0),
            other => {
                let span = expr_span(&other);
                let d = p
                    .dcx()
                    .err(span, format!("cannot assign to {}", expr_noun(&other)))
                    .with_primary_label("not a valid assignment target")
                    .with_note("assign to a variable, an array element, or `_`");
                p.report(d);
                TargetKind::Discard(span)
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
    p.expect(Tok![:]);
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
fn ty(p: &mut Parser, sizes: Sizes) -> TypeKind {
    let m = p.start();
    let mut typ = match p.cur() {
        Tok![int] => TypeKind::Int(p.bump(Tok![int])),
        Tok![bool] => TypeKind::Bool(p.bump(Tok![bool])),
        _ => {
            let stop = STMT_FOLLOW.union(ts![=, ')', ,]);
            return TypeKind::Error(Error(p.err_and_recover(Expected::Type, stop)));
        }
    };

    let mut first_unsized: Option<Span> = None;
    while p.at(Tok!['[']) {
        let dm = p.start();
        let open = p.bump(Tok!['[']);

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
                    .err(dim_span, "a global array cannot be given a size")
                    .with_primary_label("size not allowed here")
                    .with_note("globals are declared unsized: `x: int[]`");
                p.report(d);
            }
            (Some(_), Some(unsized_span)) => {
                let d = p
                    .dcx()
                    .err(dim_span, "an array size cannot follow an unsized dimension")
                    .with_primary_label("this dimension is sized")
                    .with_secondary_label(unsized_span, "this one is not")
                    .with_note(
                        "sizes must always come first: [] does not have a size so [size] doesn't know how to allocate",
                    );
                p.report(d);
            }
            (None, None) => first_unsized = Some(dim_span),
            _ => {}
        }

        typ = TypeKind::Array(ArrType {
            span: p.finish(m),
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
    expr_bp(p, 0)
}

fn expr_bp(p: &mut Parser, min_bp: u8) -> ExprKind {
    if !p.enter() {
        return ExprKind::Error(Error(p.skip_until(STMT_FOLLOW)));
    }
    let m = p.start();

    let mut lhs = if p.at_any(ts![-, !]) {
        let op = unary_op(p);
        let operand = expr_bp(p, PREFIX_BP);
        ExprKind::Unary(UnaryExpr {
            span: p.finish(m),
            op,
            operand: Box::new(operand),
        })
    } else {
        primary(p)
    };

    while let Some((lbp, rbp, kind)) = infix(p.cur()) {
        if lbp < min_bp {
            break;
        }
        let op = BinOp {
            span: p.cur_span(),
            kind,
        };
        p.bump_any();
        let rhs = expr_bp(p, rbp);
        lhs = ExprKind::Binary(BinaryExpr {
            span: p.finish(m),
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        });
    }

    p.leave();
    lhs
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
fn primary(p: &mut Parser) -> ExprKind {
    let m = p.start();

    let mut e = match p.cur() {
        Tok![ident] if p.nth_at(1, Tok!['(']) => ExprKind::Call(proc_call(p)),
        Tok![ident] => ExprKind::Id(ident_at(p)),
        Tok![length] => ExprKind::Length(length_call(p)),
        Tok!['('] => {
            let open = p.bump(Tok!['(']);
            let saved = p.enter_recovery(ts![')']);
            let inner = expr(p);
            p.leave_recovery(saved);
            p.expect_close(Tok![')'], open, "parenthesised expression");
            inner // parentheses group; they leave no node behind
        }
        k if LIT_FIRST.contains(k) => ExprKind::Lit(lit_at(p)),
        _ => ExprKind::Error(Error(p.err_and_recover(Expected::Expr, STMT_FOLLOW))),
    };

    // indexing binds tighter than every operator, and chains.
    while p.at(Tok!['[']) {
        let open = p.bump(Tok!['[']);
        let saved = p.enter_recovery(ts![']']);
        let index = expr(p);
        p.leave_recovery(saved);
        p.expect_close(Tok![']'], open, "index");
        e = ExprKind::Index(ExprIndex {
            span: p.finish(m),
            array: Box::new(e),
            index: Box::new(index),
        });
    }
    e
}

/// ```text
/// proc_call ::= ident "(" expr,* ")"
/// ```
fn proc_call(p: &mut Parser) -> ProcCall {
    let m = p.start();
    let id = ident_at(p);
    let open = p.bump(Tok!['(']);
    let saved = p.enter_recovery(ts![')'].union(TokenSet::of(Tok![,])));
    let args = p.separated(
        /* sepr */ Tok![,],
        /* close */ ts![')'],
        /* first */ EXPR_FIRST,
        /* what */ Expected::Expr,
        /* kind */ List::Any,
        /* elem */ |p| Some(expr(p)),
    );
    p.leave_recovery(saved);
    p.expect_close(Tok![')'], open, "argument list");
    ProcCall {
        span: p.finish(m),
        id,
        args,
    }
}

/// ```text
/// length_call ::= "length" "(" expr ")"
/// ```
fn length_call(p: &mut Parser) -> LengthCall {
    let m = p.start();
    p.bump(Tok![length]);

    if !p.at(Tok!['(']) {
        p.expect(Tok!['(']);
        return LengthCall {
            span: p.finish(m),
            param: Box::new(ExprKind::Error(Error(p.gap_span()))),
        };
    }
    let open = p.bump(Tok!['(']);
    let saved = p.enter_recovery(ts![')']);
    let param = expr(p);
    p.leave_recovery(saved);
    p.expect_close(Tok![')'], open, "argument list");
    LengthCall {
        span: p.finish(m),
        param: Box::new(param),
    }
}

// ---- literals & identifiers ----

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
        p.err_expected(Expected::Ident);
        None
    }
}

fn lit_at(p: &mut Parser) -> LitKind {
    match p.cur() {
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
        Tok!['{'] => LitKind::Array(ArrayLitKind::Array(array_lit(p))),
        k => unreachable!("lit_at on {k}"),
    }
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
fn array_lit(p: &mut Parser) -> ArrayLit {
    let m = p.start();
    let open = p.bump(Tok!['{']);
    let saved = p.enter_recovery(ts!['}'].union(TokenSet::of(Tok![,])));
    let value = p.separated(
        /* sepr */ Tok![,],
        /* close */ ts!['}'],
        /* first */ EXPR_FIRST,
        /* what */ Expected::Expr,
        /* kind */ List::Trailing,
        /* elem */ |p| Some(expr(p)),
    );
    p.leave_recovery(saved);
    p.expect_close(Tok!['}'], open, "array literal");
    ArrayLit {
        span: p.finish(m),
        value,
    }
}

// ---- helpers ----

/// Every list loop calls this. A production that consumes nothing turns its enclosing
/// loop into a hang, worst case we'll skip one extra token for the guarentee that we
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

fn expr_span(e: &ExprKind) -> Span {
    match e {
        ExprKind::Id(n) => n.span,
        ExprKind::Lit(l) => lit_span(l),
        ExprKind::Index(n) => n.span,
        ExprKind::Call(n) => n.span,
        ExprKind::Length(n) => n.span,
        ExprKind::Unary(n) => n.span,
        ExprKind::Binary(n) => n.span,
        ExprKind::Error(Error(s)) => *s,
    }
}

fn lit_span(l: &LitKind) -> Span {
    match l {
        LitKind::Int(n) => n.span,
        LitKind::Bool(n) => n.span,
        LitKind::Char(n) => n.span,
        LitKind::Array(ArrayLitKind::String(n)) => n.span,
        LitKind::Array(ArrayLitKind::Array(n)) => n.span,
    }
}

fn stmt_span(s: &StmtKind) -> Span {
    match s {
        StmtKind::Assign(n) => n.span,
        StmtKind::Decls(n) => n.span,
        StmtKind::If(n) => n.span,
        StmtKind::While(n) => n.span,
        StmtKind::Return(n) => n.span,
        StmtKind::Call(n) => n.span,
        StmtKind::Block(n) => n.span,
        StmtKind::Error(Error(s)) => *s,
    }
}
