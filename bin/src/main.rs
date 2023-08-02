use matching::ordered_tree_matching;
use merge::merge;
use model::CSTNode;

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

    let matchings_left_base = ordered_tree_matching(&left, &base);
    let matchings_right_base = ordered_tree_matching(&right, &base);
    let matchings_lef_right = ordered_tree_matching(&left, &right);
    let result = merge(
        &base,
        &left,
        &right,
        &matchings_left_base,
        &matchings_right_base,
        &matchings_lef_right,
    );

    println!("{:#?}", pretty_print(result))
}

pub fn pretty_print(node: CSTNode) -> String {
    match node {
        CSTNode::Terminal { value, .. } => value,
        CSTNode::NonTerminal { children, .. } => {
            children.iter().fold(String::new(), |acc, current| {
                let mut result = acc.to_owned();
                result.push_str(" ");
                result.push_str(&pretty_print(current.clone()));
                result
            })
        }
    }
}
