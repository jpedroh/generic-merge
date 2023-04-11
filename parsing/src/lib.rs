use tree_sitter::{Node, Parser};

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum CSTNode {
    Terminal {
        kind: String,
        value: String,
    },
    NonTerminal {
        kind: String,
        children: Vec<CSTNode>,
    },
}

fn explore_node(node: Node, src: &str) -> CSTNode {
    if node.child_count() == 0 {
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
                .map(|child| explore_node(child, src))
                .collect(),
        }
    }
}

pub fn parse_string(src: &str, parser: &mut Parser) -> CSTNode {
    let parsed = parser.parse(src, None).unwrap();
    let mut cursor = parsed.root_node().walk();

    CSTNode::NonTerminal {
        kind: "program".into(),
        children: parsed
            .root_node()
            .children(&mut cursor)
            .into_iter()
            .map(|child| explore_node(child, src))
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use tree_sitter::Parser;

    use super::*;

    #[test]
    fn it_parses_an_interface() {
        let code = r#"
            public static interface HelloWorld {
                void sayHello(String name);
            }
        "#;
        let mut parser = Parser::new();
        parser
            .set_language(tree_sitter_java::language())
            .expect("Error loading Java grammar");
        let result = parse_string(code, &mut parser);
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
        assert_eq!(expected, result)
    }
}
