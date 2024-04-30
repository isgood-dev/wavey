use std::fs;

pub fn get_audio_files() -> Vec<String> {
    let mut files = vec![];
    let paths = fs::read_dir("./assets/audio").unwrap();

    for path in paths {
        let path = path.unwrap().path();
        let path = path.to_str().unwrap().to_string();
        files.push(path);
    }

    files
}