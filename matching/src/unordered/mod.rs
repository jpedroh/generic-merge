mod assignment_problem;
mod unique_label;

pub fn calculate_matchings<'a>(
    left: &'a model::CSTNode,
    right: &'a model::CSTNode,
    matching_handlers: &'a matching_handlers::MatchingHandlers<'a>,
) -> crate::Matchings<'a> {
    match (left, right) {
        (model::CSTNode::NonTerminal(_), model::CSTNode::NonTerminal(_)) => {
            assignment_problem::calculate_matchings(left, right, matching_handlers)
        }
        _ => unreachable!("Unordered matching is only supported for non-terminals."),
    }
}
