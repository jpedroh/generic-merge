mod parser_configuration;

use std::{error::Error, fs};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to file in base revision
    #[arg(short, long)]
    base_path: std::path::PathBuf,

    /// Path to file in left revision
    #[arg(short, long)]
    left_path: std::path::PathBuf,

    /// Path to file in right revision
    #[arg(short, long)]
    right_path: std::path::PathBuf,

    /// Path where the merged file should be written
    #[arg(short, long)]
    merge_path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let base = fs::read_to_string(&args.base_path)?;
    let left = fs::read_to_string(args.left_path)?;
    let right = fs::read_to_string(args.right_path)?;

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
