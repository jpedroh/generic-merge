mod cli_exit_codes;
mod control;

pub use cli_exit_codes::*;
pub use control::{run_diff_on_files, run_tool_on_merge_scenario};
