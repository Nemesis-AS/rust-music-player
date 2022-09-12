use std::collections::HashMap;

use rusqlite::Connection;

#[derive(Debug)]
pub struct Track {
    pub id: String,
    pub artist: String,
    pub title: String,
    pub album: String,
    pub genre: String,
    pub file: String,
    pub duration: i32,
    pub name: String,
    pub ext: String,
    pub directory: String,
    pub last_modified: String,
}

impl Track {
    pub fn from_hashmap(hashmap: HashMap<String, String>) -> Self {
        let null_str: String = String::from("");
        Self {
            id: hashmap.get("mb_id").unwrap_or(&null_str).clone(),
            artist: hashmap.get("artist").unwrap_or(&null_str).clone(),
            title: hashmap.get("title").unwrap_or(&null_str).clone(),
            album: hashmap.get("album").unwrap_or(&null_str).clone(),
            genre: hashmap.get("genre").unwrap_or(&null_str).clone(),
            file: String::from(""),
            duration: 0,
            name: hashmap.get("name").unwrap_or(&null_str).clone(),
            ext: hashmap.get("ext").unwrap_or(&null_str).clone(),
            directory: hashmap.get("directory").unwrap_or(&null_str).clone(),
            last_modified: hashmap.get("last_modified").unwrap_or(&null_str).clone(),
        }
    }
}

pub fn init() -> Connection {
    let conn = Connection::open_in_memory().expect("Couldn't Open DB Connection");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tracks (
            id TEXT PRIMARY KEY,
            artist TEXT,
            title TEXT,
            album TEXT,
            genre TEXT,
            file TEXT,
            duration INTEGER,
            name TEXT,
            ext TEXT,
            directory TEXT,
            last_modified TEXT
        )",
        (),
    )
    .expect("Couldn't Create Table track");

    conn
}

pub fn add_track_to_db(conn: &Connection, track: Track) -> () {
    conn.execute(
        "INSERT INTO tracks VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        (
            &track.id,
            &track.artist,
            &track.title,
            &track.album,
            &track.genre,
            &track.file,
            &track.duration,
            &track.name,
            &track.ext,
            &track.directory,
        ),
    )
    .expect("An Error Occured while inserting record in Table 'track'");
}

pub fn get_track_by_id(conn: &Connection, id: i32) {
    let mut stmt = conn
        .prepare("SELECT * FROM tracks WHERE id = ?")
        .expect("Could not prepare SQL Statement");
    let rows = stmt
        .query_map([id], |row| {
            Ok(Track {
                id: row.get(0).expect("Couldn't fetch value from row"),
                artist: row.get(1).expect("Couldn't fetch value from row"),
                title: row.get(2).expect("Couldn't fetch value from row"),
                album: row.get(3).expect("Couldn't fetch value from row"),
                genre: row.get(4).expect("Couldn't fetch value from row"),
                file: row.get(5).expect("Couldn't fetch value from row"),
                duration: row.get(6).expect("Couldn't fetch value from row"),
                name: row.get(7).expect("Couldn't fetch value from row"),
                ext: row.get(8).expect("Couldn't fetch value from row"),
                directory: row.get(9).expect("Couldn't fetch value from row"),
                last_modified: row.get(10).expect("Coudn't fetch value from row"),
            })
        })
        .expect("Error while running SQL Statement");

    for row in rows {
        println!("Tracks: {:?}", row.unwrap());
    }
}

// fn get_value_or_err(item: Result<T, E>) -> String {
//     match item {
//         Ok(value) => value.to_string(),
//         Err(err) => err.to_string(),
//     }
// }