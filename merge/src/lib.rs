mod merged_cst_node;
mod odered_merge;

use matching::Matchings;
use merged_cst_node::MergedCSTNode;
use model::CSTNode;
use odered_merge::ordered_merge;

pub fn merge<'a>(
    base: &'a CSTNode<'a>,
    left: &'a CSTNode<'a>,
    right: &'a CSTNode<'a>,
    base_left_matchings: &'a Matchings<'a>,
    base_right_matchings: &'a Matchings<'a>,
    left_right_matchings: &'a Matchings<'a>,
) -> MergedCSTNode<'a> {
    return ordered_merge(
        base,
        left,
        right,
        base_left_matchings,
        base_right_matchings,
        left_right_matchings,
    );
}
