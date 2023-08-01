#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MatchingEntry {
    pub score: usize,
}

impl MatchingEntry {
    pub fn with_score(score: usize) -> Self {
        return MatchingEntry { score };
    }
}
