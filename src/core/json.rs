use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;

use log::info;

fn read_file() -> Result<HashMap<String, String>, std::io::Error> {
    let file = File::open("./assets/settings.json")?;
    let reader = BufReader::new(file);
    let data: HashMap<String, String> = serde_json::from_reader(reader)?;

    info!("Read settings file.");

    Ok(data)
}

pub fn check_exists() -> bool {
    Path::new("./assets/settings.json").exists()
}

pub fn create_file() -> Result<(), std::io::Error> {
    let data = r#"{
    "theme": "Dark",
    "rpc_enabled": false
}"#;

    match File::create("./assets/settings.json") {
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

    let mut file = File::create("./assets/settings.json")?;
    file.write_all(new_data.as_bytes())?;

    Ok(())
}

pub async fn get_rpc_enabled() -> Result<bool, std::io::Error> {
    let data = read_file()?;
    let rpc_enabled = data.get("rpc_enabled").unwrap().parse::<bool>().unwrap();
    Ok(rpc_enabled)
}
