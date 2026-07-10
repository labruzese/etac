use super::types;
use crate::context::Env;
use crate::Typecheck;
use etac_ast::*;
use etac_span::SourceCache;

// collects the global scope of a program
pub fn add_interface<C: SourceCache>(env: &mut Env<'_, C>, interface: &Interface) {
    for _item in &interface.items {

    }
}

pub fn collect_global<C: SourceCache>(env: &mut Env<'_, C>, prog: &Program) {
    let definitions = &prog.definitions;
    for def in definitions {
        match &def.kind {
            DefinitionKind::Method(m) => {
                env.scopes.declare_fn(
                    m.node_id(),
                    m.id.sym.clone(),
                    types::FnTy {
                        from: m.params.iter().map(|decl| (&decl.typ.kind).into()).collect(),
                        to: m.ret_types.iter().map(|typ| (&typ.kind).into()).collect(),
                    },
                );
            }
            DefinitionKind::GlobDecl(gd) => { let _ = gd.typecheck(env); }
            DefinitionKind::Error => (), // Error already recorded
        }
    }
}
