mod cli_args;
mod cli_exit_codes;
mod control;
mod language;

use clap::Parser;

fn main() {
    env_logger::init();

    let args = cli_args::CliArgs::parse();

    let base = std::fs::read_to_string(&args.base_path).unwrap_or_else(|error| {
        log::error!("Error while reading base file: {}", error);
        std::process::exit(cli_exit_codes::READING_FILE_ERROR)
    });
    let left = std::fs::read_to_string(&args.left_path).unwrap_or_else(|error| {
        log::error!("Error while reading left file: {}", error);
        std::process::exit(cli_exit_codes::READING_FILE_ERROR)
    });
    let right = std::fs::read_to_string(&args.right_path).unwrap_or_else(|error| {
        log::error!("Error while reading right file: {}", error);
        std::process::exit(cli_exit_codes::READING_FILE_ERROR)
    });

    let language = language::get_language_from_name(&args.language).unwrap_or_else(|error| {
        log::error!("Error while guessing language: {}", error);
        std::process::exit(cli_exit_codes::GUESS_LANGUAGE_ERROR)
    });

    let result = control::run_tool_on_merge_scenario(language, &base, &left, &right)
        .unwrap_or_else(|error| {
            log::error!("Error while running tool: {}", error);
            std::process::exit(cli_exit_codes::INTERNAL_EXECUTION_ERROR)
        });

    std::fs::write(args.merge_path, result.to_string()).unwrap_or_else(|error| {
        log::error!("Error while writing output file: {}", error);
        std::process::exit(cli_exit_codes::WRITING_FILE_ERROR)
    });

    match result {
        control::ExecutionResult::WithConflicts(_) => {
            log::info!("Execution finished with conflicts");
            std::process::exit(cli_exit_codes::SUCCESS_WITH_CONFLICTS)
        }
        control::ExecutionResult::WithoutConflicts(_) => {
            log::info!("Execution finished without conflicts");
            std::process::exit(cli_exit_codes::SUCCESS_WITHOUT_CONFLICTS)
        }
    }
}
