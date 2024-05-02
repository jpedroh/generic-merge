mod java;

use java::get_default_java_parsing_handlers;
use model::Language;

pub type ParsingHandler = fn(root: model::CSTNode) -> model::CSTNode;

pub struct ParsingHandlers {
    handlers: Vec<ParsingHandler>,
}

impl ParsingHandlers {
    pub fn new(handlers: Vec<ParsingHandler>) -> Self {
        Self { handlers }
    }

    pub fn run<'a>(&'a self, root: model::CSTNode<'a>) -> model::CSTNode<'a> {
        self.handlers.iter().fold(root, |acc, handler| handler(acc))
    }
}

impl From<Language> for ParsingHandlers {
    fn from(language: Language) -> Self {
        match language {
            Language::Java => get_default_java_parsing_handlers(),
        }
    }
}
