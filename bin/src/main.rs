mod cli_args;
mod cli_exit_codes;
mod control;
mod language;

use clap::Parser;

fn main() {
    let args = cli_args::CliArgs::parse();

    // let args = cli_args::CliArgs {
    //     base_path: "/Users/jpedroh/Projetos/jFSTMerge/testfiles/shelltests/big/base/src/main/java/com/pa/util/EnumPublicationLocalType.java".into(),
    //     left_path: "/Users/jpedroh/Projetos/jFSTMerge/testfiles/shelltests/big/left/src/main/java/com/pa/util/EnumPublicationLocalType.java".into(),
    //     right_path: "/Users/jpedroh/Projetos/jFSTMerge/testfiles/shelltests/big/right/src/main/java/com/pa/util/EnumPublicationLocalType.java".into(),
    //     merge_path: "EnumPublicationLocalType.java".into()
    // };

    let base = std::fs::read_to_string(&args.base_path).unwrap_or_else(|error| {
        println!("Error reading file: {}", error);
        std::process::exit(cli_exit_codes::READING_FILE_ERROR)
    });
    let left = std::fs::read_to_string(args.left_path).unwrap_or_else(|error| {
        println!("Error reading file: {}", error);
        std::process::exit(cli_exit_codes::READING_FILE_ERROR)
    });
    let right = std::fs::read_to_string(args.right_path).unwrap_or_else(|error| {
        println!("Error reading file: {}", error);
        std::process::exit(cli_exit_codes::READING_FILE_ERROR)
    });

    let language = language::get_language_by_file_path(&args.base_path).unwrap_or_else(|error| {
        println!("Error guessing language: {}", error);
        std::process::exit(cli_exit_codes::GUESS_LANGUAGE_ERROR)
    });

    let result = control::run_tool_on_merge_scenario(language, &base, &left, &right)
        .unwrap_or_else(|error| {
            println!("Error running tool: {}", error);
            std::process::exit(cli_exit_codes::INTERNAL_EXECUTION_ERROR)
        });

    std::fs::write(args.merge_path, result.to_string()).unwrap_or_else(|error| {
        println!("Error writing file: {}", error);
        std::process::exit(cli_exit_codes::WRITING_FILE_ERROR)
    });

    match result {
        control::ExecutionResult::WithConflicts(_) => {
            std::process::exit(cli_exit_codes::SUCCESS_WITH_CONFLICTS)
        }
        control::ExecutionResult::WithoutConflicts(_) => {
            std::process::exit(cli_exit_codes::SUCCESS_WITHOUT_CONFLICTS)
        }
    }
}
