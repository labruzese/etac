use lasso::{RodeoReader, Spur, ThreadedRodeo};
use rustc_hash::FxHasher;
use std::hash::BuildHasherDefault;
type FxBuild = BuildHasherDefault<FxHasher>;

pub use lasso::Interner;
pub use lasso::Resolver;

pub type Symbol = Spur;
/// The interner. It hands out symbols through `&self`, so every thread of a compilation can share the one.
pub type EtacInterner = ThreadedRodeo<Symbol, FxBuild>;
pub type EtacReader = RodeoReader<Symbol, FxBuild>;

pub mod state {
    use super::*;

    pub trait State {
        fn symbols(&self) -> impl Resolver;
    }
    // includes interner
    impl<T: Resolver> State for T {
        fn symbols(&self) -> impl Resolver {
            self
        }
    }
}
