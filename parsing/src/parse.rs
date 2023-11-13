use crate::tree_sitter_parser::ParserConfiguration;
use model::{CSTNode, Point};
use tree_sitter::Node;

fn explore_node<'a>(node: Node, src: &'a str, config: &'a ParserConfiguration) -> CSTNode<'a> {
    if node.child_count() == 0 || config.stop_compilation_at.contains(node.kind()) {
        CSTNode::Terminal {
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
        }
    } else {
        let mut cursor = node.walk();
        CSTNode::NonTerminal {
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
        }
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

    let parsed = parser.parse(src, None);
    match parsed {
        Some(parsed) => Result::Ok(explore_node(parsed.root_node(), src, config)),
        None => Result::Err("It was not possible to parse the tree."),
    }
}

#[cfg(test)]
mod tests {
    use model::CSTNode::{NonTerminal, Terminal};
    use model::Point;

    use super::*;

    #[test]
    fn it_parses_an_interface() {
        let code = r#"
            public static interface HelloWorld {
                void sayHello(String name);
            }
        "#;
        let parser_configuration = ParserConfiguration {
            language: tree_sitter_java::language(),
            stop_compilation_at: [].into_iter().collect(),
        };
        let result = parse_string(code, &parser_configuration);
        let expected = NonTerminal {
            kind: "program",
            children: vec![NonTerminal {
                kind: "interface_declaration",
                children: vec![
                    NonTerminal {
                        kind: "modifiers",
                        children: vec![
                            Terminal {
                                kind: "public",
                                value: "public",
                                start_position: Point { row: 1, column: 12 },
                                end_position: Point { row: 1, column: 18 },
                            },
                            Terminal {
                                kind: "static",
                                value: "static",
                                start_position: Point { row: 1, column: 19 },
                                end_position: Point { row: 1, column: 25 },
                            },
                        ],
                        start_position: Point { row: 1, column: 12 },
                        end_position: Point { row: 1, column: 25 },
                    },
                    Terminal {
                        kind: "interface",
                        value: "interface",
                        start_position: Point { row: 1, column: 26 },
                        end_position: Point { row: 1, column: 35 },
                    },
                    Terminal {
                        kind: "identifier",
                        value: "HelloWorld",
                        start_position: Point { row: 1, column: 36 },
                        end_position: Point { row: 1, column: 46 },
                    },
                    NonTerminal {
                        kind: "interface_body",
                        children: vec![
                            Terminal {
                                kind: "{",
                                value: "{",
                                start_position: Point { row: 1, column: 47 },
                                end_position: Point { row: 1, column: 48 },
                            },
                            NonTerminal {
                                kind: "method_declaration",
                                children: vec![
                                    Terminal {
                                        kind: "void_type",
                                        value: "void",
                                        start_position: Point { row: 2, column: 16 },
                                        end_position: Point { row: 2, column: 20 },
                                    },
                                    Terminal {
                                        kind: "identifier",
                                        value: "sayHello",
                                        start_position: Point { row: 2, column: 21 },
                                        end_position: Point { row: 2, column: 29 },
                                    },
                                    NonTerminal {
                                        kind: "formal_parameters",
                                        children: vec![
                                            Terminal {
                                                kind: "(",
                                                value: "(",
                                                start_position: Point { row: 2, column: 29 },
                                                end_position: Point { row: 2, column: 30 },
                                            },
                                            NonTerminal {
                                                kind: "formal_parameter",
                                                children: vec![
                                                    Terminal {
                                                        kind: "type_identifier",
                                                        value: "String",
                                                        start_position: Point {
                                                            row: 2,
                                                            column: 30,
                                                        },
                                                        end_position: Point { row: 2, column: 36 },
                                                    },
                                                    Terminal {
                                                        kind: "identifier",
                                                        value: "name",
                                                        start_position: Point {
                                                            row: 2,
                                                            column: 37,
                                                        },
                                                        end_position: Point { row: 2, column: 41 },
                                                    },
                                                ],
                                                start_position: Point { row: 2, column: 30 },
                                                end_position: Point { row: 2, column: 41 },
                                            },
                                            Terminal {
                                                kind: ")",
                                                value: ")",
                                                start_position: Point { row: 2, column: 41 },
                                                end_position: Point { row: 2, column: 42 },
                                            },
                                        ],
                                        start_position: Point { row: 2, column: 29 },
                                        end_position: Point { row: 2, column: 42 },
                                    },
                                    Terminal {
                                        kind: ";",
                                        value: ";",
                                        start_position: Point { row: 2, column: 42 },
                                        end_position: Point { row: 2, column: 43 },
                                    },
                                ],
                                start_position: Point { row: 2, column: 16 },
                                end_position: Point { row: 2, column: 43 },
                            },
                            Terminal {
                                kind: "}",
                                value: "}",
                                start_position: Point { row: 3, column: 12 },
                                end_position: Point { row: 3, column: 13 },
                            },
                        ],
                        start_position: Point { row: 1, column: 47 },
                        end_position: Point { row: 3, column: 13 },
                    },
                ],
                start_position: Point { row: 1, column: 12 },
                end_position: Point { row: 3, column: 13 },
            }],
            start_position: Point { row: 1, column: 12 },
            end_position: Point { row: 4, column: 8 },
        };
        assert_eq!(expected, result.unwrap())
    }

    #[test]
    fn it_stops_the_compilation_when_reach_a_configured_node() {
        let code = "public static interface HelloWorld {void sayHello(String name);}";
        let parser_configuration = ParserConfiguration {
            language: tree_sitter_java::language(),
            stop_compilation_at: ["interface_body"].into_iter().collect(),
        };
        let result = parse_string(code, &parser_configuration);

        let expected = NonTerminal {
            kind: "program",
            children: vec![NonTerminal {
                kind: "interface_declaration",
                children: vec![
                    NonTerminal {
                        kind: "modifiers",
                        children: vec![
                            Terminal {
                                kind: "public",
                                value: "public",
                                start_position: Point { row: 0, column: 0 },
                                end_position: Point { row: 0, column: 6 },
                            },
                            Terminal {
                                kind: "static",
                                value: "static",
                                start_position: Point { row: 0, column: 7 },
                                end_position: Point { row: 0, column: 13 },
                            },
                        ],
                        start_position: Point { row: 0, column: 0 },
                        end_position: Point { row: 0, column: 13 },
                    },
                    Terminal {
                        kind: "interface",
                        value: "interface",
                        start_position: Point { row: 0, column: 14 },
                        end_position: Point { row: 0, column: 23 },
                    },
                    Terminal {
                        kind: "identifier",
                        value: "HelloWorld",
                        start_position: Point { row: 0, column: 24 },
                        end_position: Point { row: 0, column: 34 },
                    },
                    Terminal {
                        kind: "interface_body",
                        value: "{void sayHello(String name);}",
                        start_position: Point { row: 0, column: 35 },
                        end_position: Point { row: 0, column: 64 },
                    },
                ],
                start_position: Point { row: 0, column: 0 },
                end_position: Point { row: 0, column: 64 },
            }],
            start_position: Point { row: 0, column: 0 },
            end_position: Point { row: 0, column: 64 },
        };
        assert_eq!(expected, result.unwrap())
    }
}
