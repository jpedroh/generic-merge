use crate::{calculate_matchings, matching_entry::MatchingEntry, MatchingHandlers, Matchings};
use model::{cst_node::NonTerminal, CSTNode};
use unordered_pair::UnorderedPair;

#[derive(PartialEq, Eq, Debug, Clone)]
enum Direction {
    Top,
    Left,
    Diag,
}

#[derive(Clone)]
struct Entry<'a>(pub Direction, pub Matchings<'a>);

impl<'a> Default for Entry<'a> {
    fn default() -> Self {
        Self(Direction::Top, Default::default())
    }
}

pub fn ordered_tree_matching<'a>(
    left: &'a CSTNode,
    right: &'a CSTNode,
    matching_handlers: &'a MatchingHandlers<'a>,
) -> Matchings<'a> {
    match (left, right) {
        (
            CSTNode::NonTerminal(NonTerminal {
                children: children_left,
                ..
            }),
            CSTNode::NonTerminal(NonTerminal {
                children: children_right,
                ..
            }),
        ) => {
            let root_matching: usize = matching_handlers
                .compute_matching_score(left, right)
                .unwrap_or((left.kind() == right.kind()).into());

            // Node roots do not match, early return
            if root_matching == 0 {
                return Matchings::empty();
            }

            let m = children_left.len();
            let n = children_right.len();

            let mut matrix_m = vec![vec![0; n + 1]; m + 1];
            let mut matrix_t = vec![vec![Entry::default(); n + 1]; m + 1];

            for i in 1..m + 1 {
                for j in 1..n + 1 {
                    let left_child = children_left.get(i - 1).unwrap();
                    let right_child = children_right.get(j - 1).unwrap();

                    let w = calculate_matchings(left_child, right_child, matching_handlers);
                    let matching = w
                        .get_matching_entry(left_child, right_child)
                        .unwrap_or_default();

                    if matrix_m[i][j - 1] > matrix_m[i - 1][j] {
                        if matrix_m[i][j - 1] > matrix_m[i - 1][j - 1] + matching.score {
                            matrix_m[i][j] = matrix_m[i][j - 1];
                            matrix_t[i][j] = Entry(Direction::Left, w);
                        } else {
                            matrix_m[i][j] = matrix_m[i - 1][j - 1] + matching.score;
                            matrix_t[i][j] = Entry(Direction::Diag, w);
                        }
                    } else if matrix_m[i - 1][j] > matrix_m[i - 1][j - 1] + matching.score {
                        matrix_m[i][j] = matrix_m[i - 1][j];
                        matrix_t[i][j] = Entry(Direction::Top, w);
                    } else {
                        matrix_m[i][j] = matrix_m[i - 1][j - 1] + matching.score;
                        matrix_t[i][j] = Entry(Direction::Diag, w);
                    }
                }
            }

            let mut i = m;
            let mut j = n;

            let mut matchings = Matchings::from_single(
                UnorderedPair(left, right),
                MatchingEntry::new(
                    matrix_m[m][n] + root_matching,
                    left.contents() == right.contents(),
                ),
            );

            while i >= 1 && j >= 1 {
                match matrix_t.get(i).unwrap().get(j).unwrap().0 {
                    Direction::Top => i -= 1,
                    Direction::Left => j -= 1,
                    Direction::Diag => {
                        if matrix_m[i][j] > matrix_m[i - 1][j - 1] {
                            matchings.extend(matrix_t[i][j].1.clone());
                        }
                        i -= 1;
                        j -= 1;
                    }
                }
            }

            matchings
        }
        (_, _) => Matchings::empty(),
    }
}

#[cfg(test)]
mod tests {
    use crate::{matching_entry::MatchingEntry, *};
    use model::{
        cst_node::{NonTerminal, Terminal},
        language, CSTNode, Point,
    };

    #[test]
    fn it_matches_deep_nodes_as_well() {
        let child = CSTNode::Terminal(Terminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_b",
            value: "value_b",
            start_position: Point { row: 1, column: 0 },
            end_position: Point { row: 1, column: 7 },
            is_block_end_delimiter: false,
        });
        let left = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_a",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 1, column: 7 },
            children: vec![child.clone()],
        });
        let right = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_a",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 1, column: 7 },
            children: vec![child.clone()],
        });

        let binding = MatchingHandlers::from(language::Language::Java);
        let matchings = ordered_tree_matching(&left, &right, &binding);

        assert_eq!(
            Some(&MatchingEntry::new(1, true)),
            matchings.get_matching_entry(&child, &child)
        )
    }

    #[test]
    fn if_no_match_is_found_it_returns_none() {
        let left_child = CSTNode::Terminal(Terminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_b",
            value: "value_b",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            is_block_end_delimiter: false,
        });
        let right_child = CSTNode::Terminal(Terminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_c",
            value: "value_c",
            start_position: Point { row: 1, column: 0 },
            end_position: Point { row: 1, column: 7 },
            is_block_end_delimiter: false,
        });

        let left = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_a",
            are_children_unordered: false,
            children: vec![left_child.clone()],
            start_position: Point { row: 1, column: 0 },
            end_position: Point { row: 0, column: 7 },
        });
        let right = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_a",
            are_children_unordered: false,
            children: vec![right_child.clone()],
            start_position: Point { row: 1, column: 0 },
            end_position: Point { row: 0, column: 7 },
        });

        let binding = MatchingHandlers::from(language::Language::Java);
        let matchings = ordered_tree_matching(&left, &right, &binding);

        assert_eq!(
            None,
            matchings.get_matching_entry(&left_child, &right_child)
        )
    }

    #[test]
    fn the_matching_between_two_subtrees_is_the_sum_of_the_matchings_plus_the_root() {
        let common_child = CSTNode::Terminal(Terminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_b",
            value: "value_b",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            is_block_end_delimiter: false,
        });
        let unique_right_child = CSTNode::Terminal(Terminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_c",
            value: "value_c",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            is_block_end_delimiter: false,
        });

        let left = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_a",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![common_child.clone()],
        });
        let right = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_a",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![common_child.clone(), unique_right_child],
        });

        let binding = MatchingHandlers::from(language::Language::Java);
        let matchings = ordered_tree_matching(&left, &right, &binding);

        assert_eq!(
            Some(&MatchingEntry::new(2, false)),
            matchings.get_matching_entry(&left, &right)
        )
    }

    #[test]
    fn perfect_matching_deep_nodes() {
        let common_child = CSTNode::Terminal(Terminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_b",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            value: "value_b",
            is_block_end_delimiter: false,
        });

        let left = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_a",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![common_child.clone()],
        });
        let right = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_a",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![common_child.clone()],
        });

        let binding = MatchingHandlers::from(language::Language::Java);
        let matchings = ordered_tree_matching(&left, &right, &binding);

        assert_eq!(
            Some(&MatchingEntry::new(2, true)),
            matchings.get_matching_entry(&left, &right)
        )
    }

    #[test]
    fn perfect_matching_deeper_nodes() {
        let leaf = CSTNode::Terminal(Terminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_b",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            value: "value_b",
            is_block_end_delimiter: false,
        });

        let intermediate = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "intermediate",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![leaf],
        });

        let left = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_a",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![intermediate.clone()],
        });
        let right = CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind_a",
            are_children_unordered: false,
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            children: vec![intermediate.clone()],
        });

        let binding = MatchingHandlers::from(language::Language::Java);
        let matchings = ordered_tree_matching(&left, &right, &binding);

        assert_eq!(
            Some(&MatchingEntry::new(2, true)),
            matchings.get_matching_entry(&intermediate, &intermediate)
        );

        assert_eq!(
            Some(&MatchingEntry::new(3, true)),
            matchings.get_matching_entry(&left, &right)
        )
    }
}
