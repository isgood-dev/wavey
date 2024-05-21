use std::collections::HashMap;

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
