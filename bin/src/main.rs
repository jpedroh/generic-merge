mod cli_args;
mod control;
mod language;

use clap::Parser;

mod cli_exit_codes {
    pub const SUCCESS_WITHOUT_CONFLICTS: i32 = 0;
    pub const SUCCESS_WITH_CONFLICTS: i32 = 1;

    pub const READING_FILE_ERROR: i32 = 129;
    pub const GUESS_LANGUAGE_ERROR: i32 = 130;
    pub const WRITING_FILE_ERROR: i32 = 131;
    pub const INTERNAL_EXECUTION_ERROR: i32 = 132;
}

fn main() {
    let args = cli_args::CliArgs::parse();

    let base = std::fs::read_to_string(&args.base_path)
        .unwrap_or_else(|_| std::process::exit(cli_exit_codes::READING_FILE_ERROR));
    let left = std::fs::read_to_string(args.left_path)
        .unwrap_or_else(|_| std::process::exit(cli_exit_codes::READING_FILE_ERROR));
    let right = std::fs::read_to_string(args.right_path)
        .unwrap_or_else(|_| std::process::exit(cli_exit_codes::READING_FILE_ERROR));

    let language = language::get_language_by_file_path(&args.base_path)
        .unwrap_or_else(|_| std::process::exit(cli_exit_codes::GUESS_LANGUAGE_ERROR));

    let result = control::run_tool_on_merge_scenario(language, &base, &left, &right)
        .unwrap_or_else(|_| std::process::exit(cli_exit_codes::INTERNAL_EXECUTION_ERROR));

    std::fs::write(args.merge_path, result.to_string())
        .unwrap_or_else(|_| std::process::exit(cli_exit_codes::WRITING_FILE_ERROR));

    match result {
        control::ExecutionResult::WithConflicts(_) => {
            std::process::exit(cli_exit_codes::SUCCESS_WITH_CONFLICTS)
        }
        control::ExecutionResult::WithoutConflicts(_) => {
            std::process::exit(cli_exit_codes::SUCCESS_WITHOUT_CONFLICTS)
        }
    }
}
