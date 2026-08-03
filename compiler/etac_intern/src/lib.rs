use std::hash::BuildHasherDefault;
use lasso::{Rodeo, RodeoReader, Spur};
use rustc_hash::FxHasher;
type FxBuild = BuildHasherDefault<FxHasher>;

pub use lasso::Resolver;
pub use lasso::Interner;

pub type Symbol = Spur;
pub type EtacInterner = Rodeo<Symbol, FxBuild>;
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
