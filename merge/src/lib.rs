mod merged_cst_node;
mod ordered_merge;
mod unordered_merge;

use matching::Matchings;
use merged_cst_node::MergedCSTNode;
use model::CSTNode;
use ordered_merge::ordered_merge;
use unordered_merge::unordered_merge;

pub fn merge<'a>(
    base: &'a CSTNode<'a>,
    left: &'a CSTNode<'a>,
    right: &'a CSTNode<'a>,
    base_left_matchings: &'a Matchings<'a>,
    base_right_matchings: &'a Matchings<'a>,
    left_right_matchings: &'a Matchings<'a>,
) -> MergedCSTNode<'a> {
    if right.are_children_unordered() && left.are_children_unordered() {
        unordered_merge(
            base,
            left,
            right,
            base_left_matchings,
            base_right_matchings,
            left_right_matchings,
        )
    } else {
        ordered_merge(
            base,
            left,
            right,
            base_left_matchings,
            base_right_matchings,
            left_right_matchings,
        )
    }
}
