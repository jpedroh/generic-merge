use super::utils::find_child_of_kind;
use model::{cst_node::NonTerminal, CSTNode};

pub fn compute_matching_score_for_import_declaration<'a>(
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
            // Try to find a scoped_identifier on children, and compare them
            let identifier_left =
                find_child_of_kind(children_left, "scoped_identifier").map(|node| node.contents());
            let identifier_right =
                find_child_of_kind(children_right, "scoped_identifier").map(|node| node.contents());

            (identifier_left.is_some() && identifier_left == identifier_right).into()
        }
        (_, _) => 0,
    }
}
