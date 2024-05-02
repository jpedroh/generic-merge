use crate::matching_configuration::MatchingConfiguration;
use model::cst_node::NonTerminal;

mod assignment_problem;
mod unique_label;

pub fn calculate_matchings<'a>(
    left: &'a model::CSTNode,
    right: &'a model::CSTNode,
    matching_handlers: &'a matching_handlers::MatchingHandlers<'a>,
    config: &'a MatchingConfiguration,
) -> crate::Matchings<'a> {
    match (left, right) {
        (model::CSTNode::NonTerminal(left_nt), model::CSTNode::NonTerminal(right_nt)) => {
            if all_children_labeled(left_nt, config) && all_children_labeled(right_nt, config) {
                log::debug!("Using unique label matching.");
                unique_label::calculate_matchings(left, right, matching_handlers, config)
            } else {
                log::debug!("Using assignment problem matching.");
                assignment_problem::calculate_matchings(left, right, matching_handlers, config)
            }
        }
        _ => unreachable!("Unordered matching is only supported for non-terminals."),
    }
}

fn all_children_labeled(node: &NonTerminal, config: &MatchingConfiguration) -> bool {
    node.children
        .iter()
        .filter(|child| !config.delimiters.contains(child.kind()))
        .all(|child| config.kinds_with_label.contains(child.kind()))
}
