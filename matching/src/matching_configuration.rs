use model::Language;
use std::collections::HashSet;

pub struct MatchingConfiguration {
    pub(crate) delimiters: HashSet<&'static str>,
    pub(crate) kinds_with_label: HashSet<&'static str>,
}

impl Default for MatchingConfiguration {
    fn default() -> Self {
        MatchingConfiguration::from(Language::Java)
    }
}

impl From<Language> for MatchingConfiguration {
    fn from(language: Language) -> Self {
        match language {
            Language::Java => MatchingConfiguration {
                delimiters: ["{", "}"].into(),
                kinds_with_label: [
                    "compact_constructor_declaration",
                    "constructor_declaration",
                    "field_declaration",
                    "method_declaration",
                ]
                .into(),
            },
        }
    }
}
