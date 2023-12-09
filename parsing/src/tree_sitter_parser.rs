use model::Language;
use std::collections::HashSet;

pub struct ParserConfiguration {
    pub(crate) language: tree_sitter::Language,
    pub(crate) stop_compilation_at: HashSet<&'static str>,
    pub(crate) kinds_with_unordered_children: HashSet<&'static str>,
    pub(crate) block_end_delimiters: HashSet<&'static str>,
}

impl From<Language> for ParserConfiguration {
    fn from(language: Language) -> Self {
        match language {
            Language::Java => ParserConfiguration {
                language: tree_sitter_java::language(),
                stop_compilation_at: ["method_body"].into(),
                kinds_with_unordered_children: ["interface_body", "class_body"].into(),
                block_end_delimiters: ["}"].into(),
            },
        }
    }
}
