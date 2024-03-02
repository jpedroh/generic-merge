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

    fn make_import_of_resource(resource: &str) -> model::CSTNode {
        model::CSTNode::NonTerminal(model::cst_node::NonTerminal {
            kind: "import_declaration",
            children: vec![
                model::CSTNode::NonTerminal(model::cst_node::NonTerminal {
                    kind: "scoped_identifier",
                    children: resource.split(".").map(|part| {
                        model::CSTNode::Terminal(model::cst_node::Terminal {
                            kind: "identifier",
                            value: part,
                            ..Default::default()
                        })
                    }).collect(),
                    ..Default::default()
                })
            ],
            ..Default::default()
        })
    }
}
