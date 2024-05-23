use std::{collections::HashMap, path::PathBuf};

use iced::advanced::image::Bytes;

use log::info;
use reqwest::Client;

pub async fn request_thumbnail(url: String) -> Result<Bytes, reqwest::Error> {
    info!("Requesting thumbnail from {}", url);

    let client = Client::new();

    let response = client.get(&url).send().await?;

    let bytes = response.bytes().await?;

    Ok(bytes)
}

pub async fn request_all_thumbnails(results: Vec<HashMap<String, String>>) -> Vec<Vec<u8>> {
    let mut thumbnails = Vec::new();

    for result in results {
        let url = result.get("thumbnail").unwrap().clone();

        let bytes = request_thumbnail(url).await.unwrap();

        thumbnails.push(bytes.to_vec());
    }

    info!("Thumbnails received.");

    thumbnails
}

pub async fn request_thumbnails(
    video_ids: Vec<String>,
) -> Vec<HashMap<String, iced::advanced::image::Handle>> {
    let mut handles = Vec::new();

    for video_id in video_ids {
        let mut dir = tokio::fs::read_dir("./assets/thumbnails")
            .await
            .expect("Failed to read");

        while let Ok(Some(entry)) = dir.next_entry().await {
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_str().unwrap().to_string();

            if file_name.contains(&video_id) {
                let bytes = tokio::fs::read(path).await.expect("Failed to read file");
                let handle = iced::advanced::image::Handle::from_bytes(bytes);

                let mut map = HashMap::new();
                map.insert(video_id.clone(), handle);

                handles.push(map);
            } else {
                let bytes = tokio::fs::read(PathBuf::from("./assets/thumbnails/default.jpg"))
                    .await
                    .expect("Failed to read file");
                let handle = iced::advanced::image::Handle::from_bytes(bytes);

                let mut map = HashMap::new();
                map.insert(video_id.clone(), handle);
            }
        }
    }

    handles
}
