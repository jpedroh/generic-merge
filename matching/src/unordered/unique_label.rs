use model::{cst_node::NonTerminal, CSTNode};
use unordered_pair::UnorderedPair;

use crate::{matching_configuration::MatchingConfiguration, MatchingEntry, Matchings};

pub fn calculate_matchings<'a>(
    left: &'a CSTNode,
    right: &'a CSTNode,
    config: &'a MatchingConfiguration<'a>,
) -> crate::Matchings<'a> {
    match (left, right) {
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
                    let is_same_identifier = config
                        .handlers
                        .compute_matching_score(child_left, child_right)
                        .unwrap_or_else(|| (child_left.kind() == child_right.kind()).into());

                    if is_same_identifier == 1 {
                        let child_matchings =
                            crate::calculate_matchings(child_left, child_right, config);

                        if let Some(matching_entry) =
                            child_matchings.get_matching_entry(child_left, child_right)
                        {
                            if matching_entry.score >= 1 {
                                sum += matching_entry.score;
                                result.extend(child_matchings);
                            }
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
        _ => unreachable!("Unordered matching is only supported for non-terminals."),
    }
}
