use matching::matching_configuration::MatchingConfiguration;
use model::language::Language;
use parsing::ParserConfiguration;

#[test]
fn the_perfect_matching_calculation_is_correct() -> Result<(), Box<dyn std::error::Error>> {
    let config = ParserConfiguration::from(Language::Java);
    let left = parsing::parse_string(
        r#"""
            public class Main {
                static {
                    int x = 2;
                }

                public static void main() {
                    int a = 0;
                }

                public static void teste() {
                    
                }
            }
        """#,
        &config,
    )?;

    let right = parsing::parse_string(
        r#"""
            public class Main {
                public static void teste() {
                    
                }
                static {
                    int x = 2;
                }

                public static void main() {
                    int a = 0;

                }
            }
        """#,
        &config,
    )?;

    let matching_configuration = MatchingConfiguration::from(Language::Java);
    let matchings = matching::calculate_matchings(&left, &right, &matching_configuration);
    assert!(
        matchings
            .get_matching_entry(&left, &right)
            .unwrap()
            .is_perfect_match
    );
    Ok(())
}
