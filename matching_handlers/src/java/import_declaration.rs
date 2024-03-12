use model::CSTNode;

pub fn compute_matching_score_for_import_declaration<'a>(
    left: &'a CSTNode<'a>,
    right: &'a CSTNode<'a>,
) -> usize {
    (left.contents() == right.contents()).into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn imports_of_the_same_resource_matches_with_one() {
        let result = super::compute_matching_score_for_import_declaration(
            &make_import_of_resource("java.util.array"),
            &make_import_of_resource("java.util.array"),
        );
        assert_eq!(1, result);
    }

    #[test]
    fn imports_of_different_resources_matches_with_zero() {
        let result = super::compute_matching_score_for_import_declaration(
            &make_import_of_resource("java.util.array"),
            &make_import_of_resource("java.util.list"),
        );
        assert_eq!(0, result);
    }

    #[test]
    fn imports_with_asterisks_do_match_if_they_are_equal() {
        let node = model::CSTNode::NonTerminal(model::cst_node::NonTerminal {
            kind: "import_declaration",
            children: vec![
                model::CSTNode::Terminal(model::cst_node::Terminal {
                    kind: "identifier",
                    value: "AST",
                    ..Default::default()
                }),
                model::CSTNode::Terminal(model::cst_node::Terminal {
                    kind: ".",
                    value: ".",
                    ..Default::default()
                }),
                model::CSTNode::Terminal(model::cst_node::Terminal {
                    kind: "asterisk",
                    value: "*",
                    ..Default::default()
                }),
            ],
            ..Default::default()
        });

        let result = super::compute_matching_score_for_import_declaration(&node, &node);

        assert_eq!(1, result);
    }

    fn make_import_of_resource(resource: &str) -> model::CSTNode {
        model::CSTNode::NonTerminal(model::cst_node::NonTerminal {
            kind: "import_declaration",
            children: vec![model::CSTNode::NonTerminal(model::cst_node::NonTerminal {
                kind: "scoped_identifier",
                children: resource
                    .split(".")
                    .map(|part| {
                        model::CSTNode::Terminal(model::cst_node::Terminal {
                            kind: "identifier",
                            value: part,
                            ..Default::default()
                        })
                    })
                    .collect(),
                ..Default::default()
            })],
            ..Default::default()
        })
    }
}
