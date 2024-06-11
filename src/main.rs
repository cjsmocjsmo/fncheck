use std::env;
use std::path::Path;
use walkdir::WalkDir;

// pub mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 1 {
        println!("Please provide a directory path as an argument.");
        return;
    }

    let dir_path = &args[1];
    if !Path::new(dir_path).is_dir() {
        println!("The provided path is not a directory.");
        return;
    }

    let mp3_files = find_media(&dir_path);
    let mut totalcount = 0;
    let mut badcount = 0;

    for mp3 in &mp3_files {
        totalcount += 1;
        
        let check = check_file_name_format(mp3.to_string());
        if !check {
            println!("Filename format is incorrect:\n\t {:?}", mp3);
            badcount += 1;
        }
    }
    println!("\n\nFound {} files formatted incorrectly.", badcount);
    println!("Scanned {} media files.", totalcount);
}


pub fn find_media(dir_path: &String) -> Vec<String> {
    println!("Dir path: {:?}", dir_path);
    let mut media_files = Vec::new();
    for entry in WalkDir::new(dir_path) {
        let entry = entry.unwrap();
        if entry.path().extension().map_or(false, |ext| {
            ext == "mp3"
                || ext == "MP3"
                || ext == "flac"
                || ext == "FLAC"
                || ext == "ogg"
                || ext == "OGG"
                || ext == "wav"
                || ext == "WAV"
        }) {
            media_files.push(entry.path().to_string_lossy().into_owned());
        }
    }

    media_files
}

pub fn check_file_name_format(apath: String) -> bool {
    let re = regex::Regex::new(r"\d+_\d\d+_-_.+_-_.+_-_.+\.mp3").unwrap();
    let re2 = regex::Regex::new(r"\d+_\d\d+_-_.+_-_.+_-_.+\.flac").unwrap();
    let re3 = regex::Regex::new(r"\d+_\d\d+_-_.+_-_.+_-_.+\.ogg").unwrap();
    let re4 = regex::Regex::new(r"\d+_\d\d+_-_.+_-_.+_-_.+\.wav").unwrap();
    let file_name = Path::new(&apath)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();

    if re.is_match(file_name) {
        return true;
    } else if re2.is_match(file_name) {
        return true;
    } else if re3.is_match(file_name) {
        return true;
    } else if re4.is_match(file_name) {
        return true;
    } else {
        return false;
    }
}