mod odered_merge;

use matching::Matchings;
use model::CSTNode;
use odered_merge::ordered_merge;

pub fn merge(
    base: &CSTNode,
    left: &CSTNode,
    right: &CSTNode,
    base_left_matchings: &Matchings,
    base_right_matchings: &Matchings,
    left_right_matchings: &Matchings,
) -> CSTNode {
    return ordered_merge(
        base,
        left,
        right,
        base_left_matchings,
        base_right_matchings,
        left_right_matchings,
    );
}
