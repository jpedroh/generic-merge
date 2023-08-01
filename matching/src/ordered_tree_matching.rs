use crate::{matching_entry::MatchingEntry, Matchings};
use model::CSTNode;
use std::collections::HashMap;
use utils::unordered_pair::UnorderedPair;

#[derive(PartialEq, Eq, Debug, Clone)]
enum Direction {
    TOP,
    LEFT,
    DIAG,
}

#[derive(Clone)]
struct Entry(
    pub Direction,
    pub HashMap<UnorderedPair<CSTNode>, MatchingEntry>,
);

impl Default for Entry {
    fn default() -> Self {
        Self(Direction::TOP, Default::default())
    }
}

pub fn ordered_tree_matching(left: &CSTNode, right: &CSTNode) -> Matchings {
    return Matchings::new(ordered_tree_matching_helper(left, right));
}

fn ordered_tree_matching_helper(
    left: &CSTNode,
    right: &CSTNode,
) -> HashMap<UnorderedPair<CSTNode>, MatchingEntry> {
    match (left, right) {
        (
            CSTNode::NonTerminal {
                kind: kind_left,
                children: children_left,
            },
            CSTNode::NonTerminal {
                kind: kind_right,
                children: children_right,
            },
        ) => {
            let root_matching: usize = (kind_left == kind_right).into();

            let m = children_left.len();
            let n = children_right.len();

            let mut matrix_m = vec![vec![0; n + 1]; m + 1];
            let mut matrix_t = vec![vec![Entry::default(); n + 1]; m + 1];

            for i in 1..m + 1 {
                for j in 1..n + 1 {
                    let left_child = children_left.get(i - 1).unwrap();
                    let right_child = children_right.get(j - 1).unwrap();

                    let w = ordered_tree_matching_helper(left_child, right_child);
                    let matching = w
                        .get(&UnorderedPair::new(
                            left_child.to_owned(),
                            right_child.to_owned(),
                        ))
                        .unwrap();

                    if matrix_m[i][j - 1] > matrix_m[i - 1][j] {
                        if matrix_m[i][j - 1] > matrix_m[i - 1][j - 1] + matching.score {
                            matrix_m[i][j] = matrix_m[i][j - 1];
                            matrix_t[i][j] = Entry(Direction::LEFT, w);
                        } else {
                            matrix_m[i][j] = matrix_m[i - 1][j - 1] + matching.score;
                            matrix_t[i][j] = Entry(Direction::DIAG, w);
                        }
                    } else {
                        if matrix_m[i - 1][j] > matrix_m[i - 1][j - 1] + matching.score {
                            matrix_m[i][j] = matrix_m[i - 1][j];
                            matrix_t[i][j] = Entry(Direction::TOP, w);
                        } else {
                            matrix_m[i][j] = matrix_m[i - 1][j - 1] + matching.score;
                            matrix_t[i][j] = Entry(Direction::DIAG, w);
                        }
                    }
                }
            }

            let mut i = m;
            let mut j = n;
            let mut children = Vec::<&HashMap<UnorderedPair<CSTNode>, MatchingEntry>>::new();

            while i >= 1 && j >= 1 {
                match matrix_t.get(i).unwrap().get(j).unwrap().0 {
                    Direction::TOP => i = i - 1,
                    Direction::LEFT => j = j - 1,
                    Direction::DIAG => {
                        if matrix_m[i][j] > matrix_m[i - 1][j - 1] {
                            children.push(&matrix_t[i][j].1);
                        }
                        i = i - 1;
                        j = j - 1;
                    }
                }
            }

            let matching = MatchingEntry {
                score: matrix_m[m][n] + root_matching,
            };
            let mut result = HashMap::new();
            result.insert(
                UnorderedPair::new(left.to_owned(), right.to_owned()),
                matching,
            );
            children.into_iter().for_each(|child_matchings| {
                child_matchings.iter().for_each(|(key, matching)| {
                    result.insert(key.to_owned(), matching.to_owned());
                })
            });
            result
        }
        (
            CSTNode::Terminal {
                kind: kind_left,
                value: value_left,
            },
            CSTNode::Terminal {
                kind: kind_right,
                value: value_right,
            },
        ) => {
            let mut result = HashMap::new();
            result.insert(
                UnorderedPair::new(left.to_owned(), right.to_owned()),
                MatchingEntry {
                    score: (kind_left == kind_right && value_left == value_right).into(),
                },
            );
            result
        }
        (_, _) => {
            let mut result = HashMap::new();
            result.insert(
                UnorderedPair::new(left.to_owned(), right.to_owned()),
                MatchingEntry { score: 0 },
            );
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{matching_entry::MatchingEntry, *};
    use model::CSTNode;

    #[test]
    fn two_terminal_nodes_matches_with_a_score_of_one_if_they_have_the_same_kind_and_value() {
        let left = CSTNode::Terminal {
            kind: "kind".to_owned(),
            value: "value".to_owned(),
        };
        let right = CSTNode::Terminal {
            kind: "kind".to_owned(),
            value: "value".to_owned(),
        };

        let matchings = ordered_tree_matching(&left, &right);

        assert_eq!(
            Some(&MatchingEntry::with_score(1)),
            matchings.get_matching_entry(left, right)
        )
    }

    #[test]
    fn two_terminal_nodes_have_a_match_with_score_zero_if_they_have_different_value() {
        let left = CSTNode::Terminal {
            kind: "kind".to_owned(),
            value: "value_a".to_owned(),
        };
        let right = CSTNode::Terminal {
            kind: "kind".to_owned(),
            value: "value_b".to_owned(),
        };

        let matchings = ordered_tree_matching(&left, &right);

        assert_eq!(
            Some(&MatchingEntry::with_score(0)),
            matchings.get_matching_entry(left, right)
        )
    }

    #[test]
    fn two_terminal_nodes_have_a_match_with_score_zero_if_they_have_different_kind() {
        let left = CSTNode::Terminal {
            kind: "kind_a".to_owned(),
            value: "value".to_owned(),
        };
        let right = CSTNode::Terminal {
            kind: "kind_b".to_owned(),
            value: "value".to_owned(),
        };

        let matchings = ordered_tree_matching(&left, &right);

        assert_eq!(
            Some(&MatchingEntry::with_score(0)),
            matchings.get_matching_entry(left, right)
        )
    }

    #[test]
    fn two_terminal_nodes_have_a_match_with_score_zero_if_they_have_different_kind_and_value() {
        let left = CSTNode::Terminal {
            kind: "kind_a".to_owned(),
            value: "value_a".to_owned(),
        };
        let right = CSTNode::Terminal {
            kind: "kind_b".to_owned(),
            value: "value_a".to_owned(),
        };

        let matchings = ordered_tree_matching(&left, &right);

        assert_eq!(
            Some(&MatchingEntry::with_score(0)),
            matchings.get_matching_entry(left, right)
        )
    }

    #[test]
    fn it_matches_deep_nodes_as_well() {
        let child = CSTNode::Terminal {
            kind: "kind_b".into(),
            value: "value_b".into(),
        };
        let left = CSTNode::NonTerminal {
            kind: "kind_a".to_owned(),
            children: vec![child.clone()],
        };
        let right = CSTNode::NonTerminal {
            kind: "kind_a".to_owned(),
            children: vec![child.clone()],
        };

        let matchings = ordered_tree_matching(&left, &right);

        assert_eq!(
            Some(&MatchingEntry::with_score(1)),
            matchings.get_matching_entry(child.clone(), child)
        )
    }

    #[test]
    fn if_no_match_is_found_it_returns_none() {
        let left_child = CSTNode::Terminal {
            kind: "kind_b".into(),
            value: "value_b".into(),
        };
        let right_child = CSTNode::Terminal {
            kind: "kind_c".into(),
            value: "value_c".into(),
        };

        let left = CSTNode::NonTerminal {
            kind: "kind_a".to_owned(),
            children: vec![left_child.clone()],
        };
        let right = CSTNode::NonTerminal {
            kind: "kind_a".to_owned(),
            children: vec![right_child.clone()],
        };

        let matchings = ordered_tree_matching(&left, &right);

        assert_eq!(None, matchings.get_matching_entry(left_child, right_child))
    }

    #[test]
    fn the_matching_between_two_subtrees_is_the_sum_of_the_matchings_plus_the_root() {
        let common_child = CSTNode::Terminal {
            kind: "kind_b".into(),
            value: "value_b".into(),
        };
        let unique_right_child = CSTNode::Terminal {
            kind: "kind_c".into(),
            value: "value_c".into(),
        };

        let left = CSTNode::NonTerminal {
            kind: "kind_a".to_owned(),
            children: vec![common_child.clone()],
        };
        let right = CSTNode::NonTerminal {
            kind: "kind_a".to_owned(),
            children: vec![common_child.clone(), unique_right_child],
        };

        let matchings = ordered_tree_matching(&left, &right);

        assert_eq!(
            Some(&MatchingEntry::with_score(2)),
            matchings.get_matching_entry(left, right)
        )
    }
}
