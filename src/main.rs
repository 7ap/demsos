use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to target image
    image: PathBuf,

    /// Optional vanity hash
    hash: Option<String>,
}

fn main() {
    let args = Args::parse();
}
