mod matching;
mod matching_entry;
mod matchings;
mod ordered_tree_matching;
mod unordered_tree_matching;

pub use matching_entry::MatchingEntry;
pub use matchings::Matchings;
pub use ordered_tree_matching::ordered_tree_matching;

pub fn calculate_matchings<'a>(
    left: &'a model::CSTNode,
    right: &'a model::CSTNode,
) -> Matchings<'a> {
    if left.can_be_matching_unordered() && right.can_be_matching_unordered() {
        unordered_tree_matching::unordered_tree_matching(left, right)
    } else {
        ordered_tree_matching::ordered_tree_matching(left, right)
    }
}
