use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    /// Runs only the diffing algorithm and outputs if the two files matches.
    #[arg(short, long, num_args = 0)]
    pub(crate) diff_only: bool,

    /// Path to file in base revision
    #[arg(short, long, requires_if("false", "diff_only"))]
    pub(crate) base_path: Option<std::path::PathBuf>,

    /// Path to file in left revision
    #[arg(short, long)]
    pub(crate) left_path: std::path::PathBuf,

    /// Path to file in right revision
    #[arg(short, long)]
    pub(crate) right_path: std::path::PathBuf,

    /// Path where the merged file should be written
    #[arg(short, long, requires_if("false", "diff_only"))]
    pub(crate) merge_path: Option<std::path::PathBuf>,

    /// The language that the files being merged are written in.
    /// If not provided the language will try to be inferred by the extension of the base file.
    #[arg(long)]
    pub(crate) language: Option<String>,

    /// The log level provided for the execution.
    /// If not provided defaults to INFO.
    #[arg(long)]
    pub(crate) log_level: Option<log::LevelFilter>,
}
