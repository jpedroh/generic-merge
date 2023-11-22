use std::collections::HashSet;

use matching::Matchings;
use model::CSTNode;

use crate::{merge, MergedCSTNode};

pub fn unordered_merge<'a>(
    base: &'a CSTNode<'a>,
    left: &'a CSTNode<'a>,
    right: &'a CSTNode<'a>,
    base_left_matchings: &'a Matchings<'a>,
    base_right_matchings: &'a Matchings<'a>,
    left_right_matchings: &'a Matchings<'a>,
) -> MergedCSTNode<'a> {
    match (base, left, right) {
        (
            CSTNode::Terminal {
                kind,
                value: value_base,
                ..
            },
            CSTNode::Terminal {
                value: value_left, ..
            },
            CSTNode::Terminal {
                value: value_right, ..
            },
        ) => {
            // Unchanged
            if value_left == value_base && value_right == value_base {
                base.to_owned().into()
            // Changed in both
            } else if value_left != value_base && value_right != value_base {
                match diffy::merge(value_base, value_left, value_right) {
                    Ok(value) => MergedCSTNode::Terminal { kind, value },
                    Err(value) => MergedCSTNode::Terminal { kind, value },
                }
            // Only left changed
            } else if value_left != value_base {
                left.to_owned().into()
            // Only right changed
            } else {
                right.to_owned().into()
            }
        }
        (
            CSTNode::NonTerminal { kind, .. },
            CSTNode::NonTerminal {
                children: children_left,
                ..
            },
            CSTNode::NonTerminal { .. },
        ) => {
            let mut result_children = vec![];
            let mut processed_children = HashSet::<&CSTNode>::new();

            for left_node in children_left.iter() {
                let matching_base_left = base_left_matchings.find_matching_for(left_node);
                let matching_left_right = left_right_matchings.find_matching_for(left_node);

                match (matching_base_left, matching_left_right) {
                    // Added only by left
                    (None, None) => {
                        result_children.push(left_node.to_owned().into());
                        processed_children.insert(left_node);
                    }
                    (None, Some(_)) => todo!(),
                    (Some(_), None) => todo!(),
                    (Some(_), Some(right_matching)) => {
                        result_children.push(merge(
                            left_node,
                            left_node,
                            right_matching.matching_node,
                            base_left_matchings,
                            base_right_matchings,
                            left_right_matchings,
                        ));
                        processed_children.insert(left_node);
                        processed_children.insert(right_matching.matching_node);
                    }
                }
            }

            MergedCSTNode::NonTerminal {
                kind,
                children: result_children,
            }
        }
        (_, _, _) => panic!("Can not merge Terminal with Non-Terminal"),
    }
}

#[cfg(test)]
mod tests {
    use matching::unordered_tree_matching;
    use model::CSTNode;

    use crate::MergedCSTNode;

    use super::unordered_merge;

    fn _assert_merge_is_correct_and_idempotent_with_respect_to_parent_side(
        base: &CSTNode,
        parent_a: &CSTNode,
        parent_b: &CSTNode,
        expected_merge: &MergedCSTNode,
    ) {
        let matchings_base_parent_a = unordered_tree_matching(base, parent_a);
        let matchings_base_parent_b = unordered_tree_matching(base, parent_b);
        let matchings_parents = unordered_tree_matching(parent_a, parent_b);

        let merged_tree = unordered_merge(
            base,
            parent_a,
            parent_b,
            &matchings_base_parent_a,
            &matchings_base_parent_b,
            &matchings_parents,
        );
        let merged_tree_swap = unordered_merge(
            base,
            parent_b,
            parent_a,
            &matchings_base_parent_b,
            &matchings_base_parent_a,
            &matchings_parents,
        );

        assert_eq!(expected_merge, &merged_tree);
        assert_eq!(expected_merge, &merged_tree_swap)
    }

    fn assert_merge_output_is(
        base: &CSTNode,
        parent_a: &CSTNode,
        parent_b: &CSTNode,
        expected_merge: &MergedCSTNode,
    ) {
        let matchings_base_parent_a = unordered_tree_matching(base, parent_a);
        let matchings_base_parent_b = unordered_tree_matching(base, parent_b);
        let matchings_parents = unordered_tree_matching(parent_a, parent_b);

        let merged_tree = unordered_merge(
            base,
            parent_a,
            parent_b,
            &matchings_base_parent_a,
            &matchings_base_parent_b,
            &matchings_parents,
        );

        assert_eq!(expected_merge, &merged_tree);
    }

    #[test]
    fn test_merge_node_added_only_by_left() {
        let base = CSTNode::NonTerminal {
            kind: "interface_body",
            start_position: model::Point { row: 0, column: 0 },
            end_position: model::Point { row: 0, column: 0 },
            children: vec![
                CSTNode::Terminal {
                    kind: "{",
                    value: "{",
                    start_position: model::Point { row: 0, column: 1 },
                    end_position: model::Point { row: 0, column: 1 },
                },
                CSTNode::Terminal {
                    kind: "}",
                    value: "}",
                    start_position: model::Point { row: 1, column: 1 },
                    end_position: model::Point { row: 1, column: 1 },
                },
            ],
        };

        let left = CSTNode::NonTerminal {
            kind: "interface_body",
            start_position: model::Point { row: 0, column: 0 },
            end_position: model::Point { row: 0, column: 0 },
            children: vec![
                CSTNode::Terminal {
                    kind: "{",
                    value: "{",
                    start_position: model::Point { row: 0, column: 1 },
                    end_position: model::Point { row: 0, column: 1 },
                },
                CSTNode::Terminal {
                    kind: "method_declaration",
                    value: "main",
                    start_position: model::Point { row: 1, column: 0 },
                    end_position: model::Point { row: 1, column: 4 },
                },
                CSTNode::Terminal {
                    kind: "}",
                    value: "}",
                    start_position: model::Point { row: 2, column: 1 },
                    end_position: model::Point { row: 2, column: 1 },
                },
            ],
        };

        let right = CSTNode::NonTerminal {
            kind: "interface_body",
            start_position: model::Point { row: 0, column: 0 },
            end_position: model::Point { row: 0, column: 0 },
            children: vec![
                CSTNode::Terminal {
                    kind: "{",
                    value: "{",
                    start_position: model::Point { row: 0, column: 1 },
                    end_position: model::Point { row: 0, column: 1 },
                },
                CSTNode::Terminal {
                    kind: "}",
                    value: "}",
                    start_position: model::Point { row: 1, column: 1 },
                    end_position: model::Point { row: 1, column: 1 },
                },
            ],
        };

        let merge = MergedCSTNode::NonTerminal {
            kind: "interface_body",
            children: vec![
                MergedCSTNode::Terminal {
                    kind: "{",
                    value: String::from("{"),
                },
                MergedCSTNode::Terminal {
                    kind: "method_declaration",
                    value: String::from("main"),
                },
                MergedCSTNode::Terminal {
                    kind: "}",
                    value: String::from("}"),
                },
            ],
        };

        assert_merge_output_is(&base, &left, &right, &merge);
    }
}
