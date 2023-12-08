use std::collections::HashSet;

use matching::Matchings;
use model::{
    cst_node::{NonTerminal, Terminal},
    CSTNode,
};

use crate::{merge, MergeError, MergedCSTNode};

pub fn unordered_merge<'a>(
    base: &'a CSTNode<'a>,
    left: &'a CSTNode<'a>,
    right: &'a CSTNode<'a>,
    base_left_matchings: &'a Matchings<'a>,
    base_right_matchings: &'a Matchings<'a>,
    left_right_matchings: &'a Matchings<'a>,
) -> Result<MergedCSTNode<'a>, MergeError> {
    match (base, left, right) {
        (
            CSTNode::NonTerminal(NonTerminal { kind, .. }),
            CSTNode::NonTerminal(NonTerminal {
                children: children_left,
                ..
            }),
            CSTNode::NonTerminal(NonTerminal {
                children: children_right,
                ..
            }),
        ) => {
            let mut result_children = vec![];
            let mut processed_nodes: HashSet<&CSTNode> = HashSet::new();

            for left_child in children_left.iter() {
                match left_child {
                    CSTNode::Terminal(Terminal { value, .. }) => {
                        if value == &"}" {
                            break;
                        }
                    }
                    CSTNode::NonTerminal { .. } => {}
                }

                let matching_base_left = base_left_matchings.find_matching_for(left_child);
                let matching_left_right = left_right_matchings.find_matching_for(left_child);

                match (matching_base_left, matching_left_right) {
                    // Added only by left
                    (None, None) => {
                        result_children.push(left_child.to_owned().into());
                        processed_nodes.insert(left_child);
                    }
                    (None, Some(right_matching)) => {
                        result_children.push(
                            merge(
                                left_child,
                                left_child,
                                right_matching.matching_node,
                                base_left_matchings,
                                base_right_matchings,
                                left_right_matchings,
                            )
                            .unwrap(),
                        );
                        processed_nodes.insert(left_child);
                        processed_nodes.insert(right_matching.matching_node);
                    }
                    // Removed in right
                    (Some(matching_base_left), None) => {
                        // Changed in left, conflict!
                        if !matching_base_left.is_perfect_match {
                            result_children.push(MergedCSTNode::Conflict {
                                left: Some(Box::new(left_child.to_owned().into())),
                                right: None,
                            })
                        }
                        processed_nodes.insert(left_child);
                    }
                    (Some(_), Some(right_matching)) => {
                        result_children.push(
                            merge(
                                left_child,
                                left_child,
                                right_matching.matching_node,
                                base_left_matchings,
                                base_right_matchings,
                                left_right_matchings,
                            )
                            .unwrap(),
                        );
                        processed_nodes.insert(left_child);
                        processed_nodes.insert(right_matching.matching_node);
                    }
                }
            }

            for right_child in children_right.iter() {
                if processed_nodes.contains(right_child) {
                    continue;
                }

                let matching_base_right = base_right_matchings.find_matching_for(right_child);
                let matching_left_right = left_right_matchings.find_matching_for(right_child);

                match (matching_base_right, matching_left_right) {
                    // Added only by right
                    (None, None) => {
                        result_children.push(right_child.to_owned().into());
                    }
                    (None, Some(matching_left_right)) => {
                        result_children.push(
                            merge(
                                right_child,
                                matching_left_right.matching_node,
                                right_child,
                                base_left_matchings,
                                base_right_matchings,
                                left_right_matchings,
                            )
                            .unwrap(),
                        );
                    }
                    // Removed in left
                    (Some(matching_base_right), None) => {
                        // Changed in right, conflict!
                        if !matching_base_right.is_perfect_match {
                            result_children.push(MergedCSTNode::Conflict {
                                left: None,
                                right: Some(Box::new(right_child.to_owned().into())),
                            })
                        }
                    }
                    (Some(_), Some(matching_left_right)) => {
                        result_children.push(
                            merge(
                                right_child,
                                matching_left_right.matching_node,
                                right_child,
                                base_left_matchings,
                                base_right_matchings,
                                left_right_matchings,
                            )
                            .unwrap(),
                        );
                    }
                }
            }

            Ok(MergedCSTNode::NonTerminal {
                kind,
                children: result_children,
            })
        }
        (_, _, _) => Err(MergeError::MergingTerminalWithNonTerminal),
    }
}

#[cfg(test)]
mod tests {
    use matching::{unordered_tree_matching, Matchings};
    use model::{
        cst_node::{NonTerminal, Terminal},
        CSTNode, Point,
    };

    use crate::{MergeError, MergedCSTNode};

    use super::unordered_merge;

    fn assert_merge_is_correct_and_idempotent_with_respect_to_parent_side(
        base: &CSTNode,
        parent_a: &CSTNode,
        parent_b: &CSTNode,
        expected_merge: &MergedCSTNode,
    ) -> Result<(), MergeError> {
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
        )?;
        let merged_tree_swap = unordered_merge(
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

    fn assert_merge_output_is(
        base: &CSTNode,
        parent_a: &CSTNode,
        parent_b: &CSTNode,
        expected_merge: &MergedCSTNode,
    ) -> Result<(), MergeError> {
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
        )?;

        assert_eq!(expected_merge, &merged_tree);

        Ok(())
    }

    #[test]
    fn test_merge_node_added_only_by_one_parent() -> Result<(), MergeError> {
        let base = CSTNode::NonTerminal(NonTerminal {
            kind: "interface_body",
            are_children_unordered: true,
            start_position: model::Point { row: 0, column: 0 },
            end_position: model::Point { row: 0, column: 0 },
            children: vec![
                CSTNode::Terminal(Terminal {
                    kind: "{",
                    value: "{",
                    start_position: model::Point { row: 0, column: 1 },
                    end_position: model::Point { row: 0, column: 1 },
                }),
                CSTNode::Terminal(Terminal {
                    kind: "}",
                    value: "}",
                    start_position: model::Point { row: 1, column: 1 },
                    end_position: model::Point { row: 1, column: 1 },
                }),
            ],
        });

        let parent_a = CSTNode::NonTerminal(NonTerminal {
            kind: "interface_body",
            are_children_unordered: true,
            start_position: model::Point { row: 0, column: 0 },
            end_position: model::Point { row: 0, column: 0 },
            children: vec![
                CSTNode::Terminal(Terminal {
                    kind: "{",
                    value: "{",
                    start_position: model::Point { row: 0, column: 1 },
                    end_position: model::Point { row: 0, column: 1 },
                }),
                CSTNode::Terminal(Terminal {
                    kind: "method_declaration",
                    value: "main",
                    start_position: model::Point { row: 1, column: 0 },
                    end_position: model::Point { row: 1, column: 4 },
                }),
                CSTNode::Terminal(Terminal {
                    kind: "}",
                    value: "}",
                    start_position: model::Point { row: 2, column: 1 },
                    end_position: model::Point { row: 2, column: 1 },
                }),
            ],
        });

        let parent_b = CSTNode::NonTerminal(NonTerminal {
            kind: "interface_body",
            are_children_unordered: true,
            start_position: model::Point { row: 0, column: 0 },
            end_position: model::Point { row: 0, column: 0 },
            children: vec![
                CSTNode::Terminal(Terminal {
                    kind: "{",
                    value: "{",
                    start_position: model::Point { row: 0, column: 1 },
                    end_position: model::Point { row: 0, column: 1 },
                }),
                CSTNode::Terminal(Terminal {
                    kind: "}",
                    value: "}",
                    start_position: model::Point { row: 1, column: 1 },
                    end_position: model::Point { row: 1, column: 1 },
                }),
            ],
        });

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

        assert_merge_is_correct_and_idempotent_with_respect_to_parent_side(
            &base, &parent_a, &parent_b, &merge,
        )
    }

    #[test]
    fn test_both_parents_add_the_same_node_and_both_subtrees_are_equal() -> Result<(), MergeError> {
        let base = CSTNode::NonTerminal(NonTerminal {
            kind: "interface_body",
            are_children_unordered: true,
            start_position: model::Point { row: 0, column: 0 },
            end_position: model::Point { row: 0, column: 0 },
            children: vec![
                CSTNode::Terminal(Terminal {
                    kind: "{",
                    value: "{",
                    start_position: model::Point { row: 0, column: 1 },
                    end_position: model::Point { row: 0, column: 1 },
                }),
                CSTNode::Terminal(Terminal {
                    kind: "}",
                    value: "}",
                    start_position: model::Point { row: 1, column: 1 },
                    end_position: model::Point { row: 1, column: 1 },
                }),
            ],
        });

        let parent_a = CSTNode::NonTerminal(NonTerminal {
            kind: "interface_body",
            are_children_unordered: true,
            start_position: model::Point { row: 0, column: 0 },
            end_position: model::Point { row: 0, column: 0 },
            children: vec![
                CSTNode::Terminal(Terminal {
                    kind: "{",
                    value: "{",
                    start_position: model::Point { row: 0, column: 1 },
                    end_position: model::Point { row: 0, column: 1 },
                }),
                CSTNode::NonTerminal(NonTerminal {
                    kind: "method_declaration",
                    are_children_unordered: false,
                    start_position: model::Point { row: 1, column: 0 },
                    end_position: model::Point { row: 1, column: 4 },
                    children: vec![CSTNode::Terminal(Terminal {
                        kind: "identifier",
                        value: "main",
                        start_position: model::Point { row: 0, column: 1 },
                        end_position: model::Point { row: 0, column: 1 },
                    })],
                }),
                CSTNode::Terminal(Terminal {
                    kind: "}",
                    value: "}",
                    start_position: model::Point { row: 2, column: 1 },
                    end_position: model::Point { row: 2, column: 1 },
                }),
            ],
        });

        let parent_b = CSTNode::NonTerminal(NonTerminal {
            kind: "interface_body",
            are_children_unordered: true,
            start_position: model::Point { row: 0, column: 0 },
            end_position: model::Point { row: 0, column: 0 },
            children: vec![
                CSTNode::Terminal(Terminal {
                    kind: "{",
                    value: "{",
                    start_position: model::Point { row: 0, column: 1 },
                    end_position: model::Point { row: 0, column: 1 },
                }),
                CSTNode::NonTerminal(NonTerminal {
                    kind: "method_declaration",
                    are_children_unordered: false,
                    start_position: model::Point { row: 1, column: 0 },
                    end_position: model::Point { row: 1, column: 4 },
                    children: vec![CSTNode::Terminal(Terminal {
                        kind: "identifier",
                        value: "main",
                        start_position: model::Point { row: 0, column: 1 },
                        end_position: model::Point { row: 0, column: 1 },
                    })],
                }),
                CSTNode::Terminal(Terminal {
                    kind: "}",
                    value: "}",
                    start_position: model::Point { row: 2, column: 1 },
                    end_position: model::Point { row: 2, column: 1 },
                }),
            ],
        });

        let expected_merge = MergedCSTNode::NonTerminal {
            kind: "interface_body",
            children: vec![
                MergedCSTNode::Terminal {
                    kind: "{",
                    value: String::from("{"),
                },
                MergedCSTNode::NonTerminal {
                    kind: "method_declaration",
                    children: vec![MergedCSTNode::Terminal {
                        kind: "identifier",
                        value: String::from("main"),
                    }],
                },
                MergedCSTNode::Terminal {
                    kind: "}",
                    value: String::from("}"),
                },
            ],
        };

        assert_merge_is_correct_and_idempotent_with_respect_to_parent_side(
            &base,
            &parent_a,
            &parent_b,
            &expected_merge,
        )
    }

    #[test]
    fn test_merge_one_parent_removes_a_node_while_the_other_keeps_it_unchanged(
    ) -> Result<(), MergeError> {
        let base = CSTNode::NonTerminal(NonTerminal {
            kind: "interface_body",
            are_children_unordered: true,
            start_position: model::Point { row: 0, column: 0 },
            end_position: model::Point { row: 0, column: 0 },
            children: vec![
                CSTNode::Terminal(Terminal {
                    kind: "{",
                    value: "{",
                    start_position: model::Point { row: 0, column: 1 },
                    end_position: model::Point { row: 0, column: 1 },
                }),
                CSTNode::NonTerminal(NonTerminal {
                    kind: "method_declaration",
                    are_children_unordered: false,
                    start_position: model::Point { row: 1, column: 0 },
                    end_position: model::Point { row: 1, column: 4 },
                    children: vec![CSTNode::Terminal(Terminal {
                        kind: "identifier",
                        value: "main",
                        start_position: model::Point { row: 0, column: 1 },
                        end_position: model::Point { row: 0, column: 1 },
                    })],
                }),
                CSTNode::Terminal(Terminal {
                    kind: "}",
                    value: "}",
                    start_position: model::Point { row: 1, column: 1 },
                    end_position: model::Point { row: 1, column: 1 },
                }),
            ],
        });

        let parent_a = CSTNode::NonTerminal(NonTerminal {
            kind: "interface_body",
            are_children_unordered: true,
            start_position: model::Point { row: 0, column: 0 },
            end_position: model::Point { row: 0, column: 0 },
            children: vec![
                CSTNode::Terminal(Terminal {
                    kind: "{",
                    value: "{",
                    start_position: model::Point { row: 0, column: 1 },
                    end_position: model::Point { row: 0, column: 1 },
                }),
                CSTNode::NonTerminal(NonTerminal {
                    kind: "method_declaration",
                    are_children_unordered: false,
                    start_position: model::Point { row: 1, column: 0 },
                    end_position: model::Point { row: 1, column: 4 },
                    children: vec![CSTNode::Terminal(Terminal {
                        kind: "identifier",
                        value: "main",
                        start_position: model::Point { row: 0, column: 1 },
                        end_position: model::Point { row: 0, column: 1 },
                    })],
                }),
                CSTNode::Terminal(Terminal {
                    kind: "}",
                    value: "}",
                    start_position: model::Point { row: 2, column: 1 },
                    end_position: model::Point { row: 2, column: 1 },
                }),
            ],
        });

        let parent_b = CSTNode::NonTerminal(NonTerminal {
            kind: "interface_body",
            are_children_unordered: true,
            start_position: model::Point { row: 0, column: 0 },
            end_position: model::Point { row: 0, column: 0 },
            children: vec![
                CSTNode::Terminal(Terminal {
                    kind: "{",
                    value: "{",
                    start_position: model::Point { row: 0, column: 1 },
                    end_position: model::Point { row: 0, column: 1 },
                }),
                CSTNode::Terminal(Terminal {
                    kind: "}",
                    value: "}",
                    start_position: model::Point { row: 2, column: 1 },
                    end_position: model::Point { row: 2, column: 1 },
                }),
            ],
        });

        let expected_merge = MergedCSTNode::NonTerminal {
            kind: "interface_body",
            children: vec![
                MergedCSTNode::Terminal {
                    kind: "{",
                    value: String::from("{"),
                },
                MergedCSTNode::Terminal {
                    kind: "}",
                    value: String::from("}"),
                },
            ],
        };

        assert_merge_is_correct_and_idempotent_with_respect_to_parent_side(
            &base,
            &parent_a,
            &parent_b,
            &expected_merge,
        )
    }

    #[test]
    fn test_merge_one_parent_removes_a_node_while_the_other_changed_it() -> Result<(), MergeError> {
        let base = CSTNode::NonTerminal(NonTerminal {
            kind: "interface_body",
            are_children_unordered: true,
            start_position: model::Point { row: 0, column: 0 },
            end_position: model::Point { row: 0, column: 0 },
            children: vec![
                CSTNode::Terminal(Terminal {
                    kind: "{",
                    value: "{",
                    start_position: model::Point { row: 0, column: 1 },
                    end_position: model::Point { row: 0, column: 1 },
                }),
                CSTNode::NonTerminal(NonTerminal {
                    kind: "method_declaration",
                    are_children_unordered: false,
                    start_position: model::Point { row: 1, column: 0 },
                    end_position: model::Point { row: 1, column: 4 },
                    children: vec![
                        CSTNode::Terminal(Terminal {
                            kind: "identifier",
                            value: "method",
                            start_position: model::Point { row: 0, column: 1 },
                            end_position: model::Point { row: 0, column: 1 },
                        }),
                        CSTNode::Terminal(Terminal {
                            kind: "kind_a",
                            value: "value_a",
                            start_position: model::Point { row: 0, column: 1 },
                            end_position: model::Point { row: 0, column: 1 },
                        }),
                        CSTNode::Terminal(Terminal {
                            kind: "kind_b",
                            value: "value_b",
                            start_position: model::Point { row: 0, column: 1 },
                            end_position: model::Point { row: 0, column: 1 },
                        }),
                    ],
                }),
                CSTNode::Terminal(Terminal {
                    kind: "}",
                    value: "}",
                    start_position: model::Point { row: 1, column: 1 },
                    end_position: model::Point { row: 1, column: 1 },
                }),
            ],
        });

        let parent_a = CSTNode::NonTerminal(NonTerminal {
            kind: "interface_body",
            are_children_unordered: true,
            start_position: model::Point { row: 0, column: 0 },
            end_position: model::Point { row: 0, column: 0 },
            children: vec![
                CSTNode::Terminal(Terminal {
                    kind: "{",
                    value: "{",
                    start_position: model::Point { row: 0, column: 1 },
                    end_position: model::Point { row: 0, column: 1 },
                }),
                CSTNode::NonTerminal(NonTerminal {
                    kind: "method_declaration",
                    are_children_unordered: false,
                    start_position: model::Point { row: 1, column: 0 },
                    end_position: model::Point { row: 1, column: 4 },
                    children: vec![
                        CSTNode::Terminal(Terminal {
                            kind: "identifier",
                            value: "method",
                            start_position: model::Point { row: 0, column: 1 },
                            end_position: model::Point { row: 0, column: 1 },
                        }),
                        CSTNode::Terminal(Terminal {
                            kind: "kind_a",
                            value: "value_a",
                            start_position: model::Point { row: 0, column: 1 },
                            end_position: model::Point { row: 0, column: 1 },
                        }),
                        CSTNode::Terminal(Terminal {
                            kind: "kind_b",
                            value: "new_value_b",
                            start_position: model::Point { row: 0, column: 1 },
                            end_position: model::Point { row: 0, column: 1 },
                        }),
                    ],
                }),
                CSTNode::Terminal(Terminal {
                    kind: "}",
                    value: "}",
                    start_position: model::Point { row: 2, column: 1 },
                    end_position: model::Point { row: 2, column: 1 },
                }),
            ],
        });

        let parent_b = CSTNode::NonTerminal(NonTerminal {
            kind: "interface_body",
            are_children_unordered: true,
            start_position: model::Point { row: 0, column: 0 },
            end_position: model::Point { row: 0, column: 0 },
            children: vec![
                CSTNode::Terminal(Terminal {
                    kind: "{",
                    value: "{",
                    start_position: model::Point { row: 0, column: 1 },
                    end_position: model::Point { row: 0, column: 1 },
                }),
                CSTNode::Terminal(Terminal {
                    kind: "}",
                    value: "}",
                    start_position: model::Point { row: 2, column: 1 },
                    end_position: model::Point { row: 2, column: 1 },
                }),
            ],
        });

        assert_merge_output_is(
            &base,
            &parent_a,
            &parent_b,
            &MergedCSTNode::NonTerminal {
                kind: "interface_body",
                children: vec![
                    MergedCSTNode::Terminal {
                        kind: "{",
                        value: String::from("{"),
                    },
                    MergedCSTNode::Conflict {
                        left: Some(Box::new(MergedCSTNode::NonTerminal {
                            kind: "method_declaration",
                            children: vec![
                                MergedCSTNode::Terminal {
                                    kind: "identifier",
                                    value: String::from("method"),
                                },
                                MergedCSTNode::Terminal {
                                    kind: "kind_a",
                                    value: String::from("value_a"),
                                },
                                MergedCSTNode::Terminal {
                                    kind: "kind_b",
                                    value: String::from("new_value_b"),
                                },
                            ],
                        })),
                        right: None,
                    },
                    MergedCSTNode::Terminal {
                        kind: "}",
                        value: String::from("}"),
                    },
                ],
            },
        )?;
        assert_merge_output_is(
            &base,
            &parent_b,
            &parent_a,
            &MergedCSTNode::NonTerminal {
                kind: "interface_body",
                children: vec![
                    MergedCSTNode::Terminal {
                        kind: "{",
                        value: String::from("{"),
                    },
                    MergedCSTNode::Conflict {
                        left: None,
                        right: Some(Box::new(MergedCSTNode::NonTerminal {
                            kind: "method_declaration",
                            children: vec![
                                MergedCSTNode::Terminal {
                                    kind: "identifier",
                                    value: String::from("method"),
                                },
                                MergedCSTNode::Terminal {
                                    kind: "kind_a",
                                    value: String::from("value_a"),
                                },
                                MergedCSTNode::Terminal {
                                    kind: "kind_b",
                                    value: String::from("new_value_b"),
                                },
                            ],
                        })),
                    },
                    MergedCSTNode::Terminal {
                        kind: "}",
                        value: String::from("}"),
                    },
                ],
            },
        )
    }

    #[test]
    fn test_can_not_merge_terminal_with_non_terminal() -> Result<(), Box<dyn std::error::Error>> {
        let error = unordered_merge(
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
