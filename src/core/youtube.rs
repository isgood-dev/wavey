use std::{collections::HashMap, path::Path};

use rusty_ytdl::{Video, VideoOptions, VideoQuality, VideoSearchOptions};

use crate::sql;

pub async fn download_from_url(url: String) -> bool {
    let video_options = VideoOptions {
        quality: VideoQuality::HighestAudio,
        filter: VideoSearchOptions::Audio,
        ..Default::default()
    };

    let video = Video::new_with_options(url, video_options).unwrap();
    
    let video_info = video.get_info().await.unwrap();

    let path_str = format!("./assets/audio/{}.mp3", video_info.video_details.video_id.as_str());
    let path = Path::new(&path_str);

    video.download(path).await.unwrap();

    let mut to_store = HashMap::new();
    to_store.insert("format_type".to_string(), "mp3".to_string());
    to_store.insert("video_id".to_string(), video_info.video_details.video_id);
    to_store.insert("display_name".to_string(), video_info.video_details.title);
    
    let _ = sql::add_music(to_store);
    
    true
}