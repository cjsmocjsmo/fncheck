// use std::env;
use std::path::Path;
use walkdir::WalkDir;

// pub mod utils;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // if args.len() < 1 {
    //     println!("Please provide a directory path as an argument.");
    //     return;
    // }

    // let dir_path = &args[1];
    let dir_path = "/media/pinas/foo1/Music/Music".to_string();
    // if !Path::new(&dir_path).is_dir() {
    //     println!("The provided path is not a directory.");
    //     return;
    // }

    let mp3_files = find_media(&dir_path);
    let mut totalcount = 0;
    // let mut badcount = 0;

    for mp3 in &mp3_files {
        totalcount += 1;

        // let _ = clean_filename(mp3.to_string());
        
        let _check = check_file_name_format(mp3.to_string());
        // if !check {
        //     // println!("Filename format is incorrect:\n\t {:?}", mp3);
        //     badcount += 1;
        // }
    }
    // println!("\n\nFound {} files formatted incorrectly.", badcount);
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

pub fn clean_filename(apath: String) -> String {
    let file_parts = split_path(&apath).unwrap();
    let file_name1 = file_parts.1;
    let file_name2 = remove_parentheses_and_contents(&file_name1);
    let file_name3 = file_name2.replace("&", "And")
        .replace("+", "And")
        .replace(" .mp3", ".mp3")
        .replace("'", "")
        .replace(",", "")
        .replace(". ", "_-_")
        .replace(" ", "_");
    let file_name4 = file_parts.0 + "/" + &file_name3;
    let file_name = file_name4.replace(" ", "_");

    let _ = rename_file(apath, file_name.clone());
    // println!("File name: {:?}", file_name);



    file_name
}

pub fn rename_file(oldpath: String, newpath: String) {
    if let Err(e) = std::fs::rename(&oldpath, newpath) {
        println!("Failed to rename file: {}. Error: {}", oldpath, e);
    }
}

pub fn check_file_name_format(apath: String) {
    let file_parts = split_path(&apath).unwrap();
    let file_name = file_parts.1;
    let file_name = remove_parentheses_and_contents(&file_name);
    // println!("File name: {:?}", file_name);
    let re = regex::Regex::new(r"\d_\d{2}_-_.+_-_.+_-_.+\.mp3").unwrap();
    let re2 = regex::Regex::new(r"\d{2}_-_.+_-_.+_-_.+\.flac").unwrap();
    let re3 = regex::Regex::new(r"\d_d{2}_-_.+_-_.+_-_.+\.ogg").unwrap();
    let re4 = regex::Regex::new(r"\d_d{2}_-_.+_-_.+_-_.+\.wav").unwrap();
    let re5 = regex::Regex::new(r"^\d{2}\.\s[1-9A-Za-z\s]+\s-\s[1-9A-Za-z\s]+\.mp3$").unwrap();
    let re6 = regex::Regex::new(r"^\d_\d{2}_\-\_[1-9A-Za-z\s]+_\-\_[1-9A-Za-z\s]+\-\_[1-9A-Za-z\s]+\.mp3$").unwrap();
    // let file_name = Path::new(&apath)
    //     .file_name()
    //     .unwrap()
    //     .to_str()
    //     .unwrap();+

    let mut remat = 0;
    let mut re2mat = 0;
    let mut re3mat = 0;
    let mut re4mat = 0;
    let mut re5mat = 0;
    let mut re6mat = 0;

    if re.is_match(&file_name) {
        remat += 1;
        // println!("Matched re: {:?}", file_name);
        
    } else if re2.is_match(&file_name) {
        re2mat += 1;
        
    } else if re3.is_match(&file_name) {
        re3mat += 1;
        
    } else if re4.is_match(&file_name) {
        re4mat += 1;
        
    } else if re5.is_match(&file_name) {
        re5mat += 1;
        println!("Matched re5: {:?}", file_name);
        
    } else if re6.is_match(&file_name) {
        re6mat += 1;
        println!("Matched re6: {:?}", file_name);
        
    } else {
        println!("fuck: {:?}", file_name);
    }

    println!("Matched re: {:?}", remat);
    println!("Matched re2: {:?}", re2mat);
    println!("Matched re3: {:?}", re3mat);
    println!("Matched re4: {:?}", re4mat);
    println!("Matched re5: {:?}", re5mat);
    println!("Matched re6: {:?}", re6mat);
}

fn split_path(path_str: &str) -> Option<(String, String)> {
    let path = Path::new(path_str);

    let dir = path
        .parent()
        .map(|p| p.to_str().unwrap_or(""))
        .unwrap_or("."); // Handle root paths

    let file = path
        .file_name()
        .map(|f| f.to_str().unwrap_or(""))
        .unwrap_or("");

    Some((dir.to_string(), file.to_string()))
}

fn remove_parentheses_and_contents(input: &str) -> String {
    // Define the regular expression to match parentheses and their contents
    let re = regex::Regex::new(r"\([^)]*\)").unwrap(); 

    // Replace all matches with an empty string
    re.replace_all(input, "").to_string()
}