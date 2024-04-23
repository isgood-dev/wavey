use std::path::Path;

use rustube::{Id, Video};

pub async fn download_video(url: &str) -> bool {
    println!("1");
    
    let id = Id::from_raw(&url).unwrap();

    let video = Video::from_id(id.into_owned()).await.unwrap();

    let download_path = Path::new("./assets/audio");

    let _ = video
        .best_audio()
        .unwrap()
        .download_to_dir(download_path)
        .await;

    println!("2");

    true
}