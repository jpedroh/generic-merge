use super::utils::find_child_of_kind;
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
            let identifier_left =
                find_child_of_kind(children_left, "identifier").map(|node| node.contents());
            let identifier_right =
                find_child_of_kind(children_right, "identifier").map(|node| node.contents());

            // We also need to take method arguments into account because of overloading
            let arguments_left =
                find_child_of_kind(children_left, "formal_parameters").map(|node| node.contents());
            let arguments_right =
                find_child_of_kind(children_right, "formal_parameters").map(|node| node.contents());

            let identifiers_are_equal =
                identifier_left.is_some() && identifier_left == identifier_right;
            let arguments_are_equal = arguments_left.is_some() && arguments_left == arguments_right;

            (identifiers_are_equal && arguments_are_equal).into()
        }
        (_, _) => 0,
    }
}
