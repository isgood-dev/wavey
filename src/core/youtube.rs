use std::{collections::HashMap, path::Path};

use rustube::{Id, Video};

use crate::sql;

pub async fn download_from_url(url: String) -> bool {
    let id = Id::from_raw(&url).unwrap();

    let video = Video::from_id(id.into_owned()).await.unwrap();

    let download_path = Path::new("./assets/audio");

    let audio_info = video.best_audio().unwrap();
    let video_details = &video.best_audio().unwrap().video_details;

    let mut video_info = HashMap::new();

    video_info.insert("format_type".to_string(), audio_info.format_type.map_or_else(|| "None".to_string(),|ft| format!("{:?}", ft)));
    video_info.insert("video_id".to_string(), video_details.video_id.to_string());
    

    let _ = video
        .best_audio()
        .unwrap()
        .download_to_dir(download_path)
        .await;

    let _ = sql::add_music(video_info);

    true
}