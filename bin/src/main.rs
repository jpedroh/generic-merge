fn main() {
    let base = r#"
        public static interface HelloWorld {
            void sayHello(String name);
        }
    "#;
    let left = r#"
        public static interface HelloWorld {
            void sayHello(String name);
        }
    "#;
    let right = r#"
        public static interface HelloWorld {
            void sayHello(String name);
        }
    "#;

    let parser_configuration = parsing::ParserConfiguration::from_language(model::Language::Java);

    let base_tree = parsing::parse_string(base, &parser_configuration).unwrap();
    let left_tree = parsing::parse_string(left, &parser_configuration).unwrap();
    let right_tree = parsing::parse_string(right, &parser_configuration).unwrap();

    let matchings_left_base = matching::ordered_tree_matching(&left_tree, &base_tree);
    let matchings_right_base = matching::ordered_tree_matching(&right_tree, &base_tree);
    let matchings_left_right = matching::ordered_tree_matching(&left_tree, &right_tree);

    let result = merge::merge(
        &base_tree,
        &left_tree,
        &right_tree,
        &matchings_left_base,
        &matchings_right_base,
        &matchings_left_right,
    );

    println!("{:#?}", result)
}
