use matching::ordered_tree_matching;

fn main() {
    let base = parsing::parse_string(
        r#"
        public static interface HelloWorld {
            void sayHello(String name);
        }
    "#,
        parsing::ParserConfiguration::from_language(model::Language::Java),
    )
    .unwrap();
    let left = parsing::parse_string(
        r#"
        public static interface HelloWorld {
            void sayHello(String name);
            void sayBye(String name);
        }
    "#,
        parsing::ParserConfiguration::from_language(model::Language::Java),
    )
    .unwrap();
    let right = parsing::parse_string(
        r#"
        public static interface HelloWorld {
            void killAllHumans();
            void sayHello(String name);
        }
    "#,
        parsing::ParserConfiguration::from_language(model::Language::Java),
    )
    .unwrap();

    let result = ordered_tree_matching(&left, &base);

    // result.into_iter().for_each(|(pair, matching)| {
    //     println!("{:#?}", pair);
    //     println!("{:?}", matching);
    // });
}
