use ast_node::ASTNode;
use changes_tree_node::{ChangesTreeNode, Modification};

mod ast_node;
mod changes_tree_node;

fn get_children_tuples_to_evaluate<'a>(
    base: &'a ASTNode,
    parent: &'a ASTNode,
) -> Vec<(Option<&'a ASTNode>, Option<&'a ASTNode>)> {
    use itertools::Itertools;

    let base_children_in_parent: Vec<(Option<&'a ASTNode>, Option<&'a ASTNode>)> = base
        .children
        .iter()
        .map(|base_node| {
            let node_in_parent = parent.children.iter().find(|parent_node| {
                base_node.kind == parent_node.kind && base_node.identifier == parent_node.identifier
            });
            (Some(base_node), node_in_parent)
        })
        .collect();

    let parent_children_in_base: Vec<(Option<&'a ASTNode>, Option<&'a ASTNode>)> = parent
        .children
        .iter()
        .map(|parent_node| {
            let node_in_base = base.children.iter().find(|base_node| {
                parent_node.kind == base_node.kind && parent_node.identifier == base_node.identifier
            });
            (node_in_base, Some(parent_node))
        })
        .collect();

    parent_children_in_base
        .into_iter()
        .chain(base_children_in_parent.into_iter())
        .unique()
        .collect()
}

fn compute_changes_tree<'a>(
    base: Option<&'a ASTNode>,
    parent: Option<&'a ASTNode>,
) -> ChangesTreeNode<'a> {
    match (base, parent) {
        (None, Some(parent)) => ChangesTreeNode::of_added(parent),
        (Some(base), None) => ChangesTreeNode::of_removed(base),
        (Some(base), Some(parent)) => {
            if base.body == parent.body {
                return ChangesTreeNode::of_unchanged(parent);
            }

            ChangesTreeNode {
                kind: &parent.kind,
                identifier: &parent.identifier,
                body: &parent.body,
                children: get_children_tuples_to_evaluate(base, parent)
                    .into_iter()
                    .map(|(base, parent)| compute_changes_tree(base, parent))
                    .collect(),
                modification: Modification::Changed,
            }
        }
        (_, _) => panic!("Must have at least one node to compare."),
    }
}

#[cfg(test)]
mod tests {
    use crate::{compute_changes_tree, ASTNode, ChangesTreeNode, Modification};

    #[test]
    fn if_base_is_none_and_parent_is_some_the_node_was_added_in_parent() {
        let parent = ASTNode {
            kind: String::from("class_declaration"),
            identifier: String::from("Main"),
            body: String::from("public class Main { public static void main(){} }"),
            children: vec![],
        };
        let result = compute_changes_tree(None, Some(&parent));
        assert_eq!(result.modification, Modification::Added);
    }

    #[test]
    fn if_base_is_some_and_parent_is_none_the_node_was_removed_in_parent() {
        let base = ASTNode {
            kind: String::from("class_declaration"),
            identifier: String::from("Main"),
            body: String::from("public class Main { public static void main(){} }"),
            children: vec![],
        };
        let result = compute_changes_tree(Some(&base), None);
        assert_eq!(result.modification, Modification::Removed);
    }

    #[test]
    fn if_two_nodes_have_the_same_body_they_are_unchanged() {
        let base = ASTNode {
            kind: String::from("class_declaration"),
            identifier: String::from("Main"),
            body: String::from("public class Main { public static void main(){} }"),
            children: vec![],
        };
        let parent = ASTNode {
            kind: String::from("class_declaration"),
            identifier: String::from("Main"),
            body: String::from("public class Main { public static void main(){} }"),
            children: vec![],
        };
        let result = compute_changes_tree(Some(&base), Some(&parent));
        assert_eq!(result.modification, Modification::Unchanged);
    }

    #[test]
    fn if_i_add_a_node_in_parent_children_it_is_marked_as_an_addition() {
        let base = ASTNode {
            kind: String::from("class_declaration"),
            identifier: String::from("Main"),
            body: String::from("public class Main { }"),
            children: vec![],
        };
        let parent = ASTNode {
            kind: String::from("class_declaration"),
            identifier: String::from("Main"),
            body: String::from("public class Main { public static void main(){} }"),
            children: vec![ASTNode {
                kind: String::from("method_declaration"),
                identifier: String::from("main"),
                body: String::from("public static void main(){}"),
                children: vec![],
            }],
        };

        let result = compute_changes_tree(Some(&base), Some(&parent));

        assert_eq!(
            result,
            ChangesTreeNode {
                kind: "class_declaration",
                identifier: "Main",
                body: "public class Main { public static void main(){} }",
                modification: Modification::Changed,
                children: vec![ChangesTreeNode {
                    kind: "method_declaration",
                    identifier: "main",
                    body: "public static void main(){}",
                    modification: Modification::Added,
                    children: vec![]
                }]
            }
        );
    }

    #[test]
    fn if_i_remove_a_node_in_parent_children_it_is_marked_as_a_removal() {
        let base = ASTNode {
            kind: String::from("class_declaration"),
            identifier: String::from("Main"),
            body: String::from("public class Main { public static void main(){} }"),
            children: vec![ASTNode {
                kind: String::from("method_declaration"),
                identifier: String::from("main"),
                body: String::from("public static void main(){}"),
                children: vec![],
            }],
        };
        let parent = ASTNode {
            kind: String::from("class_declaration"),
            identifier: String::from("Main"),
            body: String::from("public class Main { }"),
            children: vec![],
        };

        let result = compute_changes_tree(Some(&base), Some(&parent));

        assert_eq!(
            result,
            ChangesTreeNode {
                kind: "class_declaration",
                identifier: "Main",
                body: "public class Main { }",
                modification: Modification::Changed,
                children: vec![ChangesTreeNode {
                    kind: "method_declaration",
                    identifier: "main",
                    body: "public static void main(){}",
                    modification: Modification::Removed,
                    children: vec![]
                }]
            }
        );
    }

    #[test]
    fn if_i_change_a_node_in_parent_children_it_is_marked_as_a_change() {
        let base = ASTNode {
            kind: String::from("class_declaration"),
            identifier: String::from("Main"),
            body: String::from("public class Main { public static void main(){int x = 10;} }"),
            children: vec![ASTNode {
                kind: String::from("method_declaration"),
                identifier: String::from("main"),
                body: String::from("public static void main(){int x = 10;}"),
                children: vec![],
            }],
        };
        let parent = ASTNode {
            kind: String::from("class_declaration"),
            identifier: String::from("Main"),
            body: String::from("public class Main { public static void main(){int x = 20;} }"),
            children: vec![ASTNode {
                kind: String::from("method_declaration"),
                identifier: String::from("main"),
                body: String::from("public static void main(){int x = 20;}"),
                children: vec![],
            }],
        };

        let result = compute_changes_tree(Some(&base), Some(&parent));

        assert_eq!(
            result,
            ChangesTreeNode {
                kind: "class_declaration",
                identifier: "Main",
                body: "public class Main { public static void main(){int x = 20;} }",
                modification: Modification::Changed,
                children: vec![ChangesTreeNode {
                    kind: "method_declaration",
                    identifier: "main",
                    body: "public static void main(){int x = 20;}",
                    modification: Modification::Changed,
                    children: vec![]
                }]
            }
        );
    }

    #[test]
    fn adding_a_node_and_keeping_other_unchanged() {
        let base = ASTNode {
            kind: String::from("class_declaration"),
            identifier: String::from("Main"),
            body: String::from("public class Main { public static void main(){int x = 10;} }"),
            children: vec![ASTNode {
                kind: String::from("method_declaration"),
                identifier: String::from("main"),
                body: String::from("public static void main(){int x = 10;}"),
                children: vec![],
            }],
        };
        let parent = ASTNode {
            kind: String::from("class_declaration"),
            identifier: String::from("Main"),
            body: String::from("public class Main { public static void main(){int x = 10;} public static void test(){} }"),
            children: vec![ASTNode {
                kind: String::from("method_declaration"),
                identifier: String::from("main"),
                body: String::from("public static void main(){int x = 10;}"),
                children: vec![],
            },
            ASTNode {
                kind: String::from("method_declaration"),
                identifier: String::from("test"),
                body: String::from("public static void test(){}"),
                children: vec![],
            }],
        };

        let result = compute_changes_tree(Some(&base), Some(&parent));

        assert_eq!(
            result,
            ChangesTreeNode {
                kind: "class_declaration",
                identifier: "Main",
                body: "public class Main { public static void main(){int x = 10;} public static void test(){} }",
                modification: Modification::Changed,
                children: vec![
                    ChangesTreeNode {
                        kind: "method_declaration",
                        identifier: "main",
                        body: "public static void main(){int x = 10;}",
                        modification: Modification::Unchanged,
                        children: vec![]
                    },
                    ChangesTreeNode {
                        kind: "method_declaration",
                        identifier: "test",
                        body: "public static void test(){}",
                        modification: Modification::Added,
                        children: vec![]
                    }
                ]
            }
        );
    }

    #[test]
    fn test_adding_a_node_in_an_inner_node() {
        let base = ASTNode {
            kind: String::from("class_declaration"),
            identifier: String::from("Main"),
            body: String::from(
                "public class Main { public static class Inner{} public static void main(){} }",
            ),
            children: vec![
                ASTNode {
                    kind: String::from("class_declaration"),
                    identifier: String::from("Inner"),
                    body: String::from("public static class Inner{}"),
                    children: vec![],
                },
                ASTNode {
                    kind: String::from("method_declaration"),
                    identifier: String::from("main"),
                    body: String::from("public static void main(){}"),
                    children: vec![],
                },
            ],
        };

        let parent = ASTNode {
            kind: String::from("class_declaration"),
            identifier: String::from("Main"),
            body: String::from(
                "public class Main { public static class Inner{public static void main(){}} public static void main(){} }",
            ),
            children: vec![
                ASTNode {
                    kind: String::from("class_declaration"),
                    identifier: String::from("Inner"),
                    body: String::from("public static class Inner{public static void main(){}}"),
                    children: vec![
                        ASTNode {
                            kind: String::from("method_declaration"),
                            identifier: String::from("main"),
                            body: String::from("public static void main(){}"),
                            children: vec![],
                        },
                    ],
                },
                ASTNode {
                    kind: String::from("method_declaration"),
                    identifier: String::from("main"),
                    body: String::from("public static void main(){}"),
                    children: vec![],
                },
            ],
        };

        let result = compute_changes_tree(Some(&base), Some(&parent));

        assert_eq!(
            result,
            ChangesTreeNode {
                kind: "class_declaration",
                identifier: "Main",
                body: "public class Main { public static class Inner{public static void main(){}} public static void main(){} }",
                modification: Modification::Changed,
                children: vec![
                    ChangesTreeNode {
                        kind: "class_declaration",
                        identifier: "Inner",
                        body: "public static class Inner{public static void main(){}}",
                        modification: Modification::Changed,
                        children: vec![
                            ChangesTreeNode {
                                kind: "method_declaration",
                                identifier: "main",
                                body: "public static void main(){}",
                                modification: Modification::Added,
                                children: vec![]
                            },
                        ]
                    },
                    ChangesTreeNode {
                        kind: "method_declaration",
                        identifier: "main",
                        body: "public static void main(){}",
                        modification: Modification::Unchanged,
                        children: vec![]
                    }
                ]
            }
        );
    }
}
