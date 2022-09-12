use std::{
    collections::HashMap,
    env,
    fs::{self, ReadDir},
    io::Error,
    path::PathBuf,
};

use db::Track;

mod db;
mod reader;

// const PATH: &str = "D:/Music/HQ/FLAC";
const PATH: &str = "D:/Music/Test";

fn main() -> Result<(), Error> {
    env::set_var("RUST_BACKTRACE", "1");

    scan_dir(&String::from(PATH));

    // let conn = db::init();

    // let new_track: Track = Track {
    //     id: 1,
    //     artist: String::from("TheFatRat"),
    //     title: String::from("Electrified"),
    //     album: String::from("Electrified"),
    //     genre: String::from("Electronic"),
    //     file: String::from("idk"),
    //     duration: 190,
    //     name: String::from("01 Electrified"),
    //     ext: String::from("mp3"),
    //     directory: String::from("D:/Music/Test"),
    // };

    // db::add_track_to_db(&conn, new_track);

    // let rows = db::get_track_by_id(&conn, 1);
    // db::get_track_by_id(&conn, 1);

    // for row in rows {
    //     println!("Track: {:?}", row.unwrap());
    // }

    Ok(())
}

fn scan_dir(dir_path: &String) {
    let item_res: Result<ReadDir, Error> = fs::read_dir(dir_path);

    match item_res {
        Ok(items) => {
            for item in items {
                let path = item.unwrap().path();

                if path.is_dir() {
                    scan_dir(&path.display().to_string());
                } else {
                    // println!("{}", path.display());
                    read_metadata(path.display().to_string(), &dir_path.to_owned());
                }
            }
        }
        Err(err) => println!("Error: {}\nCaused by Directory: {}", err, dir_path),
    }
}

fn read_metadata(file_path: String, directory: &str) {
    let mut metadata: HashMap<String, String> = reader::read(&file_path);

    let path: PathBuf = PathBuf::from(&file_path);

    metadata.insert(
        String::from("name"),
        path.file_name().unwrap().to_str().unwrap_or("").to_string(),
    );

    metadata.insert(
        String::from("ext"),
        path.extension().unwrap().to_str().unwrap_or("").to_string(),
    );

    metadata.insert(String::from("directory"), String::from(directory));

    let track: Track = Track::from_hashmap(metadata);
    println!("{:?}", track);
}
