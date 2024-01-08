use super::utils::find_identifier;
use model::{cst_node::NonTerminal, CSTNode};

fn find_variable_declarator<'a>(node_children: &'a [CSTNode<'a>]) -> Option<&'a NonTerminal<'a>> {
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
            let identifier_left = find_variable_declarator(children_left)
                .and_then(|node| find_identifier(&node.children))
                .map(|node| node.value);
            let identifier_right = find_variable_declarator(children_right)
                .and_then(|node| find_identifier(&node.children))
                .map(|node| node.value);

            (identifier_left.is_some() && identifier_left == identifier_right).into()
        }
        (_, _) => 0,
    }
}

#[cfg(test)]
mod tests {
    use model::{
        cst_node::{NonTerminal, Terminal},
        CSTNode,
    };

    use crate::java::field_declaration::compute_matching_score_for_field_declaration;

    #[test]
    fn it_returns_one_if_nodes_have_the_same_identifier() {
        let left = make_field_declarator_node_with_identifier("an_identifier");
        let right = make_field_declarator_node_with_identifier("an_identifier");
        let matching_score = compute_matching_score_for_field_declaration(&left, &right);
        assert_eq!(1, matching_score);
    }

    #[test]
    fn it_returns_zero_if_nodes_have_different_identifiers() {
        let left = make_field_declarator_node_with_identifier("an_identifier_a");
        let right = make_field_declarator_node_with_identifier("an_identifier_b");
        let matching_score = compute_matching_score_for_field_declaration(&left, &right);
        assert_eq!(0, matching_score);
    }

    fn make_field_declarator_node_with_identifier(identifier: &str) -> CSTNode {
        return CSTNode::NonTerminal(NonTerminal {
            kind: "field_declaration",
            children: vec![
                CSTNode::NonTerminal(NonTerminal {
                    kind: "modifiers",
                    children: vec![CSTNode::Terminal(Terminal {
                        kind: "private",
                        value: "private",
                        ..Default::default()
                    })],
                    ..Default::default()
                }),
                CSTNode::Terminal(Terminal {
                    kind: "type_identifier",
                    value: "String",
                    ..Default::default()
                }),
                CSTNode::NonTerminal(NonTerminal {
                    kind: "variable_declarator",
                    children: vec![CSTNode::Terminal(Terminal {
                        kind: "identifier",
                        value: identifier,
                        ..Default::default()
                    })],
                    ..Default::default()
                }),
                CSTNode::Terminal(Terminal {
                    kind: ";",
                    value: ";",
                    ..Default::default()
                }),
            ],
            ..Default::default()
        });
    }
}
