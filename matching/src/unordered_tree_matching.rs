use matching_handlers::MatchingHandlers;
use model::{
    cst_node::{NonTerminal, Terminal},
    CSTNode,
};
use unordered_pair::UnorderedPair;

use crate::{calculate_matchings, MatchingEntry, Matchings};

pub fn unordered_tree_matching<'a>(
    left: &'a CSTNode,
    right: &'a CSTNode,
    matching_handlers: &'a MatchingHandlers<'a>,
) -> crate::Matchings<'a> {
    match (left, right) {
        (
            CSTNode::Terminal(Terminal {
                kind: kind_left,
                value: value_left,
                ..
            }),
            CSTNode::Terminal(Terminal {
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
        (
            CSTNode::NonTerminal(NonTerminal {
                kind: kind_left,
                children: children_left,
                ..
            }),
            CSTNode::NonTerminal(NonTerminal {
                kind: kind_right,
                children: children_right,
                ..
            }),
        ) => {
            let root_matching: usize = (kind_left == kind_right).into();

            let mut sum = 0;
            let mut result = Matchings::empty();

            for child_left in children_left {
                for child_right in children_right {
                    let matching_score =
                        calculate_matchings(child_left, child_right, matching_handlers);
                    log::debug!(
                        "{:?}",
                        matching_score.get_matching_entry(child_left, child_right)
                    );

                    if let Some(matching_entry) =
                        matching_score.get_matching_entry(child_left, child_right)
                    {
                        if matching_entry.score >= 1 {
                            sum += matching_entry.score;
                            result.extend(matching_score);
                        }
                    }
                }
            }

            result.extend(Matchings::from_single(
                UnorderedPair(left, right),
                MatchingEntry {
                    score: sum + root_matching,
                    is_perfect_match: left.contents() == right.contents(),
                },
            ));

            result
        }
        (_, _) => panic!("Invalid configuration reached"),
    }
}
