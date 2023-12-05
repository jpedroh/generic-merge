use model::CSTNode;
use unordered_pair::UnorderedPair;

use crate::{calculate_matchings, MatchingEntry, Matchings};

pub fn unordered_tree_matching<'a>(left: &'a CSTNode, right: &'a CSTNode) -> crate::Matchings<'a> {
    match (left, right) {
        (
            CSTNode::Terminal {
                kind: kind_left,
                value: value_left,
                ..
            },
            CSTNode::Terminal {
                kind: kind_right,
                value: value_right,
                ..
            },
        ) => {
            let is_perfetch_match = kind_left == kind_right && value_left == value_right;
            Matchings::from_single(
                UnorderedPair(left, right),
                MatchingEntry::new(is_perfetch_match.into(), is_perfetch_match),
            )
        }
        (
            CSTNode::NonTerminal {
                kind: kind_left,
                children: children_left,
                ..
            },
            CSTNode::NonTerminal {
                kind: kind_right,
                children: children_right,
                ..
            },
        ) => {
            let root_matching: usize = (kind_left == kind_right).into();

            let mut sum = 0;
            let mut result = Matchings::empty();

            for child_left in children_left {
                for child_right in children_right {
                    let matching_score = compute_matching_score(child_left, child_right);

                    if matching_score == 1 {
                        let child_matching = calculate_matchings(child_left, child_right);
                        sum += child_matching
                            .get_matching_entry(child_left, child_right)
                            .map_or(0, |matching| matching.score);
                        result.extend(child_matching);
                    }
                }
            }

            result.extend(Matchings::from_single(
                UnorderedPair(left, right),
                MatchingEntry {
                    score: sum + root_matching,
                    is_perfect_match: left == right,
                },
            ));

            result
        }
        (_, _) => panic!("Invalid configuration reached"),
    }
}

fn compute_matching_score<'a>(left: &'a CSTNode, right: &'a CSTNode) -> usize {
    match (left, right) {
        (
            CSTNode::Terminal {
                kind: kind_left,
                value: value_left,
                ..
            },
            CSTNode::Terminal {
                kind: kind_right,
                value: value_right,
                ..
            },
        ) => (kind_left == kind_right && value_left == value_right).into(),
        (
            CSTNode::NonTerminal {
                children: children_left,
                ..
            },
            CSTNode::NonTerminal {
                children: children_right,
                ..
            },
        ) => {
            // Try to find an identifier on children, and compare them
            let identifier_left = children_left.iter().find(|node| match node {
                CSTNode::Terminal { kind, .. } => kind == &"identifier",
                _ => false,
            });

            let identifier_right = children_right.iter().find(|node| match node {
                CSTNode::Terminal { kind, .. } => kind == &"identifier",
                _ => false,
            });

            match (identifier_left, identifier_right) {
                (Some(identifier_left), Some(identifier_right)) => {
                    match (identifier_left, identifier_right) {
                        (
                            CSTNode::Terminal {
                                value: value_left, ..
                            },
                            CSTNode::Terminal {
                                value: value_right, ..
                            },
                        ) if value_left == value_right => 1,
                        (_, _) => 0,
                    }
                }
                (_, _) => 0,
            }
        }
        (_, _) => 0,
    }
}
