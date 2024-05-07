use std::{collections::HashMap, path::Path};
use tokio::fs;
use tokio::process::Command;

use rusty_ytdl::{Video, VideoOptions, VideoQuality, VideoSearchOptions};

use crate::sql;

pub async fn ffmpeg_convert_codec(video_id: String) -> bool {
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
