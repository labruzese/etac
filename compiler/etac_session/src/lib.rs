//! The compilation session: the state every phase of one `etac` run shares.

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
const _: () = assert_send_sync::<Session>();

/// Everything that lives for a whole compilation: the source text of every file read so far, and the interned
/// strings.
///
/// `S` is the phase the session is in. Parsing interns new strings, so it holds a writable interner; once parsing is
/// done, [`finish_parse`](Session::finish_parse) freezes it into a reader.
pub struct Session<S: state::State = state::Parse> {
    pub sources: etac_source::SourceMap,
    pub state: S,
}

impl Default for Session<state::Parse> {
    fn default() -> Self {
        Self {
            sources: Default::default(),
            state: state::Parse {
                interner: etac_intern::EtacInterner::with_hasher(BuildHasherDefault::new()),
            },
        }
    }
}

impl Session {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn finish_parse(self) -> Session<state::Typecheck> {
        let state = state::Typecheck {
            interner: self.state.interner.into_reader(),
        };
        Session {
            sources: self.sources,
            state,
        }
    }
}
