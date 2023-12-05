use parsing::ParserConfiguration;

#[derive(Debug)]
pub enum ExecutionError {
    ParsingError,
}

#[derive(Debug)]
pub enum ExecutionResult {
    WithConflicts(String),
    WithoutConflicts(String),
}

impl ToString for ExecutionResult {
    fn to_string(&self) -> String {
        match self {
            ExecutionResult::WithConflicts(value) => value.to_owned(),
            ExecutionResult::WithoutConflicts(value) => value.to_owned(),
        }
    }
}

pub fn run_tool_on_merge_scenario(
    language: model::Language,
    base: &str,
    left: &str,
    right: &str,
) -> Result<ExecutionResult, ExecutionError> {
    if base == left {
        return Ok(ExecutionResult::WithoutConflicts(right.to_string()));
    }

    if base == right {
        return Ok(ExecutionResult::WithoutConflicts(left.to_string()));
    }

    let parser_configuration = ParserConfiguration::from(language);

    let base_tree = parsing::parse_string(&base, &parser_configuration)
        .map_err(|_| ExecutionError::ParsingError)?;
    let left_tree = parsing::parse_string(&left, &parser_configuration)
        .map_err(|_| ExecutionError::ParsingError)?;
    let right_tree = parsing::parse_string(&right, &parser_configuration)
        .map_err(|_| ExecutionError::ParsingError)?;

    let matchings_left_base = matching::calculate_matchings(&left_tree, &base_tree);
    let matchings_right_base = matching::calculate_matchings(&right_tree, &base_tree);
    let matchings_left_right = matching::calculate_matchings(&left_tree, &right_tree);

    let result = merge::merge(
        &base_tree,
        &left_tree,
        &right_tree,
        &matchings_left_base,
        &matchings_right_base,
        &matchings_left_right,
    );

    match has_conflict(&result) {
        true => Ok(ExecutionResult::WithConflicts(result.to_string())),
        false => Ok(ExecutionResult::WithoutConflicts(result.to_string())),
    }
}

fn has_conflict(result: &merge::MergedCSTNode) -> bool {
    match result {
        merge::MergedCSTNode::NonTerminal { children, .. } => {
            children.into_iter().any(|child| has_conflict(child))
        }
        merge::MergedCSTNode::Terminal { .. } => false,
        merge::MergedCSTNode::Conflict { .. } => true,
    }
}
