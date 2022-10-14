use std::path::PathBuf;

use clap::Parser;
use image::GenericImageView;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Expression {
    r#type: String,
    id: usize,
    color: String,
    latex: String,
    #[serde(rename = "fillOpacity")]
    fill_opacity: String,
    #[serde(rename = "lineOpacity")]
    line_opacity: String,
    #[serde(rename = "lineWidth")]
    line_width: String,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to target image
    file: PathBuf,

    /// Vanity hash (in the URL)
    hash: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if cli.file.extension().unwrap() != "png" {
        panic!("File must be a PNG.")
    }

    if cli.file.metadata().unwrap().len() > 5000000 {
        panic!("File must be less than 5 megabytes.")
    }

    let image = image::open(cli.file).unwrap();

    let mut expressions: Vec<Expression> = Vec::new();

    for (x, y, pixel) in image.pixels() {
        let expression = Expression {
            r#type: String::from("expression"),
            id: expressions.len(),
            color: format!("rgb({}, {}, {})", pixel.0[0], pixel.0[1], pixel.0[2]),
            #[rustfmt::skip]
            latex: format!("{}\\le x\\le{}\\left\\{{{}\\le y\\le{}\\right\\}}", y, y + 1, x, x + 1),
            fill_opacity: String::from(""),
            line_opacity: String::from(""),
            line_width: String::from(""),
        };

        expressions.push(expression);
    }

    Ok(())
}
