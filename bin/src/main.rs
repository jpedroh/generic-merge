mod cli_args;
mod parser_configuration;

use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli_args::CliArgs::parse();

    let base = std::fs::read_to_string(&args.base_path)?;
    let left = std::fs::read_to_string(args.left_path)?;
    let right = std::fs::read_to_string(args.right_path)?;

    let parser_configuration =
        parser_configuration::get_parser_configuration_by_file_path(&args.base_path)?;

    let base_tree = parsing::parse_string(&base, &parser_configuration).unwrap();
    let left_tree = parsing::parse_string(&left, &parser_configuration).unwrap();
    let right_tree = parsing::parse_string(&right, &parser_configuration).unwrap();

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

    std::fs::write(args.merge_path, result.to_string())?;

    Ok(())
}
