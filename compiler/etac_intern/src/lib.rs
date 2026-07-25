use std::{collections::HashMap, hash::BuildHasherDefault};
use lasso::{Key, Rodeo, RodeoReader, Spur};
use rustc_hash::FxHasher;
use etac_source::FileId;

#[derive(Default)]
pub struct InternerMap {
    map: HashMap<u32, RodeoReader>,
}

impl InternerMap {
    pub fn insert<'sm>(&mut self, id: FileId<'sm>, reader: RodeoReader) {
        self.map.insert(id.base, reader);
    }
    pub fn get<'sm>(&self, id: FileId<'sm>) -> Option<&RodeoReader> {
        self.map.get(&id.base)
    }
}

type FxBuild = BuildHasherDefault<FxHasher>;
pub type InternReader = RodeoReader<Symbol, FxBuild>;

pub type Symbol = Spur;

macro_rules! keywords {
    ($($field:ident => $text:literal),* $(,)?) => {
        #[derive(Copy, Clone, Debug)]
        pub struct Keywords {
            $(pub $field: Symbol,)*
        }

        impl Keywords {
            /// Must be called on a *fresh* interner so these land at indices 0..N.
            fn prefill(rodeo: &mut Rodeo<Symbol, FxBuild>) -> Self {
                Self { $($field: rodeo.get_or_intern_static($text),)* }
            }

            pub const COUNT: usize = [$(stringify!($field)),*].len();
        }
    };
}

keywords! {
    kw_use    => "use",
    kw_length => "length",
    kw_while  => "while",
    kw_if     => "if",
    kw_else   => "else",
    kw_return => "return",
    kw_int    => "int",
    kw_bool   => "bool",
    kw_true   => "true",
    kw_false  => "false",
}

pub struct Interner {
    rodeo: Rodeo<Symbol, FxBuild>,
    pub kw: Keywords,
}

impl Interner {
    pub fn new() -> Self {
        let mut rodeo = Rodeo::with_hasher(FxBuild::default());
        let kw = Keywords::prefill(&mut rodeo);
        Interner { rodeo, kw }
    }

    #[inline]
    pub fn intern(&mut self, s: &str) -> Symbol {
        self.rodeo.get_or_intern(s)
    }

    #[inline]
    pub fn resolve(&self, sym: Symbol) -> &str {
        self.rodeo.resolve(&sym)
    }

    #[inline]
    pub fn is_keyword(sym: Symbol) -> bool {
        sym.into_usize() < Keywords::COUNT
    }

    /// Call once parsing is done but resolution becomes contention-free and `Sync`.
    pub fn into_reader(self) -> RodeoReader<Symbol, FxBuild> {
        self.rodeo.into_reader()
    }
}

impl Default for Interner {
    fn default() -> Self { Self::new() }
}
