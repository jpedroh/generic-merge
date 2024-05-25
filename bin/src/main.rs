mod cli_args;
mod cli_exit_codes;
mod control;
mod language;

use clap::Parser;

fn main() {
    let args = cli_args::CliArgs::parse();
    env_logger::builder().filter_level(args.log_level.unwrap_or(log::LevelFilter::Info)).init();

    log::info!("Starting Generic Merge tool execution");
    log::debug!("Parsed arguments: {:?}", args);

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

    let language = match args.language {
        Some(language) => language::get_language_from_name(&language),
        None => language::get_language_by_file_path(&args.base_path),
    }
    .unwrap_or_else(|error| {
        log::error!("Error while retrieving language configuration: {}", error);
        std::process::exit(cli_exit_codes::INVALID_LANGUAGE_ERROR)
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
