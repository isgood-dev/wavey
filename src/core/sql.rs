use async_rusqlite::Connection;

#[derive(Debug, Clone)]
pub enum DatabaseError {
    
}



pub async fn create_database_tables() -> Result<bool, DatabaseError> {
    let conn = Connection::open("./assets")
        .await
        .expect("Failed to create audio DB.");

    let _ = conn
        .call(|conn| {
            conn.execute(
            "CREATE TABLE music (
                    music_id INTEGER PRIMARY KEY
                    video_id TEXT NOT NULL,
                    extension TEXT NOT NULL,
                    display_name TEXT
                )

                CREATE TABLE playlists (
                    playlist_id INTEGER PRIMARY KEY,
                    name TEXT NOT NULL
                )
                
                CREATE TABLE music_playlists (
                    music_id INTEGER FOREIGN KEY REFERENCES music(music_id),
                    playlist_id INTEGER FOREIGN KEY REFERENCES playlists(playlist_id)
                )
                ",
        (),
            )
        })
        .await;

    

    Ok(true)
}
