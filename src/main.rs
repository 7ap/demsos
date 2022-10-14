use std::fs;
use std::path::PathBuf;

use base64::encode;
use clap::Parser;
use image::GenericImageView;
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to target image
    file: PathBuf,

    /// Vanity hash (in the URL)
    hash: Option<String>,
}

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

#[derive(Serialize, Deserialize)]
struct Viewport {
    xmin: i32,
    xmax: i32,
    ymin: i32,
    ymax: i32,
}

#[derive(Serialize, Deserialize)]
struct Graph {
    viewport: Viewport,
}

#[derive(Serialize, Deserialize)]
struct Expressions {
    list: Vec<Expression>,
}

#[derive(Serialize, Deserialize)]
struct CalcState {
    version: i32,
    #[serde(rename = "randomSeed")]
    random_seed: String,
    graph: Graph,
    expressions: Expressions,
}

#[derive(Serialize, Deserialize)]
struct SaveData {
    thumb_data: String,
    calc_state: String,
    is_update: String,
    lang: String,
    my_graphs: String,
    graph_hash: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let file = cli.file;
    let hash = cli.hash.unwrap();

    if file.extension().unwrap() != "png" {
        panic!("File must be a PNG.")
    }

    if file.metadata()?.len() > 5000000 {
        panic!("File must be less than 5 megabytes.")
    }

    if String::len(&hash) != 10 {
        panic!("Hash must be 10 characters.")
    }

    let image = image::open(&file).unwrap();

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

    let calc_state = CalcState {
        version: 9,
        random_seed: String::from("TOOD"),
        graph: Graph {
            viewport: Viewport {
                xmin: -170,
                xmax: 170,
                ymin: -100,
                ymax: 100,
            },
        },
        expressions: Expressions { list: expressions },
    };

    let save_data = SaveData {
        thumb_data: format!("data:image/png;base64,{}", encode(fs::read(file)?)),
        calc_state: serde_json::to_string(&calc_state)?,
        is_update: String::from("false"),
        lang: String::from("en"),
        my_graphs: String::from("false"),
        graph_hash: hash, // TODO: generate random 10 character long hexadecimal string if no hash is provided
    };

    let client = reqwest::Client::new();
    let response = client.post("https://www.desmos.com/api/v1/calculator/save")
        .json(&save_data)
        .send()
        .await?;

    // TODO: make this more descriptive
    if response.status() != 200 {
        panic!("Something went wrong when uploading the file.");
    }

    println!("Graph URL: https://desmos.com/calculator/{}", &save_data.graph_hash);

    Ok(())
}
