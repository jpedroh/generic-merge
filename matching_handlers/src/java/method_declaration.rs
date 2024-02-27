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
            let type_of_left_arguments = find_child_of_kind(children_left, "formal_parameters")
                .map(|node| extract_argument_types_from_formal_parameters(node));
            let type_of_right_arguments = find_child_of_kind(children_right, "formal_parameters")
                .map(|node| extract_argument_types_from_formal_parameters(node));

            let identifiers_are_equal =
                identifier_left.is_some() && identifier_left == identifier_right;
            let arguments_are_equal = type_of_left_arguments.is_some()
                && type_of_left_arguments == type_of_right_arguments;

            (identifiers_are_equal && arguments_are_equal).into()
        }
        (_, _) => 0,
    }
}

fn extract_argument_types_from_formal_parameters(node: &CSTNode) -> Vec<String> {
    match node {
        CSTNode::Terminal(_) => vec![],
        CSTNode::NonTerminal(non_terminal) => non_terminal
            .children
            .iter()
            .filter(|inner_node| inner_node.kind() == "formal_parameter")
            .filter_map(|inner_node| match inner_node {
                CSTNode::Terminal(_) => None,
                CSTNode::NonTerminal(non_terminal) => {
                    non_terminal.children.first().map(|v| v.contents())
                }
            })
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use model::{
        cst_node::{NonTerminal, Terminal},
        CSTNode,
    };

    use crate::java::method_declaration::compute_matching_score_for_method_declaration;

    #[test]
    fn it_returns_one_if_methods_have_the_same_identifier() {
        let left =
            make_method_declaration_node("an_identifier", make_method_parameter("String", "name"));
        let right =
            make_method_declaration_node("an_identifier", make_method_parameter("String", "name"));
        let matching_score = compute_matching_score_for_method_declaration(&left, &right);
        assert_eq!(1, matching_score);
    }

    #[test]
    fn it_returns_zero_if_methods_have_different_identifiers() {
        let left = make_method_declaration_node(
            "an_identifier_a",
            make_method_parameter("String", "name"),
        );
        let right = make_method_declaration_node(
            "an_identifier_b",
            make_method_parameter("String", "name"),
        );
        let matching_score = compute_matching_score_for_method_declaration(&left, &right);
        assert_eq!(0, matching_score);
    }

    #[test]
    fn it_returns_one_if_methods_have_equal_identifiers_and_equal_parameters_list() {
        let left =
            make_method_declaration_node("an_identifier", make_method_parameter("String", "name"));
        let right = make_method_declaration_node(
            "an_identifier",
            make_method_parameter("String", "another_name"),
        );
        let matching_score = compute_matching_score_for_method_declaration(&left, &right);
        assert_eq!(1, matching_score);
    }

    #[test]
    fn it_returns_zero_if_methods_have_equal_identifiers_but_different_parameters_list() {
        let parameter_left = make_method_parameter("String", "name");
        let parameter_right = make_method_parameter("int", "another_name");

        let left = make_method_declaration_node("an_identifier", parameter_left);
        let right = make_method_declaration_node("an_identifier", parameter_right);
        let matching_score = compute_matching_score_for_method_declaration(&left, &right);
        assert_eq!(0, matching_score);
    }

    fn make_method_declaration_node<'a>(
        identifier: &'a str,
        parameter: CSTNode<'a>,
    ) -> CSTNode<'a> {
        CSTNode::NonTerminal(NonTerminal {
            kind: "method_declaration",
            children: vec![
                CSTNode::NonTerminal(NonTerminal {
                    kind: "modifiers",
                    children: vec![CSTNode::Terminal(Terminal {
                        kind: "public",
                        value: "public",
                        ..Default::default()
                    })],
                    ..Default::default()
                }),
                CSTNode::Terminal(Terminal {
                    kind: "void_type",
                    value: "void",
                    ..Default::default()
                }),
                CSTNode::Terminal(Terminal {
                    kind: "identifier",
                    value: identifier,
                    ..Default::default()
                }),
                CSTNode::NonTerminal(NonTerminal {
                    kind: "formal_parameters",
                    children: vec![
                        CSTNode::Terminal(Terminal {
                            kind: "(",
                            value: "(",
                            ..Default::default()
                        }),
                        parameter,
                        CSTNode::Terminal(Terminal {
                            kind: ")",
                            value: ")",
                            ..Default::default()
                        }),
                    ],
                    ..Default::default()
                }),
                CSTNode::NonTerminal(NonTerminal {
                    kind: "block",
                    children: vec![
                        CSTNode::Terminal(Terminal {
                            kind: "{",
                            value: "{",
                            ..Default::default()
                        }),
                        CSTNode::Terminal(Terminal {
                            kind: "}",
                            value: "}",
                            ..Default::default()
                        }),
                    ],
                    ..Default::default()
                }),
            ],
            ..Default::default()
        })
    }

    fn make_method_parameter<'a>(a_type: &'a str, identifier: &'a str) -> CSTNode<'a> {
        CSTNode::NonTerminal(NonTerminal {
            kind: "formal_parameter",
            children: vec![
                CSTNode::Terminal(Terminal {
                    kind: "type_identifier",
                    value: a_type,
                    ..Default::default()
                }),
                CSTNode::Terminal(Terminal {
                    kind: "identifier",
                    value: identifier,
                    ..Default::default()
                }),
            ],
            ..Default::default()
        })
    }
}
