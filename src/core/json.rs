use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;

use log;

fn read_file() -> Result<HashMap<String, String>, std::io::Error> {
    let file = File::open("./data/settings.json")?;
    let reader = BufReader::new(file);
    let data: HashMap<String, String> = serde_json::from_reader(reader)?;

    log::info!("Read settings file.");

    Ok(data)
}

pub fn check_exists() -> bool {
    Path::new("./data/settings.json").exists()
}

pub fn create_file() -> Result<(), std::io::Error> {
    let data = r#"{
    "theme": "Dark",
    "rpc_enabled": "false",
    "ffmpeg_path": ""
}"#;

    match File::create("./data/settings.json") {
        Ok(f) => {
            let mut file = f;
            file.write_all(data.as_bytes())?;
        }
        Err(e) => {
            return Err(e);
        }
    }

    Ok(())
}

pub fn get_theme() -> Result<String, std::io::Error> {
    let data = read_file()?;
    let theme = data.get("theme").unwrap().to_string();
    Ok(theme)
}

pub fn set_theme(theme: &str) -> Result<(), std::io::Error> {
    let data = read_file()?;
    let mut new_data = data.clone();
    new_data.insert("theme".to_string(), theme.to_string());

    let new_data = serde_json::to_string_pretty(&new_data).unwrap();

    let mut file = File::create("./data/settings.json")?;
    file.write_all(new_data.as_bytes())?;

    Ok(())
}

pub fn get_rpc_enabled() -> Result<bool, std::io::Error> {
    let data = read_file()?;
    let rpc_enabled = data.get("rpc_enabled").unwrap().parse::<bool>().unwrap();
    Ok(rpc_enabled)
}

pub fn set_rpc_enabled(enabled: bool) -> Result<(), std::io::Error> {
    let data = read_file()?;
    let mut new_data = data.clone();
    new_data.insert("rpc_enabled".to_string(), enabled.to_string());

    let new_data = serde_json::to_string_pretty(&new_data).unwrap();

    let mut file = File::create("./data/settings.json")?;
    file.write_all(new_data.as_bytes())?;

    Ok(())
}

pub fn get_ffmpeg_path() -> Result<String, std::io::Error> {
    let data = read_file()?;
    let ffmpeg_path = data.get("ffmpeg_path").unwrap().to_string();
    Ok(ffmpeg_path)
}

pub fn set_ffmpeg_path(path: &str) -> Result<(), std::io::Error> {
    let data = read_file()?;
    let mut new_data = data.clone();
    new_data.insert("ffmpeg_path".to_string(), path.to_string());

    let new_data = serde_json::to_string_pretty(&new_data).unwrap();

    let mut file = File::create("./data/settings.json")?;
    file.write_all(new_data.as_bytes())?;

    Ok(())
}
