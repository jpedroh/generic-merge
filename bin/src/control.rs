use std::{error::Error, fmt};

use parsing::ParserConfiguration;

#[derive(Debug)]
pub enum ExecutionError {
    ParsingError(&'static str),
    MergeError(merge::MergeError),
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExecutionError::ParsingError(error) => write!(f, "Parsing error occurred: {}", error),
            ExecutionError::MergeError(error) => write!(f, "Merge error occurred: {}", error),
        }
    }
}

impl Error for ExecutionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
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

    let base_tree =
        parsing::parse_string(base, &parser_configuration).map_err(ExecutionError::ParsingError)?;
    let left_tree =
        parsing::parse_string(left, &parser_configuration).map_err(ExecutionError::ParsingError)?;
    let right_tree = parsing::parse_string(right, &parser_configuration)
        .map_err(ExecutionError::ParsingError)?;

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
    )
    .map_err(ExecutionError::MergeError)?;

    match result.has_conflict() {
        true => Ok(ExecutionResult::WithConflicts(result.to_string())),
        false => Ok(ExecutionResult::WithoutConflicts(result.to_string())),
    }
}
