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
                let has_matching_base_left = base_left_matchings
                    .find_matching_for(cur_left.unwrap())
                    .is_some();
                let has_matching_base_right = base_right_matchings
                    .find_matching_for(cur_right.unwrap())
                    .is_some();
                let matching_left_right = left_right_matchings.get_matching_entry(
                    cur_left.unwrap().to_owned(),
                    cur_right.unwrap().to_owned(),
                );

                // The nodes are unchanged
                if has_matching_base_left
                    && has_matching_base_right
                    && matching_left_right.is_some()
                    && matching_left_right.unwrap().is_perfect_match
                {
                    result_children.push(merge(
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

                // This is the case where left and right both add the same nodes
                if !has_matching_base_left
                    && !has_matching_base_right
                    && matching_left_right.is_some()
                    && matching_left_right.unwrap().is_perfect_match
                {
                    result_children.push(merge(
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

                // Addition in left
                if has_matching_base_left && !has_matching_base_right {
                    result_children.push(cur_left.unwrap().to_owned());
                    cur_left = children_left_it.next();
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
                kind: kind.to_string(),
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

    fn assert_merge_is_correct_and_idempotent_with_respect_to_parent_side(
        base: CSTNode,
        parent_a: CSTNode,
        parent_b: CSTNode,
        expected_merge: CSTNode,
    ) {
        let matchings_base_parent_a = ordered_tree_matching(&base, &parent_a);
        let matchings_base_parent_b = ordered_tree_matching(&base, &parent_b);
        let matchings_parents = ordered_tree_matching(&parent_a, &parent_b);

        let merged_tree = merge(
            &base,
            &parent_a,
            &parent_b,
            &matchings_base_parent_a,
            &matchings_base_parent_b,
            &matchings_parents,
        );
        let merged_tree_swap = merge(
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
            merge(&base, &left, &right, &Matchings::empty(), &Matchings::empty(),
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
}
