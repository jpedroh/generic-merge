use model::Language;
use std::collections::HashSet;

pub struct ParserConfiguration {
    pub(crate) language: tree_sitter::Language,
    pub(crate) stop_compilation_at: HashSet<&'static str>,
}

impl ParserConfiguration {
    pub fn from_language(language: Language) -> ParserConfiguration {
        match language {
            Language::Java => ParserConfiguration {
                language: tree_sitter_java::language(),
                stop_compilation_at: vec!["method_body"].into_iter().collect(),
            },
        }
    }
}
