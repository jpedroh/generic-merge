fn main() {
    let base = r#"
        public interface Repository {
        }
    "#;
    let left: &str = r#"
        public interface Repository {
            void create(Pessoa pessoa);
            void delete(Pessoa pessoa);
        }
    "#;

    let right = r#"
        public interface Repository {
            void create(Pessoa pessoa);
            void delete(Pessoa pessoa);
            void remove(Pessoa pessoa);
            void insert(Pessoa pessoa);
        }
    "#;

    // let base = r#"
    //     public class Main {
    //         public static void main(String[] args) {
    //             System.out.println("Hello, world!");
    //             int y = 4;
    //             int j = 0;
    //         }
    //     }
    // "#;
    // let left = r#"
    //     public class Main {
    //         public static void main(String[] args) {
    //             int x = 0;
    //             System.out.println("Hello, João!");
    //             int y = 3;
    //             int j = 0;
    //         }
    //     }
    // "#;
    // let right = r#"
    //     public class Main {
    //         public static void main(String[] args) {
    //             System.out.println("Hello, Paulo!");
    //             int y = 3;
    //         }
    //     }
    // "#;

    let parser_configuration = parsing::ParserConfiguration::from(model::Language::Java);

    let base_tree = parsing::parse_string(base, &parser_configuration).unwrap();
    let left_tree = parsing::parse_string(left, &parser_configuration).unwrap();
    let right_tree = parsing::parse_string(right, &parser_configuration).unwrap();

    let matchings_left_base = matching::calculate_matchings(&left_tree, &base_tree);
    let matchings_right_base = matching::calculate_matchings(&right_tree, &base_tree);
    let matchings_left_right = matching::calculate_matchings(&left_tree, &right_tree);

    let result = merge::merge(
        &base_tree,
        &left_tree,
        &right_tree,
        &matchings_left_base,
        &matchings_right_base,
        &matchings_left_right,
    );

    println!("{}", result.to_string())
}
