use model::cst_node::Terminal;

use crate::{MergeError, MergedCSTNode};

pub fn merge_terminals<'a>(
    base: Option<&'a Terminal<'a>>,
    left: &'a Terminal<'a>,
    right: &'a Terminal<'a>,
) -> Result<MergedCSTNode<'a>, MergeError> {
    if left.value == right.value {
        return Ok(left.to_owned().into());
    }

    match diffy::merge(base.map_or("", |node| node.value), left.value, right.value) {
        Ok(value) => Ok(MergedCSTNode::Terminal {
            kind: left.kind,
            value,
        }),
        Err(value) => Ok(MergedCSTNode::Terminal {
            kind: left.kind,
            value,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MergedCSTNode;
    use model::{cst_node::Terminal, Point};

    fn assert_merge_is_correct_and_idempotent_with_respect_to_parent_side(
        base: Option<&Terminal>,
        parent_a: &Terminal,
        parent_b: &Terminal,
        expected_merge: &MergedCSTNode,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let merged_tree = merge_terminals(base, parent_a, parent_b)?;
        let merged_tree_swap = merge_terminals(base, parent_b, parent_a)?;

        assert_eq!(expected_merge, &merged_tree);
        assert_eq!(expected_merge, &merged_tree_swap);
        Ok(())
    }

    #[test]
    fn if_i_am_merging_three_unchanged_nodes_it_is_a_success(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let node = Terminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            value: "value",
            is_block_end_delimiter: false,
        };

        assert_merge_is_correct_and_idempotent_with_respect_to_parent_side(
            Some(&node),
            &node,
            &node,
            &node.clone().into(),
        )
    }

    #[test]
    fn returns_success_if_there_are_changes_in_both_parents_and_they_are_not_conflicting(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let base = Terminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            value: "\nvalue\n",
            is_block_end_delimiter: false,
        };
        let left = Terminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            value: "left\nvalue\n",
            is_block_end_delimiter: false,
        };
        let right = Terminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            value: "\nvalue\nright",
            is_block_end_delimiter: false,
        };

        assert_merge_is_correct_and_idempotent_with_respect_to_parent_side(
            Some(&base),
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
        let base = Terminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            value: "value",
            is_block_end_delimiter: false,
        };
        let left = Terminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            value: "left_value",
            is_block_end_delimiter: false,
        };
        let right = Terminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            value: "right_value",
            is_block_end_delimiter: false,
        };

        assert_eq!(
            merge_terminals(Some(&base), &left, &right)?,
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
        let base_and_left = Terminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            value: "value",
            is_block_end_delimiter: false,
        };
        let changed_parent = Terminal {
            id: uuid::Uuid::new_v4(),
            kind: "kind",
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 7 },
            value: "value_right",
            is_block_end_delimiter: false,
        };

        assert_merge_is_correct_and_idempotent_with_respect_to_parent_side(
            Some(&base_and_left),
            &base_and_left,
            &changed_parent,
            &changed_parent.clone().into(),
        )
    }
}
