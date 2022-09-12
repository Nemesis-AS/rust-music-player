use id3::{Tag, TagLike};
use metaflac::Tag as FlacTag;
use std::{collections::HashMap, path::Path};

pub fn read(file_path: &String) -> HashMap<String, String> {
    let extension = Path::new(&file_path).extension().unwrap().to_str().unwrap();

    match extension {
        "flac" => read_from_flac(file_path),
        "mp3" => read_id3(file_path),
        _ => (HashMap::new()),
    }
}

// METAFLAC
fn read_from_flac(file_path: &String) -> HashMap<String, String> {
    let tags =
        FlacTag::read_from_path(file_path).expect("An Error Occured while reading tags from FLAC");

    const FLAC_TAGS_LIST: [[&str; 2]; 5] = [
        ["MUSICBRAINZ_TRACKID", "mb_id"],
        ["ARTIST", "artist"],
        ["TITLE", "title"],
        ["ALBUM", "album"],
        ["GENRE", "genre"],
    ];

    let mut track_meta: HashMap<String, String> = HashMap::new();

    for arr in FLAC_TAGS_LIST {
        let value = get_vorbis_or_str(&tags, arr[0]);
        track_meta.insert(String::from(arr[1]), value);
    }

    // println!("{:?}", track_meta);
    track_meta
}

// Utils
fn get_vorbis_or_str(tag: &FlacTag, key: &str) -> String {
    let item = tag.get_vorbis(key);

    match item {
        Some(mut iterator) => iterator.next().unwrap_or("").to_string(),
        None => String::from(""),
    }
}

// ID3
fn read_id3(file_path: &String) -> HashMap<String, String> {
    let tag = Tag::read_from_path(file_path).expect("An Error Occured while reading ID3 Tag");

    const ID3_TAGS_LIST: [[&str; 3]; 5] = [
        ["0", "MusicBrainz Release Track Id", "mb_id"],
        ["1", "TPE1", "artist"],
        ["1", "TIT2", "title"],
        ["1", "TALB", "album"],
        ["1", "genre", "genre"],
    ];

    let mut track_meta: HashMap<String, String> = HashMap::new();

    for arr in ID3_TAGS_LIST {
        let mut value: String = String::from("");
        match arr[0] {
            "0" => {
                value = get_id3_frame(&tag, arr[1]).unwrap_or_else(|| String::from(""));
            }
            "1" => {
                if let Some(item) = tag.get(arr[1]).and_then(|frame| frame.content().text()) {
                    if item.is_empty() {
                        value = String::from("");
                    } else {
                        value = String::from(item);
                    }
                }
            }
            _ => (),
        }
        track_meta.insert(String::from(arr[2]), value);
    }

    // println!("{:?}", track_meta);
    track_meta
}

fn get_id3_frame(tag: &Tag, key: &str) -> Option<String> {
    for text in tag.extended_texts() {
        if text.description == key {
            return Some(text.value.clone());
        }
    }
    None
}
