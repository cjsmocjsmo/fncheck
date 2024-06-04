use std::env;
use std::path::Path;

pub mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a directory path as an argument.");
        return;
    }
    let dir_path = &args[1];
    if !Path::new(dir_path).is_dir() {
        println!("The provided path is not a directory.");
        return;
    }
    
    let mp3_files = utils::find_mp3s(&dir_path);
    
    for mp3 in &mp3_files {
        let foo = utils::RTools { apath: dir_path.to_string() };
        if utils::RTools::check_file_name_format(&foo) {
            println!("{} matches the pattern", mp3);
        } else {
            println!("{} does not match the pattern", mp3);
        }
    }

    // let results = utils::RTools::check_file_name_format(&foo);
    // let mp3_files = utils::find_mp3s(dir_path);
    // let results = check_file_names(&mp3_files);




    // println!("Matches: {:?}", results.0.len());
    // println!("No matches: {:?}", results.1.len());
    // println!("Found {} mp3 files.", mp3_files.len());
}
