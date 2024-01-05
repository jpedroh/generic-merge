use model::{
    cst_node::{NonTerminal, Terminal},
    CSTNode,
};

pub fn compute_matching_score_for_field_declaration<'a>(
    left: &'a CSTNode,
    right: &'a CSTNode,
) -> usize {
    match (left, right) {
        (
            CSTNode::Terminal(Terminal {
                kind: kind_left,
                value: value_left,
                ..
            }),
            CSTNode::Terminal(Terminal {
                kind: kind_right,
                value: value_right,
                ..
            }),
        ) => (kind_left == kind_right && value_left == value_right).into(),
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
            // Try to find an identifier on children, and compare them
            let variable_declarator_left = children_left
                .iter()
                .find(|node| match node {
                    CSTNode::NonTerminal(NonTerminal { kind, .. }) => {
                        kind == &"variable_declarator"
                    }
                    _ => false,
                })
                .map(|node| match node {
                    CSTNode::NonTerminal(non_terminal) => non_terminal,
                    CSTNode::Terminal(_) => panic!("Invalid configuration reached"),
                });

            let variable_declarator_right = children_right
                .iter()
                .find(|node| match node {
                    CSTNode::NonTerminal(NonTerminal { kind, .. }) => {
                        kind == &"variable_declarator"
                    }
                    _ => false,
                })
                .map(|node| match node {
                    CSTNode::NonTerminal(non_terminal) => non_terminal,
                    CSTNode::Terminal(_) => panic!("Invalid configuration reached"),
                });

            // Try to find an identifier on children, and compare them
            let identifier_left = variable_declarator_left
                .unwrap()
                .children
                .iter()
                .find(|node| match node {
                    CSTNode::Terminal(Terminal { kind, .. }) => kind == &"identifier",
                    _ => false,
                });

            let identifier_right =
                variable_declarator_right
                    .unwrap()
                    .children
                    .iter()
                    .find(|node| match node {
                        CSTNode::Terminal(Terminal { kind, .. }) => kind == &"identifier",
                        _ => false,
                    });

            match (identifier_left, identifier_right) {
                (Some(identifier_left), Some(identifier_right)) => {
                    match (identifier_left, identifier_right) {
                        (
                            CSTNode::Terminal(Terminal {
                                value: value_left, ..
                            }),
                            CSTNode::Terminal(Terminal {
                                value: value_right, ..
                            }),
                        ) if value_left == value_right => 1,
                        (_, _) => 0,
                    }
                }
                (_, _) => 0,
            }
        }
        (_, _) => 0,
    }
}
