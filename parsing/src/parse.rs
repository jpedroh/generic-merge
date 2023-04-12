use crate::tree_sitter_parser::ParserConfiguration;
use model::CSTNode;
use tree_sitter::Node;

fn explore_node(node: Node, src: &str, config: &ParserConfiguration) -> CSTNode {
    if node.child_count() == 0 || config.stop_compilation_at.contains(node.kind()) {
        CSTNode::Terminal {
            kind: node.kind().into(),
            value: src[node.byte_range()].into(),
        }
    } else {
        let mut cursor = node.walk();
        CSTNode::NonTerminal {
            kind: node.kind().into(),
            children: node
                .children(&mut cursor)
                .map(|child| explore_node(child, src, config))
                .collect(),
        }
    }
}

pub fn parse_string(src: &str, config: ParserConfiguration) -> Result<CSTNode, &'static str> {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(config.language)
        .map_err(|_| "There was an error while setting the parser language")?;

    let parsed = parser.parse(src, None);
    match parsed {
        Some(parsed) => Result::Ok(explore_node(parsed.root_node(), src, &config)),
        None => Result::Err("It was not possible to parse the tree."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_an_interface() {
        let code = r#"
            public static interface HelloWorld {
                void sayHello(String name);
            }
        "#;
        let result = parse_string(
            code,
            ParserConfiguration {
                language: tree_sitter_java::language(),
                stop_compilation_at: [].into_iter().collect(),
            },
        );
        let expected = CSTNode::NonTerminal {
            kind: "program".into(),
            children: vec![CSTNode::NonTerminal {
                kind: "interface_declaration".into(),
                children: vec![
                    CSTNode::NonTerminal {
                        kind: "modifiers".into(),
                        children: vec![
                            CSTNode::Terminal {
                                kind: "public".into(),
                                value: "public".into(),
                            },
                            CSTNode::Terminal {
                                kind: "static".into(),
                                value: "static".into(),
                            },
                        ],
                    },
                    CSTNode::Terminal {
                        kind: "interface".into(),
                        value: "interface".into(),
                    },
                    CSTNode::Terminal {
                        kind: "identifier".into(),
                        value: "HelloWorld".into(),
                    },
                    CSTNode::NonTerminal {
                        kind: "interface_body".into(),
                        children: vec![
                            CSTNode::Terminal {
                                kind: "{".into(),
                                value: "{".into(),
                            },
                            CSTNode::NonTerminal {
                                kind: "method_declaration".into(),
                                children: vec![
                                    CSTNode::Terminal {
                                        kind: "void_type".into(),
                                        value: "void".into(),
                                    },
                                    CSTNode::Terminal {
                                        kind: "identifier".into(),
                                        value: "sayHello".into(),
                                    },
                                    CSTNode::NonTerminal {
                                        kind: "formal_parameters".into(),
                                        children: vec![
                                            CSTNode::Terminal {
                                                kind: "(".into(),
                                                value: "(".into(),
                                            },
                                            CSTNode::NonTerminal {
                                                kind: "formal_parameter".into(),
                                                children: vec![
                                                    CSTNode::Terminal {
                                                        kind: "type_identifier".into(),
                                                        value: "String".into(),
                                                    },
                                                    CSTNode::Terminal {
                                                        kind: "identifier".into(),
                                                        value: "name".into(),
                                                    },
                                                ],
                                            },
                                            CSTNode::Terminal {
                                                kind: ")".into(),
                                                value: ")".into(),
                                            },
                                        ],
                                    },
                                    CSTNode::Terminal {
                                        kind: ";".into(),
                                        value: ";".into(),
                                    },
                                ],
                            },
                            CSTNode::Terminal {
                                kind: "}".into(),
                                value: "}".into(),
                            },
                        ],
                    },
                ],
            }],
        };
        assert_eq!(expected, result.unwrap())
    }

    #[test]
    fn it_stops_the_compilation_when_reach_a_configured_node() {
        let code = "public static interface HelloWorld {void sayHello(String name);}";
        let result = parse_string(
            code,
            ParserConfiguration {
                language: tree_sitter_java::language(),
                stop_compilation_at: ["interface_body"].into_iter().collect(),
            },
        );

        let expected = CSTNode::NonTerminal {
            kind: "program".into(),
            children: vec![CSTNode::NonTerminal {
                kind: "interface_declaration".into(),
                children: vec![
                    CSTNode::NonTerminal {
                        kind: "modifiers".into(),
                        children: vec![
                            CSTNode::Terminal {
                                kind: "public".into(),
                                value: "public".into(),
                            },
                            CSTNode::Terminal {
                                kind: "static".into(),
                                value: "static".into(),
                            },
                        ],
                    },
                    CSTNode::Terminal {
                        kind: "interface".into(),
                        value: "interface".into(),
                    },
                    CSTNode::Terminal {
                        kind: "identifier".into(),
                        value: "HelloWorld".into(),
                    },
                    CSTNode::Terminal {
                        kind: "interface_body".into(),
                        value: "{void sayHello(String name);}".into(),
                    },
                ],
            }],
        };
        assert_eq!(expected, result.unwrap())
    }
}
