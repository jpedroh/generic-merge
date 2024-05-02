mod matching;
pub mod matching_configuration;
mod matching_entry;
mod matchings;
pub mod ordered;
pub mod unordered;

use matching_configuration::MatchingConfiguration;
pub use matching_entry::MatchingEntry;
pub use matchings::Matchings;
use model::cst_node::Terminal;
use unordered_pair::UnorderedPair;

pub fn calculate_matchings<'a>(
    left: &'a model::CSTNode,
    right: &'a model::CSTNode,
    config: &'a MatchingConfiguration<'a>,
) -> Matchings<'a> {
    match (left, right) {
        (
            model::CSTNode::NonTerminal(non_terminal_left),
            model::CSTNode::NonTerminal(non_terminal_right),
        ) => {
            if non_terminal_left.are_children_unordered && non_terminal_right.are_children_unordered
            {
                unordered::calculate_matchings(left, right, config)
            } else {
                ordered::calculate_matchings(left, right, config)
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
            let is_perfect_match = kind_left == kind_right && value_left == value_right;
            Matchings::from_single(
                UnorderedPair(left, right),
                MatchingEntry::new(is_perfect_match.into(), is_perfect_match),
            )
        }
        (_, _) => Matchings::from_single(UnorderedPair(left, right), MatchingEntry::new(0, false)),
    }
}

#[cfg(test)]
mod tests {
    use model::{cst_node::Terminal, CSTNode, Point};

    use crate::{
        calculate_matchings, matching_configuration::MatchingConfiguration, MatchingEntry,
    };

    #[test]
    fn two_terminal_nodes_matches_with_a_score_of_one_if_they_have_the_same_kind_and_value() {
        let left = CSTNode::Terminal(Terminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            value: "value",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 5 },
            is_block_end_delimiter: false,
        });
        let right = CSTNode::Terminal(Terminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            value: "value",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 5 },
            is_block_end_delimiter: false,
        });

        let matching_configuration = MatchingConfiguration::default();
        let matchings = calculate_matchings(&left, &right, &matching_configuration);

        assert_eq!(
            Some(&MatchingEntry::new(1, true)),
            matchings.get_matching_entry(&left, &right)
        )
    }

    #[test]
    fn two_terminal_nodes_have_a_match_with_score_zero_if_they_have_different_value() {
        let left = CSTNode::Terminal(Terminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            value: "value_a",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            is_block_end_delimiter: false,
        });
        let right = CSTNode::Terminal(Terminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            value: "value_b",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            is_block_end_delimiter: false,
        });

        let matching_configuration = MatchingConfiguration::default();
        let matchings = calculate_matchings(&left, &right, &matching_configuration);

        assert_eq!(
            Some(&MatchingEntry::new(0, false)),
            matchings.get_matching_entry(&left, &right)
        )
    }

    #[test]
    fn two_terminal_nodes_have_a_match_with_score_zero_if_they_have_different_kind() {
        let left = CSTNode::Terminal(Terminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_a",
            value: "value",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 5 },
            is_block_end_delimiter: false,
        });
        let right = CSTNode::Terminal(Terminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_b",
            value: "value",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 5 },
            is_block_end_delimiter: false,
        });

        let matching_configuration = MatchingConfiguration::default();
        let matchings = calculate_matchings(&left, &right, &matching_configuration);

        assert_eq!(
            Some(&MatchingEntry::new(0, false)),
            matchings.get_matching_entry(&left, &right)
        )
    }

    #[test]
    fn two_terminal_nodes_have_a_match_with_score_zero_if_they_have_different_kind_and_value() {
        let left = CSTNode::Terminal(Terminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_a",
            value: "value_a",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            is_block_end_delimiter: false,
        });
        let right = CSTNode::Terminal(Terminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_b",
            value: "value_a",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            is_block_end_delimiter: false,
        });

        let matching_configuration = MatchingConfiguration::default();
        let matchings = calculate_matchings(&left, &right, &matching_configuration);

        assert_eq!(
            Some(&MatchingEntry::new(0, false)),
            matchings.get_matching_entry(&left, &right)
        )
    }
}
