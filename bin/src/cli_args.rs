use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    /// Path to file in base revision
    #[arg(short, long)]
    pub(crate) base_path: std::path::PathBuf,

    /// Path to file in left revision
    #[arg(short, long)]
    pub(crate) left_path: std::path::PathBuf,

    /// Path to file in right revision
    #[arg(short, long)]
    pub(crate) right_path: std::path::PathBuf,

    /// Path where the merged file should be written
    #[arg(short, long)]
    pub(crate) merge_path: std::path::PathBuf,

    /// The language that the files being merged are written in.
    /// If not provided the language will try to be inferred by the extension of the base file.
    #[arg(long)]
    pub(crate) language: Option<String>,
}
