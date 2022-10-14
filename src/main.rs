use core::panic;
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

    if cli.file.extension().unwrap() != "png" {
        panic!("File must be a PNG.")
    }

    if cli.file.metadata().unwrap().len() > 5000000 {
        panic!("File must be less than 5 megabytes.")
    }
}
