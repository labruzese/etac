//! Lower into the ast with imports linked


use etac_ast::visit::VisitorOwned;
use etac_source::{Span, SourceMap};
use etac_intern::Resolver;

struct Lowerer<'ast> {
    map: 
}

use etac_ast::hir::*;
use etac_ast::ast::{Program as AstProgram, Use as AstUse};

impl<'ast> VisitorOwned<'ast> for Lowerer<'ast> {
    type Result = Program;
    fn visit_program_ast_owned(&mut self, p: AstProgram) {
        let AstProgram {
            span,
            uses,
            definitions,
        } = p;
        self.program = Some(Program {
            span,
            uses: Vec::new(),
            definitions,
        });
    }
    fn visit_use_ast_owned(&mut self, u: &'ast AstUse) {
        let AstUse { span, interface } = u;

    }
}
