pub mod asts;       


#[derive(Default)]
pub struct EtaCache {
    // pub asts: asts::AstArena,
    pub sources: etac_source::SourceMap,
    pub interner: etac_intern::InternerMap,
}

impl EtaCache {
    pub fn new() -> Self {
        Self::default()
    }
}
