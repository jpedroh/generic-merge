#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MatchingEntry {
    pub score: usize,
    pub is_perfect_match: bool,
}

impl MatchingEntry {
    pub fn new(score: usize, is_perfect_match: bool) -> Self {
        return MatchingEntry {
            score,
            is_perfect_match,
        };
    }
}
