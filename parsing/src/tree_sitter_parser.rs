use model::{cst_node::NonTerminal, CSTNode, Language};
use std::collections::HashSet;

pub type ParsingHandler = fn(root: CSTNode) -> CSTNode;

pub struct ParsingHandlers {
    handlers: Vec<ParsingHandler>,
}

impl ParsingHandlers {
    pub fn new(handlers: Vec<ParsingHandler>) -> Self {
        Self { handlers }
    }

    pub fn run<'a>(&'a self, root: CSTNode<'a>) -> CSTNode<'a> {
        self.handlers.iter().fold(root, |acc, handler| handler(acc))
    }
}


pub struct ParserConfiguration {
    pub(crate) language: tree_sitter::Language,
    pub(crate) stop_compilation_at: HashSet<&'static str>,
    pub(crate) kinds_with_unordered_children: HashSet<&'static str>,
    pub(crate) block_end_delimiters: HashSet<&'static str>,
    pub(crate) handlers: ParsingHandlers,
}

impl From<Language> for ParserConfiguration {
    fn from(language: Language) -> Self {
        match language {
            Language::Java => ParserConfiguration {
                language: tree_sitter_java::language(),
                stop_compilation_at: [].into(),
                kinds_with_unordered_children: [
                    "interface_body",
                    "class_body",
                    "enum_body_declarations",
                ]
                .into(),
                block_end_delimiters: ["}"].into(),
                handlers: ParsingHandlers::new(vec![tweak_import_declarations]),
            },
        }
    }
}

fn tweak_import_declarations(root: CSTNode<'_>) -> CSTNode<'_> {
    match root.kind() {
        "program" => match root {
            CSTNode::Terminal(_) => root.to_owned(),
            CSTNode::NonTerminal(program) => {
                let import_declaration_children: Vec<CSTNode> = program
                    .children
                    .iter()
                    .filter(|node| node.kind() == "import_declaration")
                    .cloned()
                    .collect();

                if import_declaration_children.is_empty() {
                    return CSTNode::NonTerminal(program);
                }

                let import_declarations_start = import_declaration_children
                    .first()
                    .unwrap()
                    .start_position();

                let import_declarations_end =
                    import_declaration_children.last().unwrap().end_position();

                let import_declarations = CSTNode::NonTerminal(NonTerminal {
                    id: uuid::Uuid::new_v4(),
                    kind: "import_declarations",
                    children: import_declaration_children,
                    start_position: import_declarations_start,
                    end_position: import_declarations_end,
                    are_children_unordered: true,
                });

                let first_import_declaration_index = program
                    .children
                    .iter()
                    .position(|node| node.kind() == "import_declaration")
                    .unwrap();
                let last_import_declaration_index = program
                    .children
                    .iter()
                    .rposition(|node| node.kind() == "import_declaration")
                    .unwrap();

                let mut new_program_children: Vec<CSTNode<'_>> = vec![];
                new_program_children.extend_from_slice(
                    &program.children.iter().as_slice()[..first_import_declaration_index],
                );
                new_program_children.push(import_declarations);
                new_program_children.extend_from_slice(
                    &program.children.iter().as_slice()[last_import_declaration_index + 1..],
                );

                CSTNode::NonTerminal(NonTerminal {
                    id: program.id,
                    kind: program.kind,
                    start_position: program.start_position,
                    end_position: program.end_position,
                    children: new_program_children,
                    are_children_unordered: false,
                })
            }
        },
        _ => root.to_owned(),
    }
}