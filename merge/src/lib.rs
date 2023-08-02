use std::borrow::BorrowMut;

use matching::Matchings;
use model::CSTNode;

pub fn merge(
    base: &CSTNode,
    left: &CSTNode,
    right: &CSTNode,
    base_left_matchings: &Matchings,
    base_right_matchings: &Matchings,
    left_right_matchings: &Matchings,
) -> CSTNode {
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
                    Ok(value) => CSTNode::Terminal {
                        kind: kind.to_owned(),
                        value,
                    },
                    Err(value) => CSTNode::Terminal {
                        kind: kind.to_owned(),
                        value,
                    },
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
            CSTNode::NonTerminal {
                kind,
                children: base_children,
            },
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

            // Mutually modified
            let mut mutually_modified_children: Vec<CSTNode> = base_children
                .iter()
                .map(|node| {
                    return (
                        node,
                        base_left_matchings.find_matching_for(node),
                        base_right_matchings.find_matching_for(node),
                    );
                })
                .filter(|(_, left_match, right_match)| {
                    return left_match.is_some() && right_match.is_some();
                })
                .map(|(base, left_match, right_match)| {
                    return merge(
                        &base,
                        left_match.unwrap().matching_node,
                        right_match.unwrap().matching_node,
                        &base_left_matchings,
                        &base_right_matchings,
                        &left_right_matchings,
                    );
                })
                .collect();

            result_children.append(&mut mutually_modified_children);

            // Nodes added only in left
            result_children.append(
                children_left
                    .iter()
                    .filter(|left_child| {
                        return base_left_matchings.find_matching_for(left_child).is_none();
                        // && left_right_matchings.find_matching_for(left_child).is_none();
                    })
                    .map(|node| node.to_owned())
                    .collect::<Vec<CSTNode>>()
                    .borrow_mut(),
            );

            // Nodes added only in right
            result_children.append(
                children_right
                    .iter()
                    .filter(|right_child| {
                        return base_right_matchings
                            .find_matching_for(right_child)
                            .is_none();
                        // && left_right_matchings
                        //     .find_matching_for(right_child)
                        //     .is_none();
                    })
                    .map(|node| node.to_owned())
                    .collect::<Vec<CSTNode>>()
                    .borrow_mut(),
            );

            CSTNode::NonTerminal {
                kind: kind.to_owned(),
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

    use crate::merge;

    #[test]
    fn if_i_am_merging_three_unchanged_nodes_it_is_a_success() {
        let node = CSTNode::Terminal {
            kind: "kind".into(),
            value: "value".into(),
        };
        assert_eq!(
            merge(
                &node,
                &node,
                &node,
                &Matchings::empty(),
                &Matchings::empty(),
                &Matchings::empty()
            ),
            node
        )
    }

    #[test]
    fn returns_success_if_there_are_changes_in_both_left_and_right_and_they_are_not_conflicting() {
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

        assert_eq!(
            merge(
                &base,
                &left,
                &right,
                &Matchings::empty(),
                &Matchings::empty(),
                &Matchings::empty()
            ),
            CSTNode::Terminal {
                kind: "kind".into(),
                value: "left\nvalue\nright".into()
            }
        )
    }

    #[test]
    fn returns_conflict_if_there_are_changes_in_both_left_and_right_and_they_are_conflicting() {
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
            merge(&base, &left, &right, &Matchings::empty(), &Matchings::empty(),
            &Matchings::empty()),
           CSTNode::Terminal {
                kind: "kind".into(),
                value: "<<<<<<< ours\nleft_value||||||| original\nvalue=======\nright_value>>>>>>> theirs\n".into()
            }
        )
    }

    #[test]
    fn if_there_is_a_change_only_in_left_it_returns_the_changes_from_left() {
        let base_and_right = CSTNode::Terminal {
            kind: "kind".into(),
            value: "value".into(),
        };
        let left = CSTNode::Terminal {
            kind: "kind".into(),
            value: "value_left".into(),
        };
        assert_eq!(
            merge(
                &base_and_right,
                &left,
                &base_and_right,
                &Matchings::empty(),
                &Matchings::empty(),
                &Matchings::empty()
            ),
            left
        )
    }

    #[test]
    fn if_there_is_a_change_only_in_right_it_returns_the_changes_from_right() {
        let base_and_left = CSTNode::Terminal {
            kind: "kind".into(),
            value: "value".into(),
        };
        let right = CSTNode::Terminal {
            kind: "kind".into(),
            value: "value_right".into(),
        };
        assert_eq!(
            merge(
                &base_and_left,
                &base_and_left,
                &right,
                &Matchings::empty(),
                &Matchings::empty(),
                &Matchings::empty()
            ),
            right,
        )
    }

    #[test]
    #[should_panic(expected = "Can not merge Terminal with Non-Terminal")]
    fn test_can_not_merge_terminal_with_non_terminal() {
        merge(
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
    fn merge_puts_added_nodes_in_left_only() {
        let left = CSTNode::NonTerminal {
            kind: "kind".into(),
            children: vec![
                CSTNode::Terminal {
                    kind: "another_kind".into(),
                    value: "another_value".into(),
                },
                CSTNode::Terminal {
                    kind: "kind_left".into(),
                    value: "value_left".into(),
                },
            ],
        };
        let base_and_right = CSTNode::NonTerminal {
            kind: "kind".into(),
            children: vec![CSTNode::Terminal {
                kind: "another_kind".into(),
                value: "another_value".into(),
            }],
        };

        let matchings_left_base = ordered_tree_matching(&left, &base_and_right);

        assert_eq!(
            CSTNode::NonTerminal {
                kind: "kind".into(),
                children: vec![
                    CSTNode::Terminal {
                        kind: "kind_left".into(),
                        value: "value_left".into(),
                    },
                    CSTNode::Terminal {
                        kind: "another_kind".into(),
                        value: "another_value".into(),
                    }
                ],
            },
            merge(
                &base_and_right,
                &left,
                &base_and_right,
                &matchings_left_base,
                &Matchings::empty(),
                &Matchings::empty()
            )
        );
    }

    #[test]
    fn merge_removes_nodes_deleted_in_left_only() {
        let base_and_right = CSTNode::NonTerminal {
            kind: "kind".into(),
            children: vec![
                CSTNode::Terminal {
                    kind: "kind".into(),
                    value: "value".into(),
                },
                CSTNode::Terminal {
                    kind: "deleted_in_left".into(),
                    value: "deleted_in_left".into(),
                },
            ],
        };
        let left = CSTNode::NonTerminal {
            kind: "kind".into(),
            children: vec![CSTNode::Terminal {
                kind: "kind".into(),
                value: "value".into(),
            }],
        };

        let matchings_left_base = ordered_tree_matching(&left, &base_and_right);
        let matchings_right_base = ordered_tree_matching(&base_and_right, &base_and_right);
        let matchings_left_right = ordered_tree_matching(&base_and_right, &base_and_right);

        assert_eq!(
            CSTNode::NonTerminal {
                kind: "kind".into(),
                children: vec![CSTNode::Terminal {
                    kind: "kind".into(),
                    value: "value".into(),
                }],
            },
            merge(
                &base_and_right,
                &left,
                &base_and_right,
                &matchings_left_base,
                &matchings_right_base,
                &matchings_left_right
            )
        );
    }

    #[test]
    fn merge_independent_nodes_added_in_left_and_right() {
        let base = CSTNode::NonTerminal {
            kind: "kind".into(),
            children: vec![CSTNode::Terminal {
                kind: "kind".into(),
                value: "value".into(),
            }],
        };
        let left = CSTNode::NonTerminal {
            kind: "kind".into(),
            children: vec![
                CSTNode::Terminal {
                    kind: "kind".into(),
                    value: "value".into(),
                },
                CSTNode::Terminal {
                    kind: "added_in_left".into(),
                    value: "added_in_left".into(),
                },
            ],
        };
        let right = CSTNode::NonTerminal {
            kind: "kind".into(),
            children: vec![
                CSTNode::Terminal {
                    kind: "kind".into(),
                    value: "value".into(),
                },
                CSTNode::Terminal {
                    kind: "added_in_right".into(),
                    value: "added_in_right".into(),
                },
            ],
        };

        let matchings_left_base = ordered_tree_matching(&left, &base);
        let matchings_right_base = ordered_tree_matching(&right, &base);
        let matchings_left_right = ordered_tree_matching(&left, &right);

        assert_eq!(
            CSTNode::NonTerminal {
                kind: "kind".into(),
                children: vec![
                    CSTNode::Terminal {
                        kind: "kind".into(),
                        value: "value".into(),
                    },
                    CSTNode::Terminal {
                        kind: "added_in_left".into(),
                        value: "added_in_left".into(),
                    },
                    CSTNode::Terminal {
                        kind: "added_in_right".into(),
                        value: "added_in_right".into(),
                    }
                ],
            },
            merge(
                &base,
                &left,
                &right,
                &matchings_left_base,
                &matchings_right_base,
                &matchings_left_right
            )
        );
    }

    #[test]
    fn merge_deep_nodes_additions() {
        let base = CSTNode::NonTerminal {
            kind: "kind".into(),
            children: vec![CSTNode::NonTerminal {
                kind: "kind".into(),
                children: vec![],
            }],
        };
        let left = CSTNode::NonTerminal {
            kind: "kind".into(),
            children: vec![CSTNode::NonTerminal {
                kind: "kind".into(),
                children: vec![CSTNode::Terminal {
                    kind: "added_in_left".into(),
                    value: "added_in_left".into(),
                }],
            }],
        };
        let right = CSTNode::NonTerminal {
            kind: "kind".into(),
            children: vec![CSTNode::NonTerminal {
                kind: "kind".into(),
                children: vec![CSTNode::Terminal {
                    kind: "added_in_right".into(),
                    value: "added_in_right".into(),
                }],
            }],
        };

        let matchings_left_base = ordered_tree_matching(&left, &base);
        let matchings_right_base = ordered_tree_matching(&right, &base);
        let matchings_left_right = ordered_tree_matching(&left, &right);

        assert_eq!(
            CSTNode::NonTerminal {
                kind: "kind".into(),
                children: vec![CSTNode::NonTerminal {
                    kind: "kind".into(),
                    children: vec![
                        CSTNode::Terminal {
                            kind: "added_in_left".into(),
                            value: "added_in_left".into(),
                        },
                        CSTNode::Terminal {
                            kind: "added_in_right".into(),
                            value: "added_in_right".into(),
                        }
                    ]
                }]
            },
            merge(
                &base,
                &left,
                &right,
                &matchings_left_base,
                &matchings_right_base,
                &matchings_left_right
            )
        );
    }
}
