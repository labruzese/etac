// use std::collections::HashMap;
//
// use etac_ast::id::Id;
//
// use crate::sources::Span;
//
// pub enum AstHead {
//     Program(etac_ast::ast::Program),
//     Interface(etac_ast::ast::Interface),
// }
//
// pub type AstMap = HashMap<u32, AstHead>;
//
// #[derive(Default)]
// pub struct AstArena {
//     // FileId -> Ast
//     pub ast_arena: 
//     pub ast_map: AstMap,
//     pub spans: SpanMap,
// }
//
// #[derive(Default)]
// pub struct SpanMap {
//     map: Vec<Span>,
// }
//
// impl SpanMap {
//     pub fn span<T>(&self, node: Id<T>) -> Span {
//         *self.map
//             .get(node.index())
//             .expect("nodes always allocate with a span")
//     }
// }
