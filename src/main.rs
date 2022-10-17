use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to target file
    file: PathBuf,

    /// Optional vanity hash
    hash: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let file = args.file;
    let hash = args.hash;

    if !file.exists() {
        panic!("File does not exist.")
    }

    if file.extension().unwrap() != "png" {
        panic!("File must be a PNG.")
    }

    if hash.is_some() {
        let hash = hash.as_ref().unwrap();

        if hash.len() != 10 {
            panic!("Hash must be 10 characters long.")
        }

        if hash.chars().any(|char| !char.is_ascii_alphanumeric()) {
            panic!("Hash must be an ASCII alphanumeric string.")
        }
    }

    let image = image::open(&file)?;

    Ok(())
}
