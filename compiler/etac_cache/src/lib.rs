use std::hash::BuildHasherDefault;

mod state {
    pub trait State: Send + Sync {}
    impl State for Parse {}
    impl State for Typecheck {}

    pub struct Parse {
        pub interner: etac_intern::EtacInterner,
    }

    pub struct Typecheck {
        pub interner: etac_intern::EtacReader,
    }
}

const fn assert_send_sync<T: Send + Sync>() {}
const _: () = assert_send_sync::<EtaCache>();
pub struct EtaCache<S: state::State = state::Parse> {
    pub sources: etac_source::SourceMap,
    pub state: S,
}

impl Default for EtaCache<state::Parse> {
    fn default() -> Self {
        Self {
            sources: Default::default(),
            state: state::Parse {
                interner: etac_intern::EtacInterner::with_hasher(BuildHasherDefault::new()),
            },
        }
    }
}

impl EtaCache {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn finish_parse(self) -> EtaCache<state::Typecheck> {
        let state = state::Typecheck {
            interner: self.state.interner.into_reader(),
        };
        EtaCache {
            sources: self.sources,
            state,
        }
    }
}
