use model::{
    cst_node::{NonTerminal, Terminal},
    CSTNode,
};

pub fn compute_matching_score_for_method_declaration<'a>(
    left: &'a CSTNode,
    right: &'a CSTNode,
) -> usize {
    match (left, right) {
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
            let identifier_left = children_left
                .iter()
                .find(|node| node.kind() == "identifier")
                .and_then(|node| match node {
                    CSTNode::Terminal(terminal) => Some(terminal),
                    CSTNode::NonTerminal(_) => None,
                });

            let identifier_right = children_right
                .iter()
                .find(|node| node.kind() == "identifier")
                .and_then(|node| match node {
                    CSTNode::Terminal(terminal) => Some(terminal),
                    CSTNode::NonTerminal(_) => None,
                });

            match (identifier_left, identifier_right) {
                (
                    Some(Terminal {
                        value: value_left, ..
                    }),
                    Some(Terminal {
                        value: value_right, ..
                    }),
                ) => {
                    return (value_left == value_right).into();
                }
                (_, _) => 0,
            }
        }
        (_, _) => 0,
    }
}
