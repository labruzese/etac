use std::hash::BuildHasherDefault;
use lasso::{Rodeo, RodeoReader, Spur};
use rustc_hash::FxHasher;
type FxBuild = BuildHasherDefault<FxHasher>;

pub use lasso::Reader;
pub use lasso::Interner;

pub type Symbol = Spur;
pub type EtacInterner = Rodeo<Symbol, FxBuild>;
pub type EtacReader = RodeoReader<Symbol, FxBuild>;

pub mod state {
    use super::*;

    pub trait State {
        fn symbols(&self) -> impl Reader;
    }
    // includes interner
    impl<T: Reader> State for T {
        fn symbols(&self) -> impl Reader {
            self
        }
    }
}
