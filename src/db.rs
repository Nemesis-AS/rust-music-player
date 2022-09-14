use std::collections::HashMap;

use rusqlite::Connection;

pub struct DataBase {
    conn: Connection,
}

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

    pub fn blank() -> Self {
        Self {
            id: String::from(""),
            artist: String::from(""),
            title: String::from(""),
            album: String::from(""),
            genre: String::from(""),
            file: String::from(""),
            duration: 0,
            name: String::from(""),
            ext: String::from(""),
            directory: String::from(""),
            last_modified: String::from(""),
        }
    }
}

impl DataBase {
    pub fn init() -> Self {
        let conn = Connection::open("library.db").expect("Couldn't Open DB Connection");

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

        Self { conn }
    }

    pub fn add_track_to_db(&self, track: Track) {
        self.conn
            .execute(
                "INSERT OR IGNORE INTO tracks VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
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
                    &track.last_modified,
                ),
            )
            .expect("An Error Occured while inserting record in Table 'track'");
    }

    pub fn get_all_ids(&self) -> Vec<String> {
        let mut stmt = self.conn.prepare("SELECT * FROM tracks").unwrap();
        let output: Vec<String> = stmt
            .query_map([], |row| Ok(row.get(0).unwrap()))
            .unwrap()
            .flatten()
            .collect();

        output
    }

    // @todo Add None Case
    pub fn get_track_by_id(&self, id: String) -> Option<Track> {
        let res = self.conn.prepare("SELECT * FROM tracks WHERE id = ?");

        if let Ok(mut stmt) = res {
            let row = stmt
                .query_row([id], |row| {
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
                .unwrap_or_else(|_err| Track::blank());

            return Some(row);
        }
        None
    }
}
