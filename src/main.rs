use std::fs;
use std::path::PathBuf;

use clap::Parser;
use serde_json::json;

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

    let mut graph_hash = format!("{:x}", rand::random::<u128>())
        .chars()
        .take(10)
        .collect::<String>();

    if hash.is_some() {
        let hash = hash.as_ref().unwrap();

        if hash.len() != 10 {
            panic!("Hash must be 10 characters long.")
        }

        if hash.chars().any(|char| !char.is_ascii_alphanumeric()) {
            panic!("Hash must be an ASCII alphanumeric string.")
        }

        graph_hash = hash.to_string();
    }

    let data_url = format!("data:image/png;base64,{}", base64::encode(fs::read(&file)?));

    let calc_state = json!({
        "version": 9,
        "randomSeed": format!("{:x}", rand::random::<u128>()).chars().take(16).collect::<String>(),
        "graph": {
            "viewport": {
                "xmin": -170,
                "xmax": 170,
                "ymin": -170,
                "ymax": 170,
            },
        },
        "expressions": {
            "list": [{
                "type": "image",
                "image_url": data_url,
            }],
        },
    });

    let data = json!({
        "thumb_data": data_url,
        "calc_state": calc_state.to_string(),
        "is_update": "false",
        "lang": "en",
        "my_graphs": "false",
        "graph_hash": graph_hash,
    });

    let response = reqwest::Client::new()
        .post("https://www.desmos.com/api/v1/calculator/save")
        .json(&data)
        .send()
        .await?;

    if !response.status().is_success() {
        match response.status() {
            _ => panic!("An unknown error occurred. ({})", response.status())
        }
    }

    println!("https://www.desmos.com/calculator/{}", graph_hash);

    Ok(())
}
