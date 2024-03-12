use super::utils::find_child_of_kind;
use model::{cst_node::NonTerminal, CSTNode};

pub fn compute_matching_score_for_class_like_declaration<'a>(
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
            let identifier_left =
                find_child_of_kind(children_left, "identifier").map(|node| node.contents());
            let identifier_right =
                find_child_of_kind(children_right, "identifier").map(|node| node.contents());

            (identifier_left.is_some() && identifier_left == identifier_right).into()
        }
        (_, _) => 0,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn classes_with_the_same_name_match_with_score_one() {
        let result = super::compute_matching_score_for_class_like_declaration(
            &make_class_like_declaration("ABC"),
            &make_class_like_declaration("ABC"),
        );
        assert_eq!(1, result);
    }

    #[test]
    fn classes_of_different_names_do_not_match() {
        let result = super::compute_matching_score_for_class_like_declaration(
            &make_class_like_declaration("ABC"),
            &make_class_like_declaration("DEF"),
        );
        assert_eq!(0, result);
    }

    fn make_class_like_declaration(identifier: &str) -> model::CSTNode {
        model::CSTNode::NonTerminal(model::cst_node::NonTerminal {
            kind: "class_declaration",
            children: vec![model::CSTNode::Terminal(model::cst_node::Terminal {
                kind: "identifier",
                value: identifier,
                ..Default::default()
            })],
            ..Default::default()
        })
    }
}
