use std::collections::HashMap;
use std::path::Path;

use super::request;
use crate::db;

use tokio::fs;
use tokio::process::Command;

use rusty_ytdl::search::{SearchResult, YouTube};
use rusty_ytdl::{Video, VideoOptions, VideoQuality, VideoSearchOptions};

#[derive(Debug, Clone, PartialEq)]
pub enum StatusError {
    NetworkError,
    VideoNotFound,
    UnknownError,
    FFmpegConversionError,
    VideoOptionError,
    VideoInfoError,
    DownloadError,
    CodecError,
    ThumbnailError,
    WriteError,
}

// Simply calls ffmpeg to convert audio files from `webm` format to `mp3` format.
// YouTube does not store files in `mp3` format, so after downloading from YouTube,
// we need to use FFmpeg to convert to `mp3` codec.
// Alternatives would be nice to avoid using FFmpeg since it's a large dependancy.
async fn ffmpeg_convert_codec(video_id: String) -> Result<bool, StatusError> {
    let in_file = format!("./data/audio/{}.webm", video_id);
    let out_file = format!("./data/audio/{}.mp3", video_id);

    let cmd_dest = if cfg!(unix) {
        String::from("ffmpeg")
    } else {
        String::from("./data/ffmpeg")
    };

    let output = Command::new(cmd_dest)
        .args(&[
            "-i", &in_file, "-vn", "-ar", "44100", "-ac", "2", "-b:a", "192k", &out_file,
        ])
        .output()
        .await;

    fs::remove_file(in_file).await.unwrap();

    match output {
        Ok(output) => Ok(output.status.success()),
        Err(_) => Err(StatusError::FFmpegConversionError),
    }
}

pub async fn get_search_results(
    query: String,
) -> Result<Vec<HashMap<String, String>>, StatusError> {
    let youtube = YouTube::new().unwrap();

    let res = match youtube.search(query, None).await {
        Ok(res) => res,
        Err(error) => match error {
            rusty_ytdl::VideoError::Reqwest(_) => return Err(StatusError::NetworkError),
            rusty_ytdl::VideoError::VideoNotFound => {
                return Err(StatusError::VideoNotFound);
            }
            _ => {
                return Err(StatusError::UnknownError);
            }
        },
    };

    let mut results = Vec::new();
    let mut index = 0;

    for video in res {
        if index > 6 {
            break;
        }

        if let SearchResult::Video(video) = video {
            let mut result = HashMap::new();
            result.insert("title".to_string(), video.title);
            result.insert("thumbnail".to_string(), video.thumbnails[0].url.clone());
            result.insert("channel".to_string(), video.channel.name);
            result.insert("video_id".to_string(), video.id);
            results.push(result);

            index += 1;
        }
    }

    log::info!("Results: {:?}", results);

    Ok(results)
}

pub async fn download_from_url(url: String) -> Result<(), StatusError> {
    let video_options = VideoOptions {
        quality: VideoQuality::HighestAudio,
        filter: VideoSearchOptions::Audio,
        ..Default::default()
    };

    let video =
        Video::new_with_options(url, video_options).map_err(|_| StatusError::VideoOptionError)?;

    let video_info = video
        .get_info()
        .await
        .map_err(|_| StatusError::VideoInfoError)?;

    let path_str = format!(
        "./data/audio/{}.webm",
        video_info.video_details.video_id.as_str()
    );
    let path = Path::new(&path_str);

    video
        .download(path)
        .await
        .map_err(|_| StatusError::DownloadError)?;

    let video_id = video_info.video_details.video_id;

    let mut to_store = HashMap::new();
    to_store.insert("format_type".to_string(), "mp3".to_string());
    to_store.insert("video_id".to_string(), video_id.clone());
    to_store.insert("display_name".to_string(), video_info.video_details.title);
    to_store.insert(
        "duration".to_string(),
        video_info.video_details.length_seconds.to_string(),
    );

    let _ = db::add_music(to_store);

    ffmpeg_convert_codec(video_id.clone())
        .await
        .map_err(|_| StatusError::CodecError)?;

    let thumbnail = &video_info.video_details.thumbnails[0].url;

    let downloaded = request::request_thumbnail(thumbnail.clone())
        .await
        .map_err(|_| StatusError::ThumbnailError)?;

    let thumbnail_path = format!("./data/thumbnails/{}.jpg", video_id);
    fs::write(thumbnail_path, downloaded)
        .await
        .map_err(|_| StatusError::WriteError)?;

    Ok(())
}
