use std::{
    collections::HashMap,
    env,
    fs::{self, ReadDir},
    io::Error,
    path::PathBuf,
};

use db::{DataBase, Track};

mod db;
mod reader;

// const PATH: &str = "D:/Music/HQ/FLAC";
const PATH: &str = "D:/Music/Test";

fn main() -> Result<(), Error> {
    env::set_var("RUST_BACKTRACE", "1");

    let db: DataBase = DataBase::init();

    scan_dir(&String::from(PATH), &db);

    check_tracks(&db);
    Ok(())
}

fn scan_dir(dir_path: &String, db: &DataBase) {
    let item_res: Result<ReadDir, Error> = fs::read_dir(dir_path);

    match item_res {
        Ok(items) => {
            for item in items {
                let path = item.unwrap().path();

                if path.is_dir() {
                    scan_dir(&path.display().to_string(), db);
                } else {
                    // println!("{}", path.display());
                    let track: Track =
                        read_metadata(path.display().to_string(), &dir_path.to_owned());
                    db.add_track_to_db(track);
                }
            }
        }
        Err(err) => println!("Error: {}\nCaused by Directory: {}", err, dir_path),
    }
}

fn read_metadata(file_path: String, directory: &str) -> Track {
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
    // println!("{:?}", track);
    track
}

fn check_tracks(db: &DataBase) {
    let ids = db.get_all_ids();

    for id in ids {
        let track = db.get_track_by_id(id).unwrap_or_else(Track::blank);
        println!("{:?}", track);
    }
}
