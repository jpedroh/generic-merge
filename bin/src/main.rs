use matching::ordered_tree_matching;
use merge::merge;
use model::CSTNode;
use parsing::ParserConfiguration;

// fn run_semi_structured_merge_on_revisions<'a>(
//     language: model::Language,
//     base: &'a str,
//     left: &'a str,
//     right: &'a str,
// ) -> Result<CSTNode<'a>, &'static str> {
//     let parser_configuration = ParserConfiguration::from_language(language);

//     let base_tree = parsing::parse_string(base, &parser_configuration)?;
//     let left_tree = parsing::parse_string(left, &parser_configuration)?;
//     let right_tree = parsing::parse_string(right, &parser_configuration)?;

//     let matchings_left_base = ordered_tree_matching(&left_tree, &base_tree);
//     let matchings_right_base = ordered_tree_matching(&right_tree, &base_tree);
//     let matchings_left_right = ordered_tree_matching(&left_tree, &right_tree);

//     Ok(merge(
//         &base_tree.clone(),
//         &left_tree.clone(),
//         &right_tree.clone(),
//         &matchings_left_base.clone(),
//         &matchings_right_base.clone(),
//         &matchings_left_right.clone(),
//     ))
// }

fn main() {
    let base = r#"
        public static interface HelloWorld {
            void sayHello(String name);
        }
    "#;
    let left = r#"
        public static interface HelloWorld {
            void sayHello(String name);
            void sayBye(String name);
        }
    "#;
    let right = r#"
        public static interface HelloWorld {
            void killAllHumans();
            void sayHello(String name);
        }
    "#;

    // let result = run_semi_structured_merge_on_revisions(model::Language::Java, base, left, right);

    // println!("{:#?}", pretty_print(result.unwrap()))
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
        CSTNode::Conflict { .. } => "Conflict found".into(),
    }
}

// #[cfg(test)]
// mod tests {
//     use model::language::Language;
//     use model::CSTNode;
//     use parsing::ParserConfiguration;

//     use crate::run_semi_structured_merge_on_revisions;

//     fn parse_java_string(contents: &str) -> CSTNode {
//         parsing::parse_string(contents, ParserConfiguration::from_language(model::Language::Java)).unwrap()
//     }

//     #[test]
//     fn it_merges_three_java_revisions_that_are_equal() {
//         let code = r#"
//             public static interface HelloWorld {
//                 void sayHello(String name);
//                 void sayBye(String name);
//             }
//         "#;

//         let result =
//             run_semi_structured_merge_on_revisions(model::Language::Java, code, code, code);

//         assert_eq!(parse_java_string(code), result.unwrap())
//     }

//     #[test]
//     fn it_merges_three_java_revisions_that_adds_the_same_node() {
//         let base = r#"
//             public static interface HelloWorld {
//                 void sayBye(String name);
//             }
//         "#;

//         let parents = r#"
//             public static interface HelloWorld {
//                 void sayHello(String name);
//                 void sayBye(String name);
//             }
//         "#;

//         let merge = r#"
//             public static interface HelloWorld {
//                 void sayHello(String name);
//                 void sayBye(String name);
//             }
//         "#;

//         let result =
//             run_semi_structured_merge_on_revisions(model::Language::Java, base, parents, parents);

//         assert_eq!(parse_java_string(merge), result.unwrap())
//     }

//     #[test]
//     fn it_merges_three_java_revisions_that_removes_the_same_node() {
//         let base = r#"
//             public static interface HelloWorld {
//                 void sayHello(String name);
//                 void sayBye(String name);
//             }
//         "#;

//         let parents = r#"
//             public static interface HelloWorld {
//                 void sayBye(String name);
//             }
//         "#;

//         let merge = r#"
//             public static interface HelloWorld {
//                 void sayBye(String name);
//             }
//         "#;

//         let result =
//             run_semi_structured_merge_on_revisions(model::Language::Java, base, parents, parents);

//         assert_eq!(parse_java_string(merge), result.unwrap())
//     }
// }
