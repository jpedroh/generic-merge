use crate::tree_sitter_parser::ParserConfiguration;
use model::{
    cst_node::{NonTerminal, Terminal},
    CSTNode, Point,
};
use tree_sitter::Node;

fn explore_node<'a>(node: Node, src: &'a str, config: &'a ParserConfiguration) -> CSTNode<'a> {
    if node.child_count() == 0 || config.stop_compilation_at.contains(node.kind()) {
        CSTNode::Terminal(Terminal {
            id: uuid::Uuid::new_v4(),
            kind: node.kind(),
            start_position: Point {
                row: node.start_position().row,
                column: node.start_position().column,
            },
            end_position: Point {
                row: node.end_position().row,
                column: node.end_position().column,
            },
            value: &src[node.byte_range()],
            is_block_end_delimiter: config.block_end_delimiters.contains(node.kind()),
        })
    } else {
        let mut cursor = node.walk();
        CSTNode::NonTerminal(NonTerminal {
            id: uuid::Uuid::new_v4(),
            kind: node.kind(),
            start_position: Point {
                row: node.start_position().row,
                column: node.start_position().column,
            },
            end_position: Point {
                row: node.end_position().row,
                column: node.end_position().column,
            },
            children: node
                .children(&mut cursor)
                .map(|child| explore_node(child, src, config))
                .collect(),
            are_children_unordered: config.kinds_with_unordered_children.contains(node.kind()),
        })
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

pub fn parse_string<'a>(
    src: &'a str,
    config: &'a ParserConfiguration,
) -> Result<CSTNode<'a>, &'static str> {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(config.language)
        .map_err(|_| "There was an error while setting the parser language")?;

    let parsed = parser
        .parse(src, None)
        .ok_or("It was not possible to parse the tree.")?;
    let root = explore_node(parsed.root_node(), src, config);
    Ok(tweak_import_declarations(root))
}
