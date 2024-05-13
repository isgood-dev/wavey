use std::{collections::HashMap, path::Path};
use tokio::fs;
use tokio::process::Command;

use rusty_ytdl::{
    search::{SearchResult, YouTube},
    Video, VideoOptions, VideoQuality, VideoSearchOptions,
};

use crate::sql;

async fn ffmpeg_convert_codec(video_id: String) -> bool {
    let in_file = format!("./assets/audio/{}.webm", video_id);
    let out_file = format!("./assets/audio/{}.mp3", video_id);

    let output = Command::new("./bin/ffmpeg")
        .args(&[
            "-i", &in_file, "-vn", "-ar", "44100", "-ac", "2", "-b:a", "192k", &out_file,
        ])
        .output()
        .await;

    if output.is_err() {
        return false;
    }

    fs::remove_file(in_file).await.unwrap();

    true
}

pub async fn get_search_results(query: String) -> Vec<HashMap<String, String>> {
    let youtube = YouTube::new().unwrap();

    let res = youtube.search(query, None).await.unwrap();

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

    results
}

pub async fn download_from_url(url: String) -> bool {
    let video_options = VideoOptions {
        quality: VideoQuality::HighestAudio,
        filter: VideoSearchOptions::Audio,
        ..Default::default()
    };

    let video = Video::new_with_options(url, video_options).unwrap();

    let video_info = video.get_info().await.unwrap();

    let path_str = format!(
        "./assets/audio/{}.webm",
        video_info.video_details.video_id.as_str()
    );
    let path = Path::new(&path_str);

    video.download(path).await.unwrap();

    let video_id = video_info.video_details.video_id;

    let mut to_store = HashMap::new();
    to_store.insert("format_type".to_string(), "mp3".to_string());
    to_store.insert("video_id".to_string(), video_id.clone());
    to_store.insert("display_name".to_string(), video_info.video_details.title);
    to_store.insert(
        "duration".to_string(),
        video_info.video_details.length_seconds.to_string(),
    );

    let _ = sql::add_music(to_store);

    ffmpeg_convert_codec(video_id).await;

    true
}
