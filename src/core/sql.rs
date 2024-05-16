use std::{collections::HashMap, path::Path};

use rusqlite::{Connection, Error as RusqliteError};

#[derive(Debug)]
pub enum DatabaseError {
    SqliteError(RusqliteError),
}

impl From<RusqliteError> for DatabaseError {
    fn from(error: RusqliteError) -> Self {
        DatabaseError::SqliteError(error)
    }
}

#[derive(Debug)]
struct Music {
    music_id: i32,
    video_id: String,
    extension: String,
    duration: i32,
    display_name: String,
}

// #[derive(Debug)]
// struct Playlist {
//     playlist_id: i32,
//     name: String,
// }

// #[derive(Debug)]
// struct MusicPlaylist {
//     music_playlist_id: i32,
//     music_id: i32,
//     playlist_id: i32,
// }


// Pretty self-explanatory. Checks if the database file exists.
pub fn check_database_exists() -> bool {
    Path::new("./assets/data.db").exists()
}

// Creates the database tables. Called on startup if the database doesn't already exist.
pub fn create_database_tables() -> Result<(), DatabaseError> {
    let conn = Connection::open("./assets/data.db")?;

    conn.execute(
        "CREATE TABLE music (
            music_id INTEGER PRIMARY KEY NOT NULL,
            video_id TEXT NOT NULL,
            extension TEXT NOT NULL,
            duration INTEGER,
            display_name TEXT
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE playlists (
            playlist_id INTEGER PRIMARY KEY NOT NULL,
            name TEXT NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE music_playlists (
            music_playlists INTEGER PRIMARY KEY NOT NULL,
            music_id INTEGER REFERENCES music(music_id),
            playlist_id INTEGER REFERENCES playlists(playlist_id)
        )",
        [],
    )?;

    Ok(())
}

// Adds a track to the `music` table in the databsae. This is called when
// downloading/importing new audio tracks.
pub fn add_music(video_data: HashMap<String, String>) -> Result<(), DatabaseError> {
    let conn = Connection::open("./assets/data.db")?;

    let video_id = video_data.get("video_id").unwrap();
    let extension = video_data.get("format_type").unwrap();
    let display_name = video_data.get("display_name").unwrap();
    let duration = video_data.get("duration").unwrap();

    conn.execute(
        "INSERT INTO music (video_id, extension, display_name, duration)
        VALUES (?1, ?2, ?3, ?4)",
        [&video_id, &extension, &display_name, &duration],
    )?;

    Ok(())
}

// Gets an audio track from the database. This is called when requesting to
// play a song, as certain information is needed to be passed onto other widgets,
// such as `duration` and `display_name` for the control bar.
pub fn get_music(video_id: String) -> HashMap<String, String> {
    let conn = Connection::open("./assets/data.db").unwrap();

    let mut statement = conn
        .prepare("SELECT * FROM music WHERE video_id = ?1")
        .unwrap();
    let music_iter = statement
        .query_map(&[&video_id], |row| {
            Ok(Music {
                music_id: row.get(0)?,
                video_id: row.get(1)?,
                extension: row.get(2)?,
                duration: row.get(3)?,
                display_name: row.get(4)?,
            })
        })
        .unwrap();

    let mut music_data = HashMap::new();

    for music in music_iter {
        let music = music.unwrap();
        music_data.insert("music_id".to_string(), music.music_id.to_string());
        music_data.insert("video_id".to_string(), music.video_id);
        music_data.insert("extension".to_string(), music.extension);
        music_data.insert("duration".to_string(), music.duration.to_string());
        music_data.insert("display_name".to_string(), music.display_name);
    }

    music_data
}

// Verifies the integrity of the audio tracks in the database by comparing all
// tracks in the database to the audio files.
// If the audio track is in the database but the corresponding audio track does NOT
// exist, it will be deleted from the database.
// This is called on app startup and is not checked again.
pub fn verify_data_integrity() -> Result<(), DatabaseError> {
    let conn = Connection::open("./assets/data.db")?;

    let mut statement = conn.prepare("SELECT * FROM music").unwrap();
    let music_iter = statement
        .query_map([], |row| {
            Ok(Music {
                music_id: row.get(0)?,
                video_id: row.get(1)?,
                extension: row.get(2)?,
                duration: row.get(3)?,
                display_name: row.get(4)?,
            })
        })
        .unwrap();

    for music in music_iter {
        let music = music.unwrap();
        let path_str = format!("./assets/audio/{}.{}", music.video_id, music.extension);
        let path = Path::new(&path_str);

        if !path.exists() {
            conn.execute("DELETE FROM music WHERE music_id = ?1", [music.music_id])?;
        }
    }

    Ok(())
}

// Gets all audio tracks from the database. This is called to be displayed on
// the `track_list` for displaying all songs.
pub fn get_all_music() -> Vec<HashMap<String, String>> {
    let conn = Connection::open("./assets/data.db").unwrap();

    let mut statement = conn.prepare("SELECT * FROM music").unwrap();
    let music_iter = statement
        .query_map([], |row| {
            Ok(Music {
                music_id: row.get(0)?,
                video_id: row.get(1)?,
                extension: row.get(2)?,
                duration: row.get(3)?,
                display_name: row.get(4)?,
            })
        })
        .unwrap();

    let mut music_data = Vec::new();

    for music_item in music_iter {
        let music_item = music_item.unwrap();
        let mut music_map = HashMap::new();
        music_map.insert("music_id".to_string(), music_item.music_id.to_string());
        music_map.insert("video_id".to_string(), music_item.video_id);
        music_map.insert("extension".to_string(), music_item.extension);
        music_map.insert("duration".to_string(), music_item.duration.to_string());
        music_map.insert("display_name".to_string(), music_item.display_name);
        music_data.push(music_map);
    }

    music_data
}
