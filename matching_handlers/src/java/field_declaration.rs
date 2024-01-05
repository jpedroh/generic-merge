use super::utils::find_identifier;
use model::{
    cst_node::{NonTerminal, Terminal},
    CSTNode,
};

fn find_variable_declarator<'a>(
    node_children: &'a Vec<CSTNode<'a>>,
) -> Option<&'a NonTerminal<'a>> {
    node_children
        .iter()
        .find(|node| node.kind() == "variable_declarator")
        .and_then(|node| match node {
            CSTNode::NonTerminal(non_terminal) => Some(non_terminal),
            CSTNode::Terminal(_) => None,
        })
}

pub fn compute_matching_score_for_field_declaration<'a>(
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
            let identifier_left = find_variable_declarator(&children_left)
                .and_then(|node| find_identifier(&node.children))
                .map(|node| node.value);
            let identifier_right = find_variable_declarator(&children_right)
                .and_then(|node| find_identifier(&node.children))
                .map(|node| node.value);

            (identifier_left.is_some() && identifier_left == identifier_right).into()
        }
        (_, _) => 0,
    }
}
