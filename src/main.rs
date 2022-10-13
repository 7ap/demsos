use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to target image
    file: PathBuf,

    /// Vanity hash (in the URL)
    hash: Option<String>,
}
fn main() {
    let cli = Cli::parse();
}
