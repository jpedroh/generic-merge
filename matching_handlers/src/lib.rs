mod java;

use std::collections::HashMap;

use java::get_default_java_matching_handlers;
use model::{CSTNode, Language};

type MatchingHandler<'a> = fn(left: &'a CSTNode<'a>, right: &'a CSTNode<'a>) -> usize;

pub struct MatchingHandlers<'a> {
    matching_handlers: HashMap<&'static str, MatchingHandler<'a>>,
}

impl<'a> Default for MatchingHandlers<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> MatchingHandlers<'a> {
    pub fn new() -> Self {
        Self {
            matching_handlers: HashMap::new(),
        }
    }

    pub fn register(&mut self, key: &'static str, value: MatchingHandler<'a>) {
        self.matching_handlers.insert(key, value);
    }

    pub fn compute_matching_score(&'a self, left: &'a CSTNode, right: &'a CSTNode) -> usize {
        if left.kind() != right.kind() {
            return 0;
        }

        return self
            .matching_handlers
            .get(left.kind())
            .map_or(0, |handler| handler(left, right));
    }
}

impl From<Language> for MatchingHandlers<'_> {
    fn from(language: Language) -> Self {
        match language {
            Language::Java => get_default_java_matching_handlers(),
        }
    }
}
