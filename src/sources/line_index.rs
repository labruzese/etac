/// Precomputed byte-offset -> (line, column) map for one source text.
/// Build: O(n) scan. Query: O(log n).
#[derive(Debug)]
pub struct LineIndex {
    /// Byte offset of the first character on each line (1-indexed in output).
    line_starts: Vec<usize>,
}

impl LineIndex {
    pub fn new(text: &str) -> Self {
        let mut line_starts = Vec::with_capacity(text.len() / 40 + 1);
        line_starts.push(0);
        for (i, b) in text.bytes().enumerate() {
            if b == b'\n' {
                line_starts.push(i + 1);
            }
        }
        Self { line_starts }
    }

    /// 1-indexed (line, column). Column is a byte offset from the start of
    /// the line, matching ariadne's `Source::get_byte_line`.
    pub fn line_col(&self, offset: usize) -> (usize, usize) {
        let line_idx = self
            .line_starts
            .partition_point(|&start| start <= offset)
            .saturating_sub(1);
        (line_idx + 1, offset - self.line_starts[line_idx] + 1)
    }
}
