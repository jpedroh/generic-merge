mod matching;
mod matching_entry;
mod matchings;
mod ordered_tree_matching;
mod unordered_tree_matching;

pub use matching_entry::MatchingEntry;
pub use matchings::Matchings;
use model::cst_node::Terminal;
pub use ordered_tree_matching::ordered_tree_matching;
use unordered_pair::UnorderedPair;
pub use unordered_tree_matching::unordered_tree_matching;

pub fn calculate_matchings<'a>(
    left: &'a model::CSTNode,
    right: &'a model::CSTNode,
) -> Matchings<'a> {
    match (left, right) {
        (model::CSTNode::NonTerminal { .. }, model::CSTNode::NonTerminal { .. }) => {
            if left.are_children_unordered() && right.are_children_unordered() {
                unordered_tree_matching::unordered_tree_matching(left, right)
            } else {
                ordered_tree_matching::ordered_tree_matching(left, right)
            }
        }
        (
            model::CSTNode::Terminal(Terminal {
                kind: kind_left,
                value: value_left,
                ..
            }),
            model::CSTNode::Terminal(Terminal {
                kind: kind_right,
                value: value_right,
                ..
            }),
        ) => {
            let is_perfetch_match = kind_left == kind_right && value_left == value_right;
            Matchings::from_single(
                UnorderedPair(left, right),
                MatchingEntry::new(is_perfetch_match.into(), is_perfetch_match),
            )
        }
        (_, _) => Matchings::from_single(UnorderedPair(left, right), MatchingEntry::new(0, false)),
    }
}

#[cfg(test)]
mod tests {
    use model::{cst_node::Terminal, CSTNode, Point};

    use crate::{calculate_matchings, MatchingEntry};

    #[test]
    fn two_terminal_nodes_matches_with_a_score_of_one_if_they_have_the_same_kind_and_value() {
        let left = CSTNode::Terminal(Terminal {
            kind: "kind",
            value: "value",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 5 },
        });
        let right = CSTNode::Terminal(Terminal {
            kind: "kind",
            value: "value",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 5 },
        });

        let matchings = calculate_matchings(&left, &right);

        assert_eq!(
            Some(&MatchingEntry::new(1, true)),
            matchings.get_matching_entry(&left, &right)
        )
    }

    #[test]
    fn two_terminal_nodes_have_a_match_with_score_zero_if_they_have_different_value() {
        let left = CSTNode::Terminal(Terminal {
            kind: "kind",
            value: "value_a",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
        });
        let right = CSTNode::Terminal(Terminal {
            kind: "kind",
            value: "value_b",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
        });

        let matchings = calculate_matchings(&left, &right);

        assert_eq!(
            Some(&MatchingEntry::new(0, false)),
            matchings.get_matching_entry(&left, &right)
        )
    }

    #[test]
    fn two_terminal_nodes_have_a_match_with_score_zero_if_they_have_different_kind() {
        let left = CSTNode::Terminal(Terminal {
            kind: "kind_a",
            value: "value",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 5 },
        });
        let right = CSTNode::Terminal(Terminal {
            kind: "kind_b",
            value: "value",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 5 },
        });

        let matchings = calculate_matchings(&left, &right);

        assert_eq!(
            Some(&MatchingEntry::new(0, false)),
            matchings.get_matching_entry(&left, &right)
        )
    }

    #[test]
    fn two_terminal_nodes_have_a_match_with_score_zero_if_they_have_different_kind_and_value() {
        let left = CSTNode::Terminal(Terminal {
            kind: "kind_a",
            value: "value_a",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
        });
        let right = CSTNode::Terminal(Terminal {
            kind: "kind_b",
            value: "value_a",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
        });

        let matchings = calculate_matchings(&left, &right);

        assert_eq!(
            Some(&MatchingEntry::new(0, false)),
            matchings.get_matching_entry(&left, &right)
        )
    }
}
