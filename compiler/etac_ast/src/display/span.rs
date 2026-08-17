use std::borrow::Cow;
use std::fmt;
use std::ops::{ControlFlow};

use etac_source::{SourceMap, Span};

use crate::ast::*;
use crate::visit::Visitor;
use crate::walk::Walkable;

pub struct SpanPrinter<'ast, 'sm, NodeT> {
    node: &'ast NodeT,
    sm: &'sm SourceMap,
}

impl<'ast, 'sm, NodeT> SpanPrinter<'ast, 'sm, NodeT> {
    pub const SPAN_ELIPSE_AT_CHAR_NUM: usize = 120;
    pub const SPAN_ELIPSE_AT_LINE_NUM: usize = 6;
    pub fn new(node: &'ast NodeT, sm: &'sm SourceMap) -> Self {
        SpanPrinter {
            node,
            sm,
        }
    }
}

impl<'ast, 'sm> std::fmt::Display for SpanPrinter<'ast, 'sm, Program> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        SpanPrinterWriter {
            out: f,
            printer: self
        }.visit_program(self.node).continue_ok()
    }
}

impl<'ast, 'sm> std::fmt::Display for SpanPrinter<'ast, 'sm, Interface> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        SpanPrinterWriter {
            out: f,
            printer: self
        }.visit_interface(self.node).continue_ok()
    }
}

struct SpanPrinterWriter<'ast, 'sm, 'buf, 'prt, 'fmt, NodeT> {
    out: &'fmt mut fmt::Formatter<'buf>,
    printer: &'prt SpanPrinter<'ast, 'sm, NodeT>
}

impl<'ast, 'sm, 'buf, 'prt, 'fmt, NodeT> SpanPrinterWriter<'ast, 'sm, 'buf, 'prt, 'fmt, NodeT> {
    fn write(&mut self, tag: &str, span: &Span) -> ControlFlow<fmt::Error> {
        let seac = SpanPrinter::<'ast, 'sm, NodeT>::SPAN_ELIPSE_AT_CHAR_NUM;
        let seal = SpanPrinter::<'ast, 'sm, NodeT>::SPAN_ELIPSE_AT_LINE_NUM;

        let raw = self.printer.sm.slice(*span);
        let s = Self::dedent(raw);
        let s = Self::elipse_lines(&s, seal);

        let mut lines = s.lines();
        Self::into_cf(writeln!(self.out, "{tag:<width$}> {}", Self::elipse_line(lines.next().unwrap_or_default(), seac), width = 10))?;
        for line in lines {
            Self::into_cf(writeln!(self.out, "          > {}", Self::elipse_line(line, seac)))?;
        }
        ControlFlow::Continue(())
    }

    fn elipse_lines<'s>(str: &'s str, limit: usize) -> Cow<'s, str> {
        let half = (limit / 2).max(1);
        let head = str.match_indices('\n').nth(half - 1).map(|(i, _)| i);
        let tail = str.rmatch_indices('\n').nth(half - 1).map(|(i, _)| i + 1);
        match (head, tail) {
            (Some(h), Some(t)) if t > h + 1 => {
                Cow::Owned(format!("{}\n...\n{}", &str[..h], &str[t..]))
            }
            _ => Cow::Borrowed(str),
        }
    }

    fn elipse_line<'s>(line: &'s str, limit: usize) -> Cow<'s, str> {
        if line.len() > limit {
            let half = limit / 2;
            let head = line.floor_char_boundary(half);
            let tail = line.ceil_char_boundary(line.len() - half);
            Cow::Owned(format!("{}...{}", &line[..head], &line[tail..]))
        } else {
            Cow::Borrowed(line)
        }
    }

    fn dedent(s: &str) -> Cow<'_, str> {
        let common = s
            .lines()
            .skip(1)
            .filter(|l| !l.trim().is_empty())
            .map(|l| l.len() - l.trim_start().len())
            .min()
            .unwrap_or(0);

        if common == 0 {
            return Cow::Borrowed(s);
        }

        let mut out = String::with_capacity(s.len());
        for (i, line) in s.lines().enumerate() {
            if i > 0 {
                out.push('\n');
                // blank lines may be shorter than `common`
                out.push_str(&line[common.min(line.len())..]);
            } else {
                out.push_str(line);
            }
        }
        Cow::Owned(out)
    }

    fn into_cf<O, E>(res: Result<O, E>) -> ControlFlow<E, O> {
        match res {
            Ok(ok) => ControlFlow::Continue(ok),
            Err(err) => ControlFlow::Break(err),
        }
    }
}

impl<'ast, 'sm, 'buf, 'prt, 'fmt, NodeT> Visitor<'ast> for SpanPrinterWriter<'ast, 'sm, 'buf, 'prt, 'fmt, NodeT> {
    type Result = ControlFlow<fmt::Error>;

    fn visit_program(&mut self, n: &'ast Program) -> Self::Result {
        let Program { span, .. } = n;
        self.write("program", span)?;
        n.walk(self)
    }

    fn visit_interface(&mut self, n: &'ast Interface) -> Self::Result {
        let Interface { span, .. } = n;
        self.write("interfce", span)?;
        n.walk(self)
    }

    fn visit_use(&mut self, n: &'ast Use) -> Self::Result {
        let Use { span, .. } = n;
        self.write("use", span)?;
        n.walk(self)
    }

    fn visit_method_decl(&mut self, n: &'ast MethodDecl) -> Self::Result {
        let MethodDecl { span, .. } = n;
        self.write("mdecl", span)?;
        n.walk(self)
    }

    fn visit_method(&mut self, n: &'ast Method) -> Self::Result {
        let Method { span, .. } = n;
        self.write("method", span)?;
        n.walk(self)
    }


    fn visit_glob_decl(&mut self, n: &'ast GlobDecl) -> Self::Result {
        let GlobDecl { span, .. } = n;
        self.write("gdecl", span)?;
        n.walk(self)
    }

    fn visit_assign_stmt(&mut self, n: &'ast AssignStmt) -> Self::Result {
        let AssignStmt { span, .. } = n;
        self.write("assign", span)?;
        n.walk(self)
    }

    fn visit_multi_decl_stmt(&mut self, n: &'ast MultiDeclStmt) -> Self::Result {
        let MultiDeclStmt { span, .. } = n;
        self.write("decls", span)?;
        n.walk(self)
    }

    fn visit_decl(&mut self, n: &'ast Decl) -> Self::Result {
        let Decl { span, .. } = n;
        self.write("decl", span)?;
        n.walk(self)
    }

    fn visit_if_stmt(&mut self, n: &'ast IfStmt) -> Self::Result {
        let IfStmt { span, .. } = n;
        self.write("if", span)?;
        n.walk(self)
    }

    fn visit_while_stmt(&mut self, n: &'ast WhileStmt) -> Self::Result {
        let WhileStmt { span, .. } = n;
        self.write("while", span)?;
        n.walk(self)
    }

    fn visit_return_stmt(&mut self, n: &'ast ReturnStmt) -> Self::Result {
        let ReturnStmt { span, .. } = n;
        self.write("return", span)?;
        n.walk(self)
    }

    fn visit_proc_call(&mut self, n: &'ast ProcCall) -> Self::Result {
        let ProcCall { span, .. } = n;
        self.write("pcall", span)?;
        n.walk(self)
    }

    fn visit_block(&mut self, n: &'ast Block) -> Self::Result {
        let Block { span, .. } = n;
        self.write("block", span)?;
        n.walk(self)
    }

    fn visit_arr_type(&mut self, n: &'ast ArrType) -> Self::Result {
        let ArrType { span, .. } = n;
        self.write("arr_t", span)?;
        n.walk(self)
    }

    fn visit_ident(&mut self, n: &'ast Ident) -> Self::Result {
        let Ident { span, .. } = n;
        self.write("id", span)?;
        n.walk(self)
    }

    fn visit_int_lit(&mut self, n: &'ast IntLit) -> Self::Result {
        let IntLit { span, .. } = n;
        self.write("int_lit", span)?;
        n.walk(self)
    }
    
    fn visit_bool_lit(&mut self, n: &'ast BoolLit) -> Self::Result {
        let BoolLit { span, .. } = n;
        self.write("bool_lit", span)?;
        n.walk(self)
    }

    fn visit_char_lit(&mut self, n: &'ast CharLit) -> Self::Result {
        let CharLit { span, .. } = n;
        self.write("char_lit", span)?;
        n.walk(self)
    }

    fn visit_string_lit(&mut self, n: &'ast StringLit) -> Self::Result {
        let StringLit { span, .. } = n;
        self.write("string_lit", span)?;
        n.walk(self)
    }

    fn visit_array_lit(&mut self, n: &'ast ArrayLit) -> Self::Result {
        let ArrayLit { span, .. } = n;
        self.write("arr_lit", span)?;
        n.walk(self)
    }

    fn visit_expr_index(&mut self, n: &'ast ExprIndex) -> Self::Result {
        let ExprIndex { span, .. } = n;
        self.write("expr_idx", span)?;
        n.walk(self)
    }

    fn visit_length_call(&mut self, n: &'ast LengthCall) -> Self::Result {
        let LengthCall { span, .. } = n;
        self.write("length", span)?;
        n.walk(self)
    }

    fn visit_unary_expr(&mut self, n: &'ast UnaryExpr) -> Self::Result {
        let UnaryExpr { span, .. } = n;
        self.write("uexpr", span)?;
        n.walk(self)
    }

    fn visit_uop(&mut self, n: &'ast UOp) -> Self::Result {
        let UOp { span, .. } = n;
        self.write("uop", span)?;
        n.walk(self)
    }

    fn visit_binary_expr(&mut self, n: &'ast BinaryExpr) -> Self::Result {
        let BinaryExpr { span, .. } = n;
        self.write("bexpr", span)?;
        n.walk(self)
    }

    fn visit_bin_op(&mut self, n: &'ast BinOp) -> Self::Result {
        let BinOp { span, .. } = n;
        self.write("binop", span)?;
        n.walk(self)
    }

    fn visit_type_kind(&mut self, n: &'ast TypeKind) -> Self::Result {
        match n {
            TypeKind::Int(span) => self.write("int_t", span),
            TypeKind::Bool(span) => self.write("bool_t", span),
            _ => ControlFlow::Continue(()),
        }?;
        n.walk(self)
    }

    fn visit_target_kind(&mut self, n: &'ast TargetKind) -> Self::Result {
        match n {
            TargetKind::Discard(span) => self.write("discard", span),
            _ => ControlFlow::Continue(()),
        }?;
        n.walk(self)
    }

    fn visit_error(&mut self, n: &'ast Error) -> Self::Result {
        self.write("error", &n.0)?;
        n.walk(self)
    }
 }
