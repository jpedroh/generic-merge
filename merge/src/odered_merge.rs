use matching::Matchings;
use model::CSTNode;

pub fn ordered_merge<'a>(
    base: &'a CSTNode<'a>,
    left: &'a CSTNode<'a>,
    right: &'a CSTNode<'a>,
    base_left_matchings: &'a Matchings<'a>,
    base_right_matchings: &'a Matchings<'a>,
    left_right_matchings: &'a Matchings<'a>,
) -> CSTNode<'a> {
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
                base.to_owned()
            // Changed in both
            } else if value_left != value_base && value_right != value_base {
                match diffy::merge(&value_base, &value_left, &value_right) {
                    Ok(value) => CSTNode::Terminal { kind, value: value },
                    Err(value) => CSTNode::Terminal { kind, value: value },
                }
            // Only left changed
            } else if value_left != value_base {
                left.to_owned()
            // Only right changed
            } else {
                right.to_owned()
            }
        }
        (
            CSTNode::NonTerminal { kind, .. },
            CSTNode::NonTerminal {
                children: children_left,
                ..
            },
            CSTNode::NonTerminal {
                children: children_right,
                ..
            },
        ) => {
            let mut result_children = vec![];

            let mut children_left_it = children_left.iter();
            let mut children_right_it = children_right.iter();

            let mut cur_left = children_left_it.next();
            let mut cur_right = children_right_it.next();

            while cur_left.is_some() && cur_right.is_some() {
                let matching_base_left = base_left_matchings.find_matching_for(cur_left.unwrap());
                let matching_base_right =
                    base_right_matchings.find_matching_for(cur_right.unwrap());
                let bidirectional_matching_left_right = left_right_matchings.get_matching_entry(
                    cur_left.unwrap().to_owned(),
                    cur_right.unwrap().to_owned(),
                );
                let left_matching_in_right =
                    left_right_matchings.find_matching_for(cur_left.unwrap());
                let right_matching_in_left =
                    left_right_matchings.find_matching_for(cur_right.unwrap());

                match (
                    bidirectional_matching_left_right,
                    left_matching_in_right,
                    matching_base_left,
                    right_matching_in_left,
                    matching_base_right,
                ) {
                    (Some(_), Some(_), Some(_), Some(_), Some(_)) => {
                        result_children.push(ordered_merge(
                            &cur_left.unwrap(),
                            &cur_left.unwrap(),
                            &cur_right.unwrap(),
                            &base_left_matchings,
                            &base_right_matchings,
                            &left_right_matchings,
                        ));

                        cur_left = children_left_it.next();
                        cur_right = children_right_it.next();
                    }
                    (Some(_), Some(_), None, Some(_), None) => {
                        result_children.push(ordered_merge(
                            &cur_left.unwrap(),
                            &cur_left.unwrap(),
                            &cur_right.unwrap(),
                            &base_left_matchings,
                            &base_right_matchings,
                            &left_right_matchings,
                        ));

                        cur_left = children_left_it.next();
                        cur_right = children_right_it.next();
                    }
                    (None, Some(_), Some(_), None, Some(matching_base_right)) => {
                        if !matching_base_right.is_perfect_match {
                            result_children.push(CSTNode::Conflict {
                                left: Box::new(None),
                                right: Box::new(Some(cur_right.unwrap().to_owned())),
                            });
                        }

                        cur_right = children_right_it.next();
                    }
                    (None, Some(_), Some(_), None, None) => {
                        result_children.push(cur_right.unwrap().to_owned());

                        cur_right = children_right_it.next();
                    }
                    (None, Some(_), None, None, Some(matching_base_right)) => {
                        if !matching_base_right.is_perfect_match {
                            result_children.push(CSTNode::Conflict { left: None.into(), right: Some(cur_right.unwrap().to_owned()).into()})
                        }
                        cur_right = children_right_it.next();
                    }
                    (None, Some(_), None, None, None) => {}
                    (None, None, Some(matching_base_left), Some(_), Some(_)) => {
                        if !matching_base_left.is_perfect_match {
                            result_children.push(CSTNode::Conflict {
                                left: Box::new(Some(cur_left.unwrap().to_owned())),
                                right: Box::new(None),
                            });
                        }

                        cur_left = children_left_it.next();
                    }
                    (None, None, Some(matching_base_left), Some(_), None) => {
                        if !matching_base_left.is_perfect_match {
                            result_children.push(CSTNode::Conflict { left: Some(cur_left.unwrap().to_owned()).into(), right: None.into() })
                        }
                        cur_left = children_left_it.next();
                    }
                    (None, None, Some(_), None, Some(_)) => {}
                    (None, None, Some(matching_base_left), None, None) => {
                        result_children.push(cur_right.unwrap().to_owned());

                        if !matching_base_left.is_perfect_match {
                            result_children.push(CSTNode::Conflict {
                                left: Box::new(Some(cur_left.unwrap().to_owned())),
                                right: Box::new(None),
                            })
                        }

                        cur_left = children_left_it.next();
                        cur_right = children_right_it.next();
                    }
                    (None, None, None, Some(_), Some(_)) => {
                        result_children.push(cur_left.unwrap().to_owned());

                        cur_left = children_left_it.next();
                    }
                    (None, None, None, Some(_), None) => {}
                    (None, None, None, None, Some(matching_base_right)) => {
                        result_children.push(cur_left.unwrap().to_owned());

                        if !matching_base_right.is_perfect_match {
                            result_children.push(CSTNode::Conflict {
                                left: Box::new(None),
                                right: Box::new(Some(cur_right.unwrap().to_owned())),
                            })
                        }

                        cur_left = children_left_it.next();
                        cur_right = children_right_it.next();
                    }
                    (None, None, None, None, None) => {
                        result_children.push(CSTNode::Conflict {
                            left: Box::new(Some(cur_left.unwrap().to_owned())),
                            right: Box::new(Some(cur_right.unwrap().to_owned())),
                        });

                        cur_left = children_left_it.next();
                        cur_right = children_right_it.next();
                    }
                    (_, _, _, _, _) => panic!("[INVARIANT BROKEN]: Ordered merge found a matching configuration that should not be achieved")
                }
            }

            while cur_left.is_some() {
                result_children.push(cur_left.unwrap().to_owned());
                cur_left = children_left_it.next();
            }

            while cur_right.is_some() {
                result_children.push(cur_right.unwrap().to_owned());
                cur_right = children_right_it.next();
            }

            CSTNode::NonTerminal {
                kind,
                children: result_children,
            }
        }
        (_, _, _) => panic!("Can not merge Terminal with Non-Terminal"),
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use matching::{ordered_tree_matching, Matchings};
    use model::CSTNode;

    use super::ordered_merge;

    fn assert_merge_is_correct_and_idempotent_with_respect_to_parent_side(
        base: CSTNode,
        parent_a: CSTNode,
        parent_b: CSTNode,
        expected_merge: CSTNode,
    ) {
        let matchings_base_parent_a = ordered_tree_matching(&base, &parent_a);
        let matchings_base_parent_b = ordered_tree_matching(&base, &parent_b);
        let matchings_parents = ordered_tree_matching(&parent_a, &parent_b);

        let merged_tree = ordered_merge(
            &base,
            &parent_a,
            &parent_b,
            &matchings_base_parent_a,
            &matchings_base_parent_b,
            &matchings_parents,
        );
        let merged_tree_swap = ordered_merge(
            &base,
            &parent_b,
            &parent_a,
            &matchings_base_parent_b,
            &matchings_base_parent_a,
            &matchings_parents,
        );

        assert_eq!(expected_merge, merged_tree);
        assert_eq!(expected_merge, merged_tree_swap)
    }

    fn assert_merge_output_is(
        base: &CSTNode,
        parent_a: &CSTNode,
        parent_b: &CSTNode,
        expected_merge: &CSTNode,
    ) {
        let matchings_base_parent_a = ordered_tree_matching(&base, &parent_a);
        let matchings_base_parent_b = ordered_tree_matching(&base, &parent_b);
        let matchings_parents = ordered_tree_matching(&parent_a, &parent_b);

        let merged_tree = ordered_merge(
            &base,
            &parent_a,
            &parent_b,
            &matchings_base_parent_a,
            &matchings_base_parent_b,
            &matchings_parents,
        );

        assert_eq!(expected_merge, &merged_tree);
    }

    #[test]
    fn if_i_am_merging_three_unchanged_nodes_it_is_a_success() {
        let node = CSTNode::Terminal {
            kind: "kind".into(),
            value: "value".into(),
        };

        assert_merge_is_correct_and_idempotent_with_respect_to_parent_side(
            node.clone(),
            node.clone(),
            node.clone(),
            node,
        )
    }

    #[test]
    fn returns_success_if_there_are_changes_in_both_parents_and_they_are_not_conflicting() {
        let base = CSTNode::Terminal {
            kind: "kind".into(),
            value: "\nvalue\n".into(),
        };
        let left = CSTNode::Terminal {
            kind: "kind".into(),
            value: "left\nvalue\n".into(),
        };
        let right = CSTNode::Terminal {
            kind: "kind".into(),
            value: "\nvalue\nright".into(),
        };

        assert_merge_is_correct_and_idempotent_with_respect_to_parent_side(
            base,
            left,
            right,
            CSTNode::Terminal {
                kind: "kind".into(),
                value: "left\nvalue\nright".into(),
            },
        )
    }

    #[test]
    fn returns_conflict_if_there_are_changes_in_both_parents_and_they_are_conflicting() {
        let base = CSTNode::Terminal {
            kind: "kind".into(),
            value: "value".into(),
        };
        let left = CSTNode::Terminal {
            kind: "kind".into(),
            value: "left_value".into(),
        };
        let right = CSTNode::Terminal {
            kind: "kind".into(),
            value: "right_value".into(),
        };

        assert_eq!(
            ordered_merge(&base, &left, &right, &Matchings::empty(), &Matchings::empty(),
            &Matchings::empty()),
           CSTNode::Terminal {
                kind: "kind".into(),
                value: "<<<<<<< ours\nleft_value||||||| original\nvalue=======\nright_value>>>>>>> theirs\n".into()
            }
        )
    }

    #[test]
    fn if_there_is_a_change_only_in_one_parent_it_returns_the_changes_from_this_parent() {
        let base_and_left = CSTNode::Terminal {
            kind: "kind".into(),
            value: "value".into(),
        };
        let changed_parent = CSTNode::Terminal {
            kind: "kind".into(),
            value: "value_right".into(),
        };

        assert_merge_is_correct_and_idempotent_with_respect_to_parent_side(
            base_and_left.clone(),
            base_and_left.clone(),
            changed_parent.clone(),
            changed_parent,
        )
    }

    #[test]
    #[should_panic(expected = "Can not merge Terminal with Non-Terminal")]
    fn test_can_not_merge_terminal_with_non_terminal() {
        ordered_merge(
            &CSTNode::Terminal {
                kind: "kind".into(),
                value: "value".into(),
            },
            &CSTNode::Terminal {
                kind: "kind".into(),
                value: "value".into(),
            },
            &CSTNode::NonTerminal {
                kind: "kind".into(),
                children: vec![],
            },
            &Matchings::empty(),
            &Matchings::empty(),
            &Matchings::empty(),
        );
    }

    #[test]
    fn it_merges_non_terminals_if_there_are_non_changes() {
        let tree = CSTNode::NonTerminal {
            kind: "kind".into(),
            children: vec![
                CSTNode::Terminal {
                    kind: "kind_a".into(),
                    value: "value_a".into(),
                },
                CSTNode::Terminal {
                    kind: "kind_b".into(),
                    value: "value_b".into(),
                },
            ],
        };

        assert_merge_is_correct_and_idempotent_with_respect_to_parent_side(
            tree.clone(),
            tree.clone(),
            tree.clone(),
            tree,
        )
    }

    #[test]
    fn it_merges_non_terminals_if_both_left_and_right_add_the_same_things() {
        let base = CSTNode::NonTerminal {
            kind: "kind".into(),
            children: vec![],
        };
        let parent = CSTNode::NonTerminal {
            kind: "kind".into(),
            children: vec![
                CSTNode::Terminal {
                    kind: "kind_a".into(),
                    value: "value_a".into(),
                },
                CSTNode::Terminal {
                    kind: "kind_b".into(),
                    value: "value_b".into(),
                },
            ],
        };

        assert_merge_is_correct_and_idempotent_with_respect_to_parent_side(
            base,
            parent.clone(),
            parent.clone(),
            parent,
        )
    }

    #[test]
    fn it_merges_non_terminals_if_only_one_parent_adds_a_node_in_an_initially_empty_children_list()
    {
        let base = CSTNode::NonTerminal {
            kind: "kind".into(),
            children: vec![],
        };

        let initially_empty_parent = CSTNode::NonTerminal {
            kind: "kind".into(),
            children: vec![],
        };

        let parent_that_added = CSTNode::NonTerminal {
            kind: "kind".into(),
            children: vec![CSTNode::Terminal {
                kind: "kind_a".into(),
                value: "value_a".into(),
            }],
        };

        let merge = CSTNode::NonTerminal {
            kind: "kind".into(),
            children: vec![CSTNode::Terminal {
                kind: "kind_a".into(),
                value: "value_a".into(),
            }],
        };

        assert_merge_is_correct_and_idempotent_with_respect_to_parent_side(
            base,
            initially_empty_parent,
            parent_that_added,
            merge,
        )
    }

    #[test]
    fn it_merges_non_terminals_if_only_one_parent_adds_a_node_in_non_empty_children_list() {
        let base = CSTNode::NonTerminal {
            kind: "kind".into(),
            children: vec![CSTNode::Terminal {
                kind: "kind_a".into(),
                value: "value_a".into(),
            }],
        };

        let unchanged_parent = CSTNode::NonTerminal {
            kind: "kind".into(),
            children: vec![CSTNode::Terminal {
                kind: "kind_a".into(),
                value: "value_a".into(),
            }],
        };

        let parent_that_added = CSTNode::NonTerminal {
            kind: "kind".into(),
            children: vec![
                CSTNode::Terminal {
                    kind: "kind_a".into(),
                    value: "value_a".into(),
                },
                CSTNode::Terminal {
                    kind: "kind_b".into(),
                    value: "value_b".into(),
                },
            ],
        };

        let merge = CSTNode::NonTerminal {
            kind: "kind".into(),
            children: vec![
                CSTNode::Terminal {
                    kind: "kind_a".into(),
                    value: "value_a".into(),
                },
                CSTNode::Terminal {
                    kind: "kind_b".into(),
                    value: "value_b".into(),
                },
            ],
        };

        assert_merge_is_correct_and_idempotent_with_respect_to_parent_side(
            base,
            unchanged_parent,
            parent_that_added,
            merge,
        )
    }

    #[test]
    fn it_merges_when_one_parent_adds_a_node_and_removes_one_that_was_not_edited_in_the_other() {
        let base = CSTNode::NonTerminal {
            kind: "kind".into(),
            children: vec![CSTNode::Terminal {
                kind: "kind_a".into(),
                value: "value_a".into(),
            }],
        };

        let changed_parent = CSTNode::NonTerminal {
            kind: "kind".into(),
            children: vec![CSTNode::Terminal {
                kind: "kind_b".into(),
                value: "value_b".into(),
            }],
        };

        let unchanged_parent = CSTNode::NonTerminal {
            kind: "kind".into(),
            children: vec![CSTNode::Terminal {
                kind: "kind_a".into(),
                value: "value_a".into(),
            }],
        };

        let expected_merge = CSTNode::NonTerminal {
            kind: "kind".into(),
            children: vec![CSTNode::Terminal {
                kind: "kind_b".into(),
                value: "value_b".into(),
            }],
        };

        assert_merge_is_correct_and_idempotent_with_respect_to_parent_side(
            base,
            changed_parent,
            unchanged_parent,
            expected_merge,
        )
    }

    #[test]
    fn it_merges_when_one_parent_adds_a_node_and_removes_from_another_that_was_changed() {
        let base = CSTNode::NonTerminal {
            kind: "kind".into(),
            children: vec![CSTNode::NonTerminal {
                kind: "subtree".into(),
                children: vec![CSTNode::Terminal {
                    kind: "kind_a".into(),
                    value: "value_a".into(),
                }],
            }],
        };

        let parent_a = CSTNode::NonTerminal {
            kind: "kind".into(),
            children: vec![CSTNode::NonTerminal {
                kind: "another_subtree".into(),
                children: vec![CSTNode::Terminal {
                    kind: "kind_b".into(),
                    value: "value_b".into(),
                }],
            }],
        };

        let parent_b = CSTNode::NonTerminal {
            kind: "kind".into(),
            children: vec![CSTNode::NonTerminal {
                kind: "subtree".into(),
                children: vec![CSTNode::Terminal {
                    kind: "kind_c".into(),
                    value: "value_c".into(),
                }],
            }],
        };

        let matchings_base_parent_a = ordered_tree_matching(&base, &parent_a);
        let matchings_base_parent_b = ordered_tree_matching(&base, &parent_b);
        let matchings_parents = ordered_tree_matching(&parent_a, &parent_b);

        let merged_tree = ordered_merge(
            &base,
            &parent_a,
            &parent_b,
            &matchings_base_parent_a,
            &matchings_base_parent_b,
            &matchings_parents,
        );
        let merged_tree_swap = ordered_merge(
            &base,
            &parent_b,
            &parent_a,
            &matchings_base_parent_b,
            &matchings_base_parent_a,
            &matchings_parents,
        );

        assert_eq!(
            CSTNode::NonTerminal {
                kind: "kind".into(),
                children: vec![
                    CSTNode::NonTerminal {
                        kind: "another_subtree".into(),
                        children: vec![CSTNode::Terminal {
                            kind: "kind_b".into(),
                            value: "value_b".into(),
                        }],
                    },
                    CSTNode::Conflict {
                        left: Box::new(None),
                        right: Box::new(Some(CSTNode::NonTerminal {
                            kind: "subtree".into(),
                            children: vec![CSTNode::Terminal {
                                kind: "kind_c".into(),
                                value: "value_c".into(),
                            }],
                        })),
                    },
                ],
            },
            merged_tree
        );

        assert_eq!(
            CSTNode::NonTerminal {
                kind: "kind".into(),
                children: vec![
                    CSTNode::NonTerminal {
                        kind: "another_subtree".into(),
                        children: vec![CSTNode::Terminal {
                            kind: "kind_b".into(),
                            value: "value_b".into(),
                        }],
                    },
                    CSTNode::Conflict {
                        left: Box::new(Some(CSTNode::NonTerminal {
                            kind: "subtree".into(),
                            children: vec![CSTNode::Terminal {
                                kind: "kind_c".into(),
                                value: "value_c".into(),
                            }],
                        })),
                        right: Box::new(None),
                    },
                ],
            },
            merged_tree_swap
        );
    }

    #[test]
    fn if_both_parents_add_different_nodes_then_we_have_a_conflict() {
        let base = CSTNode::NonTerminal {
            kind: "kind".into(),
            children: vec![],
        };

        let left = CSTNode::NonTerminal {
            kind: "kind".into(),
            children: vec![CSTNode::Terminal {
                kind: "kind_a".into(),
                value: "value_a".into(),
            }],
        };

        let right = CSTNode::NonTerminal {
            kind: "kind".into(),
            children: vec![CSTNode::Terminal {
                kind: "kind_b".into(),
                value: "value_b".into(),
            }],
        };

        assert_merge_output_is(
            &base,
            &left,
            &right,
            &CSTNode::NonTerminal {
                kind: "kind".into(),
                children: vec![CSTNode::Conflict {
                    left: Box::new(Some(CSTNode::Terminal {
                        kind: "kind_a".into(),
                        value: "value_a".into(),
                    })),
                    right: Box::new(Some(CSTNode::Terminal {
                        kind: "kind_b".into(),
                        value: "value_b".into(),
                    })),
                }],
            },
        )
    }

    #[test]
    fn it_merges_when_one_parent_removes_a_node_that_was_not_changed_in_another_parent() {
        let base = CSTNode::NonTerminal {
            kind: "kind".into(),
            children: vec![
                CSTNode::Terminal {
                    kind: "kind_a".into(),
                    value: "value_a".into(),
                },
                CSTNode::Terminal {
                    kind: "kind_b".into(),
                    value: "value_b".into(),
                },
            ],
        };

        let left = CSTNode::NonTerminal {
            kind: "kind".into(),
            children: vec![
                CSTNode::Terminal {
                    kind: "kind_a".into(),
                    value: "value_a".into(),
                },
                CSTNode::Terminal {
                    kind: "kind_b".into(),
                    value: "value_b".into(),
                },
            ],
        };

        let right = CSTNode::NonTerminal {
            kind: "kind".into(),
            children: vec![CSTNode::Terminal {
                kind: "kind_b".into(),
                value: "value_b".into(),
            }],
        };

        assert_merge_output_is(
            &base,
            &left,
            &right,
            &CSTNode::NonTerminal {
                kind: "kind".into(),
                children: vec![CSTNode::Terminal {
                    kind: "kind_b".into(),
                    value: "value_b".into(),
                }],
            },
        )
    }

    #[test]
    fn it_detects_a_conflit_when_one_parent_removes_a_node_that_was_changed_in_another_parent() {
        let base = CSTNode::NonTerminal {
            kind: "kind".into(),
            children: vec![
                CSTNode::NonTerminal {
                    kind: "subtree".into(),
                    children: vec![CSTNode::Terminal {
                        kind: "kind_b".into(),
                        value: "value_b".into(),
                    }],
                },
                CSTNode::Terminal {
                    kind: "kind_a".into(),
                    value: "value_a".into(),
                },
            ],
        };

        let left = CSTNode::NonTerminal {
            kind: "kind".into(),
            children: vec![
                CSTNode::NonTerminal {
                    kind: "subtree".into(),
                    children: vec![CSTNode::Terminal {
                        kind: "kind_c".into(),
                        value: "value_c".into(),
                    }],
                },
                CSTNode::Terminal {
                    kind: "kind_a".into(),
                    value: "value_a".into(),
                },
            ],
        };

        let right = CSTNode::NonTerminal {
            kind: "kind".into(),
            children: vec![CSTNode::Terminal {
                kind: "kind_a".into(),
                value: "value_a".into(),
            }],
        };

        assert_merge_output_is(
            &base,
            &left,
            &right,
            &CSTNode::NonTerminal {
                kind: "kind".into(),
                children: vec![
                    CSTNode::Conflict {
                        left: Some(CSTNode::NonTerminal {
                            kind: "subtree".into(),
                            children: vec![CSTNode::Terminal {
                                kind: "kind_c".into(),
                                value: "value_c".into(),
                            }],
                        })
                        .into(),
                        right: None.into(),
                    },
                    CSTNode::Terminal {
                        kind: "kind_a".into(),
                        value: "value_a".into(),
                    },
                ],
            },
        );

        assert_merge_output_is(
            &base,
            &right,
            &left,
            &CSTNode::NonTerminal {
                kind: "kind".into(),
                children: vec![
                    CSTNode::Conflict {
                        left: None.into(),
                        right: Some(CSTNode::NonTerminal {
                            kind: "subtree".into(),
                            children: vec![CSTNode::Terminal {
                                kind: "kind_c".into(),
                                value: "value_c".into(),
                            }],
                        })
                        .into(),
                    },
                    CSTNode::Terminal {
                        kind: "kind_a".into(),
                        value: "value_a".into(),
                    },
                ],
            },
        )
    }

    #[test]
    fn it_merges_when_a_parent_adds_a_node() {
        let base = CSTNode::NonTerminal {
            kind: "kind".into(),
            children: vec![
                CSTNode::Terminal {
                    kind: "kind_a".into(),
                    value: "value_a".into(),
                },
                CSTNode::Terminal {
                    kind: "kind_c".into(),
                    value: "value_c".into(),
                },
            ],
        };

        let unchanged_parent = CSTNode::NonTerminal {
            kind: "kind".into(),
            children: vec![
                CSTNode::Terminal {
                    kind: "kind_a".into(),
                    value: "value_a".into(),
                },
                CSTNode::Terminal {
                    kind: "kind_c".into(),
                    value: "value_c".into(),
                },
            ],
        };

        let changed_parent = CSTNode::NonTerminal {
            kind: "kind".into(),
            children: vec![
                CSTNode::Terminal {
                    kind: "kind_a".into(),
                    value: "value_a".into(),
                },
                CSTNode::Terminal {
                    kind: "kind_b".into(),
                    value: "value_b".into(),
                },
                CSTNode::Terminal {
                    kind: "kind_c".into(),
                    value: "value_c".into(),
                },
            ],
        };

        let expected_merge = CSTNode::NonTerminal {
            kind: "kind".into(),
            children: vec![
                CSTNode::Terminal {
                    kind: "kind_a".into(),
                    value: "value_a".into(),
                },
                CSTNode::Terminal {
                    kind: "kind_b".into(),
                    value: "value_b".into(),
                },
                CSTNode::Terminal {
                    kind: "kind_c".into(),
                    value: "value_c".into(),
                },
            ],
        };

        assert_merge_is_correct_and_idempotent_with_respect_to_parent_side(
            base,
            unchanged_parent,
            changed_parent,
            expected_merge,
        )
    }

    #[test]
    fn it_merges_when_one_parent_removes_and_add_a_node() {
        let base = CSTNode::NonTerminal {
            kind: "kind".into(),
            children: vec![CSTNode::Terminal {
                kind: "kind_b".into(),
                value: "value_b".into(),
            }],
        };

        let parent_a = CSTNode::NonTerminal {
            kind: "kind".into(),
            children: vec![CSTNode::Terminal {
                kind: "kind_a",
                value: "value_a".into(),
            }],
        };

        let parent_b = CSTNode::NonTerminal {
            kind: "kind".into(),
            children: vec![
                CSTNode::Terminal {
                    kind: "kind_b".into(),
                    value: "value_b".into(),
                },
                CSTNode::Terminal {
                    kind: "kind_a",
                    value: "value_a".into(),
                },
            ],
        };

        let expected_merge = CSTNode::NonTerminal {
            kind: "kind".into(),
            children: vec![CSTNode::Terminal {
                kind: "kind_a",
                value: "value_a".into(),
            }],
        };

        assert_merge_is_correct_and_idempotent_with_respect_to_parent_side(
            base,
            parent_a,
            parent_b,
            expected_merge,
        )
    }

    #[test]
    fn it_conflicts_when_one_parent_removes_and_add_a_node() {
        let base = CSTNode::NonTerminal {
            kind: "kind",
            children: vec![CSTNode::NonTerminal {
                kind: "subtree",
                children: vec![CSTNode::Terminal {
                    kind: "kind_b",
                    value: "value_b".into(),
                }],
            }],
        };

        let parent_a = CSTNode::NonTerminal {
            kind: "kind".into(),
            children: vec![CSTNode::Terminal {
                kind: "kind_a",
                value: "value_a".into(),
            }],
        };

        let parent_b = CSTNode::NonTerminal {
            kind: "kind".into(),
            children: vec![
                CSTNode::NonTerminal {
                    kind: "subtree",
                    children: vec![CSTNode::Terminal {
                        kind: "kind_b",
                        value: "value_c".into(),
                    }],
                },
                CSTNode::Terminal {
                    kind: "kind_a",
                    value: "value_a".into(),
                },
            ],
        };

        assert_merge_output_is(
            &base,
            &parent_a,
            &parent_b,
            &CSTNode::NonTerminal {
                kind: "kind".into(),
                children: vec![
                    CSTNode::Conflict {
                        left: None.into(),
                        right: Some(CSTNode::NonTerminal {
                            kind: "subtree",
                            children: vec![CSTNode::Terminal {
                                kind: "kind_b",
                                value: "value_c".into(),
                            }],
                        })
                        .into(),
                    },
                    CSTNode::Terminal {
                        kind: "kind_a",
                        value: "value_a".into(),
                    },
                ],
            },
        );
        assert_merge_output_is(
            &base,
            &parent_b,
            &parent_a,
            &CSTNode::NonTerminal {
                kind: "kind".into(),
                children: vec![
                    CSTNode::Conflict {
                        left: Some(CSTNode::NonTerminal {
                            kind: "subtree",
                            children: vec![CSTNode::Terminal {
                                kind: "kind_b",
                                value: "value_c".into(),
                            }],
                        })
                        .into(),
                        right: None.into(),
                    },
                    CSTNode::Terminal {
                        kind: "kind_a",
                        value: "value_a".into(),
                    },
                ],
            },
        );
    }
}
