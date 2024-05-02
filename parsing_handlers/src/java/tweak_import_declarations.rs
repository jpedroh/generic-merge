use model::{cst_node::NonTerminal, CSTNode};

pub fn tweak_import_declarations(root: CSTNode<'_>) -> CSTNode<'_> {
    if root.kind() != "program" {
        return root.to_owned();
    }

    match root {
        CSTNode::Terminal(_) => root,
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
                are_children_unordered: program.are_children_unordered,
            })
        }
    }
}
