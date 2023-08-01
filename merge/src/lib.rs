use model::CSTNode;

#[derive(Debug, Eq, PartialEq)]
pub enum MergeResultNode {
    Conflict(CSTNode),
    Success(CSTNode),
}

pub fn merge(base: &CSTNode, left: &CSTNode, right: &CSTNode) -> MergeResultNode {
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
                MergeResultNode::Success(base.to_owned())
            // Changed in both
            } else if value_left != value_base && value_right != value_base {
                match diffy::merge(&value_base, &value_left, &value_right) {
                    Ok(value) => MergeResultNode::Success(CSTNode::Terminal {
                        kind: kind.to_owned(),
                        value,
                    }),
                    Err(value) => MergeResultNode::Conflict(CSTNode::Terminal {
                        kind: kind.to_owned(),
                        value,
                    }),
                }
            // Only left changed
            } else if value_left != value_base {
                MergeResultNode::Success(left.to_owned())
            // Only right changed
            } else {
                MergeResultNode::Success(right.to_owned())
            }
        }
        (CSTNode::NonTerminal { .. }, CSTNode::NonTerminal { .. }, CSTNode::NonTerminal { .. }) => {
            todo!()
        }
        (_, _, _) => panic!("Can not merge Terminal with Non-Terminal"),
    }
}

#[cfg(test)]
mod tests {
    use model::CSTNode;

    use crate::{merge, MergeResultNode};

    #[test]
    fn if_i_am_merging_three_unchanged_nodes_it_is_a_success() {
        let node = CSTNode::Terminal {
            kind: "kind".into(),
            value: "value".into(),
        };
        assert_eq!(merge(&node, &node, &node), MergeResultNode::Success(node))
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
            merge(&base, &left, &right),
            MergeResultNode::Success(CSTNode::Terminal {
                kind: "kind".into(),
                value: "left\nvalue\nright".into()
            })
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
            merge(&base, &left, &right),
            MergeResultNode::Conflict(CSTNode::Terminal {
                kind: "kind".into(),
                value: "<<<<<<< ours\nleft_value||||||| original\nvalue=======\nright_value>>>>>>> theirs\n".into()
            })
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
            merge(&base_and_right, &left, &base_and_right),
            MergeResultNode::Success(left)
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
            merge(&base_and_left, &base_and_left, &right),
            MergeResultNode::Success(right),
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
        );
    }
}
