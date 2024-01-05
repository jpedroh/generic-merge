use super::utils::find_identifier;
use model::{cst_node::NonTerminal, CSTNode};

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
            let identifier_left = find_identifier(children_left).map(|node| node.value);
            let identifier_right = find_identifier(children_right).map(|node| node.value);

            (identifier_left.is_some() && identifier_left == identifier_right).into()
        }
        (_, _) => 0,
    }
}
