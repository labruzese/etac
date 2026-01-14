use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EtaSpan<'fid> {
    pub file_id: &'fid FileId,
    pub range: std::ops::Range<usize>,
}

impl<'fid> From<(&'fid FileId, std::ops::Range<usize>)> for EtaSpan<'fid> {
    fn from(value: (&'fid FileId, std::ops::Range<usize>)) -> Self {
        EtaSpan {
            file_id: value.0,
            range: value.1,
        }
    }
}
