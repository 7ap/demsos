use std::io::prelude::*;
use std::path::PathBuf;
use std::{fs, fs::File};

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
    let hash = cli.hash;

    if file.extension().unwrap() != "png" {
        panic!("File must be a PNG.")
    }

    if file.metadata()?.len() > 5000000 {
        panic!("File must be less than 5 megabytes.")
    }

    let image = image::open(&file).unwrap().fliph();

    let mut expressions: Vec<Expression> = Vec::new();

    for (x, y, pixel) in image.pixels() {
        let expression = Expression {
            r#type: String::from("expression"),
            id: expressions.len(),
            color: format!("rgb({}, {}, {})", pixel.0[0], pixel.0[1], pixel.0[2]),
            #[rustfmt::skip]
            latex: format!("{}\\le y\\le{}\\left\\{{{}\\le x\\le{}\\right\\}}", x, x + 1, y, y + 1),
            fill_opacity: String::from("1"),
            line_opacity: String::from("1"),
            line_width: String::from("2"),
        };

        expressions.push(expression);
    }

    let calc_state = CalcState {
        version: 9,
        random_seed: format!("{:x}", rand::random::<u128>()),
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

    let mut graph_hash = format!("{:x}", rand::random::<u128>())
        .chars()
        .take(10)
        .collect::<String>();

    if hash.is_some() {
        let hash = hash.unwrap();

        if hash.len() != 10 {
            panic!("Hash must be 10 characters long.")
        }

        if hash.chars().any(|c| !c.is_ascii_hexdigit()) {
            panic!("Hash must be a hexadecimal string.")
        }

        if hash.chars().any(|c| c.is_ascii_uppercase()) {
            panic!("Hash must be lowercase.")
        }

        graph_hash = hash;
    }

    let save_data = SaveData {
        thumb_data: format!("data:image/png;base64,{}", encode(fs::read(file)?)),
        calc_state: serde_json::to_string(&calc_state)?,
        is_update: String::from("false"),
        lang: String::from("en"),
        my_graphs: String::from("false"),
        graph_hash: graph_hash,
    };

    let mut file = File::create("save_data.json")?;

    file.write_all(serde_json::to_string(&save_data)?.as_bytes())?;

    let fd = format!("thumb_data={thumb_data}&calc_state={calc_state}&is_update={is_update}&lang={lang}&my_graphs={my_graphs}&graph_hash={graph_hash}", thumb_data=save_data.thumb_data, calc_state=save_data.calc_state, is_update=save_data.is_update, lang=save_data.lang, my_graphs=save_data.my_graphs, graph_hash=save_data.graph_hash);

    let client = reqwest::Client::new();

    let response = client
        .post("https://www.desmos.com/api/v1/calculator/save")
        .body(fd)
        .send()
        .await?;

    if response.status() != 200 {
        panic!(
            "Something went wrong when uploading the file.\nStatus code: {}\nResponse: {}",
            response.status(),
            response.text().await?
        );
    }

    Ok(())
}
