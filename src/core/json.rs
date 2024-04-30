use std::io::BufWriter;

use tokio::fs::File;
use tokio::io::{AsyncReadExt, BufReader};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {

}

async fn read_json<T: Deserialize<'static>>(file_path: &str) -> Result<T, Box<dyn std::error::Error>> {
    let file = File::open(file_path).await?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).await?;
    let config: T = serde_json::from_str(&contents)?;
    Ok(config)
}

async fn write_json<T: Serialize>(file_path: &str, data: T) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create(file_path).await?;
    let mut writer = BufWriter::new(file);
    let json = serde_json::to_string(&data)?;
    writer.write_all(json.as_bytes()).await?;
    Ok(())
}

async fn append_json<T: Serialize>(file_path: &str, data: T) -> Result<(), Box<dyn std::error::Error>> {
    let file = OpenOptions::new().append(true).open(file_path).await?;
    let mut writer = BufWriter::new(file);
    let json = serde_json::to_string(&data)?;
    writer.write_all(json.as_bytes()).await?;
    Ok(())
}