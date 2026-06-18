use super::types;
use crate::context::Env;
use etac_ast::*;

// collects the global scope of a program
pub fn collect(prog: &Program) -> Env {
    let mut env = Env::new();
    let definitions = prog.definitions;
    for def in definitions {
        let kind = def.kind;
        match kind {
            DefinitionKind::Method(m) => env.insert_ident(
                m.id.sym.clone(),
                types::IdTy::Fn(types::FnTy {
                    from: m.params.iter().map(|decl| (&decl.typ.kind).into()).collect(),
                    to: m.ret_types.iter().map(|typ| (&typ.kind).into()).collect(),
                }),
            ),
            DefinitionKind::GlobDecl(gd) => env.insert_ident(gd.id.sym.clone(), types::IdTy::Var((&typ.kind).into())),
            DefinitionKind::Error => (),
        }
    }
    env
}
