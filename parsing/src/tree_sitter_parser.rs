use model::Language;
use std::collections::HashSet;

pub struct TreeSitterParserConfiguration {
    pub language: tree_sitter::Language,
    pub stop_compilation_at: HashSet<&'static str>,
}

pub fn from_language(language: Language) -> TreeSitterParserConfiguration {
    match language {
        Language::Java => TreeSitterParserConfiguration {
            language: tree_sitter_java::language(),
            stop_compilation_at: vec!["method_body"].into_iter().collect(),
        },
    }
}
