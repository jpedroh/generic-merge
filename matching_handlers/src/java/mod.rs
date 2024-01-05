mod field_declaration;
mod method_declaration;
mod utils;

use crate::MatchingHandlers;

use self::{
    field_declaration::compute_matching_score_for_field_declaration,
    method_declaration::compute_matching_score_for_method_declaration,
};

pub fn get_default_java_matching_handlers<'a>() -> MatchingHandlers<'a> {
    let mut matching_handlers: MatchingHandlers<'a> = MatchingHandlers::new();
    matching_handlers.register(
        "field_declaration",
        compute_matching_score_for_field_declaration,
    );
    matching_handlers.register(
        "method_declaration",
        compute_matching_score_for_method_declaration,
    );
    matching_handlers.register(
        "constructor_declaration",
        compute_matching_score_for_method_declaration,
    );
    matching_handlers
}
