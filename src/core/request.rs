use std::collections::HashMap;
use std::hash::Hash;
use std::path::PathBuf;

use iced::advanced::image::Bytes;
use iced::subscription;

use log::info;
use reqwest::Client;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

#[derive(Debug, Clone, PartialEq)]
pub enum RequestError {
    RequestError,
}

pub async fn request_thumbnail(url: String) -> Result<Bytes, reqwest::Error> {
    info!("Requesting thumbnail from {}", url);

    let client = Client::new();

    let response = client.get(&url).send().await?;

    let bytes = response.bytes().await?;

    Ok(bytes)
}

pub async fn request_all_thumbnails(
    results: Vec<HashMap<String, String>>,
) -> Result<Vec<Vec<u8>>, RequestError> {
    let mut thumbnails = Vec::new();

    for result in results {
        let url = result.get("thumbnail").unwrap().clone();

        let bytes = request_thumbnail(url)
            .await
            .map_err(|_| RequestError::RequestError)?;

        thumbnails.push(bytes.to_vec());
    }

    info!("Thumbnails received.");

    Ok(thumbnails)
}

pub async fn request_thumbnails(
    video_ids: Vec<String>,
) -> Vec<HashMap<String, iced::advanced::image::Handle>> {
    let mut handles = Vec::new();

    for video_id in video_ids {
        let mut dir = tokio::fs::read_dir("./data/thumbnails")
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
                let bytes = tokio::fs::read(PathBuf::from("./data/thumbnails/default.jpg"))
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

pub fn download_file<I: 'static + Hash + Copy + Send + Sync, T: ToString>(
    id: I,
    url: T,
) -> iced::Subscription<(I, Progress)> {
    subscription::unfold(id, State::Ready(url.to_string()), move |state| {
        download_progress(id, state)
    })
}

async fn download_progress<I: Copy>(id: I, state: State) -> ((I, Progress), State) {
    match state {
        State::Ready(url) => {
            let response = reqwest::get(&url).await;

            match response {
                Ok(response) => {
                    if let Some(total) = response.content_length() {
                        (
                            (id, Progress::Started),
                            State::Downloading {
                                response,
                                total,
                                downloaded: 0,
                            },
                        )
                    } else {
                        ((id, Progress::Errored), State::Finished)
                    }
                }
                Err(_) => ((id, Progress::Errored), State::Finished),
            }
        }
        State::Downloading {
            mut response,
            total,
            downloaded,
        } => match response.chunk().await {
            Ok(Some(chunk)) => {
                let downloaded = downloaded + chunk.len() as u64;

                let percentage = (downloaded as f32 / total as f32) * 100.0;

                let file = OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open("./data/ffmpeg.exe")
                    .await;

                match file {
                    Ok(mut file) => {
                        // Write the chunk to the file
                        file.write_all(&chunk)
                            .await
                            .expect("Failed to write to file");
                    }
                    Err(e) => {
                        eprintln!("Failed to open file: {}", e);
                    }
                }

                (
                    (id, Progress::Advanced(percentage)),
                    State::Downloading {
                        response,
                        total,
                        downloaded,
                    },
                )
            }
            Ok(None) => ((id, Progress::Finished), State::Finished),
            Err(_) => ((id, Progress::Errored), State::Finished),
        },
        State::Finished => iced::futures::future::pending().await,
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Progress {
    Started,
    Advanced(f32),
    Finished,
    Errored,
}

pub enum State {
    Ready(String),
    Downloading {
        response: reqwest::Response,
        total: u64,
        downloaded: u64,
    },
    Finished,
}
