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

pub fn check_database_exists() -> bool {
    Path::new("./assets/data.db").exists()
}

pub fn create_database_tables() -> Result<(), DatabaseError> {
    let conn = Connection::open("./assets/data.db")?;

    conn.execute(
        "CREATE TABLE music (
            music_id INTEGER PRIMARY KEY NOT NULL,
            video_id TEXT NOT NULL,
            extension TEXT NOT NULL,
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

pub fn add_music(video_data: HashMap<String, String>) -> Result<(), DatabaseError> {
    let conn = Connection::open("./assets/data.db")?;

    let video_id = video_data.get("video_id").unwrap();
    let extension = video_data.get("format_type").unwrap();

    conn.execute(
        "INSERT INTO music (video_id, extension)
        VALUES (?1, ?2)",
        [&video_id, &extension],
    )?;

    Ok(())
}