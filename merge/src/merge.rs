use crate::merge_error::MergeError;
use crate::ordered_merge;
use crate::unordered_merge;
use matching::Matchings;
use model::cst_node::Terminal;
use model::CSTNode;

use crate::merged_cst_node::MergedCSTNode;

pub fn merge<'a>(
    base: &'a CSTNode<'a>,
    left: &'a CSTNode<'a>,
    right: &'a CSTNode<'a>,
    base_left_matchings: &'a Matchings<'a>,
    base_right_matchings: &'a Matchings<'a>,
    left_right_matchings: &'a Matchings<'a>,
) -> Result<MergedCSTNode<'a>, MergeError> {
    match (base, left, right) {
        (
            CSTNode::Terminal(Terminal {
                kind,
                value: value_base,
                ..
            }),
            CSTNode::Terminal(Terminal {
                value: value_left, ..
            }),
            CSTNode::Terminal(Terminal {
                value: value_right, ..
            }),
        ) => {
            // Unchanged
            if value_left == value_base && value_right == value_base {
                Ok(base.to_owned().into())
            // Changed in both
            } else if value_left != value_base && value_right != value_base {
                match diffy::merge(value_base, value_left, value_right) {
                    Ok(value) => Ok(MergedCSTNode::Terminal { kind, value }),
                    Err(value) => Ok(MergedCSTNode::Terminal { kind, value }),
                }
            // Only left changed
            } else if value_left != value_base {
                Ok(left.to_owned().into())
            // Only right changed
            } else {
                Ok(right.to_owned().into())
            }
        }
        (
            CSTNode::NonTerminal(a_base),
            CSTNode::NonTerminal(a_left),
            CSTNode::NonTerminal(a_right),
        ) => {
            if a_left.are_children_unordered && a_right.are_children_unordered {
                Ok(unordered_merge(
                    a_base,
                    a_left,
                    a_right,
                    base_left_matchings,
                    base_right_matchings,
                    left_right_matchings,
                )?)
            } else {
                Ok(ordered_merge(
                    a_base,
                    a_left,
                    a_right,
                    base_left_matchings,
                    base_right_matchings,
                    left_right_matchings,
                )?)
            }
        }
        (_, _, _) => Err(MergeError::MergingTerminalWithNonTerminal),
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use matching::{ordered_tree_matching, Matchings};
    use model::{
        cst_node::{NonTerminal, Terminal},
        CSTNode, Point,
    };

    use crate::{MergeError, MergedCSTNode};

    use super::merge;

    fn assert_merge_is_correct_and_idempotent_with_respect_to_parent_side(
        base: &CSTNode,
        parent_a: &CSTNode,
        parent_b: &CSTNode,
        expected_merge: &MergedCSTNode,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let matchings_base_parent_a = ordered_tree_matching(base, parent_a);
        let matchings_base_parent_b = ordered_tree_matching(base, parent_b);
        let matchings_parents = ordered_tree_matching(parent_a, parent_b);

        let merged_tree = merge(
            base,
            parent_a,
            parent_b,
            &matchings_base_parent_a,
            &matchings_base_parent_b,
            &matchings_parents,
        )?;
        let merged_tree_swap = merge(
            base,
            parent_b,
            parent_a,
            &matchings_base_parent_b,
            &matchings_base_parent_a,
            &matchings_parents,
        )?;

        assert_eq!(expected_merge, &merged_tree);
        assert_eq!(expected_merge, &merged_tree_swap);
        Ok(())
    }

    #[test]
    fn if_i_am_merging_three_unchanged_nodes_it_is_a_success(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let node = CSTNode::Terminal(Terminal {
            kind: "kind",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            value: "value",
        });

        assert_merge_is_correct_and_idempotent_with_respect_to_parent_side(
            &node,
            &node,
            &node,
            &node.clone().into(),
        )
    }

    #[test]
    fn returns_success_if_there_are_changes_in_both_parents_and_they_are_not_conflicting(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let base = CSTNode::Terminal(Terminal {
            kind: "kind",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            value: "\nvalue\n",
        });
        let left = CSTNode::Terminal(Terminal {
            kind: "kind",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            value: "left\nvalue\n",
        });
        let right = CSTNode::Terminal(Terminal {
            kind: "kind",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            value: "\nvalue\nright",
        });

        assert_merge_is_correct_and_idempotent_with_respect_to_parent_side(
            &base,
            &left,
            &right,
            &MergedCSTNode::Terminal {
                kind: "kind",
                value: "left\nvalue\nright".to_string(),
            },
        )
    }

    #[test]
    fn returns_conflict_if_there_are_changes_in_both_parents_and_they_are_conflicting(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let base = CSTNode::Terminal(Terminal {
            kind: "kind",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            value: "value",
        });
        let left = CSTNode::Terminal(Terminal {
            kind: "kind",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            value: "left_value",
        });
        let right = CSTNode::Terminal(Terminal {
            kind: "kind",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            value: "right_value",
        });

        assert_eq!(
            merge(&base, &left, &right, &Matchings::empty(), &Matchings::empty(),
            &Matchings::empty()).unwrap(),
           MergedCSTNode::Terminal {
                kind: "kind",
                value: "<<<<<<< ours\nleft_value||||||| original\nvalue=======\nright_value>>>>>>> theirs\n".to_string()
            }
        );

        Ok(())
    }

    #[test]
    fn if_there_is_a_change_only_in_one_parent_it_returns_the_changes_from_this_parent(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let base_and_left = CSTNode::Terminal(Terminal {
            kind: "kind",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            value: "value",
        });
        let changed_parent = CSTNode::Terminal(Terminal {
            kind: "kind",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            value: "value_right",
        });

        assert_merge_is_correct_and_idempotent_with_respect_to_parent_side(
            &base_and_left,
            &base_and_left,
            &changed_parent,
            &changed_parent.clone().into(),
        )
    }

    #[test]
    fn test_can_not_merge_terminal_with_non_terminal() -> Result<(), Box<dyn std::error::Error>> {
        let error = merge(
            &CSTNode::Terminal(Terminal {
                kind: "kind",
                start_position: Point { row: 0, column: 0 },
                end_position: Point { row: 0, column: 7 },
                value: "value",
            }),
            &CSTNode::Terminal(Terminal {
                kind: "kind",
                start_position: Point { row: 0, column: 0 },
                end_position: Point { row: 0, column: 7 },
                value: "value",
            }),
            &CSTNode::NonTerminal(NonTerminal {
                kind: "kind",
                are_children_unordered: false,
                start_position: Point { row: 0, column: 0 },
                end_position: Point { row: 0, column: 7 },
                children: vec![],
            }),
            &Matchings::empty(),
            &Matchings::empty(),
            &Matchings::empty(),
        )
        .unwrap_err();

        assert_eq!(error, MergeError::MergingTerminalWithNonTerminal);

        Ok(())
    }
}
