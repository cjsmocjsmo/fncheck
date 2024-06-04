use walkdir::WalkDir;
use std::fs;
use std::path::Path;
use id3::{Tag, TagLike};

// use std::env;
// use image::GenericImageView;




pub fn find_mp3s(dir_path: &String) -> Vec<String> {
    let mut mp3_files = Vec::new();
    for entry in WalkDir::new(dir_path) {
        let entry = entry.unwrap();
        if entry.path().extension().map_or(false, |ext| ext == "mp3" || ext == "flac") {
            mp3_files.push(entry.path().to_string_lossy().into_owned());
        }
    }
    mp3_files
}

#[derive(Debug)]
pub struct RTools {
    pub apath: String,
}

impl RTools {
    pub fn check_file_name_format(&self) -> bool {
        let re = regex::Regex::new(r"\d+_\d\d+_-_.+_-_.+_-_.+\.mp3").unwrap();
        let re2 = regex::Regex::new(r"\d+_\d\d+_-_.+_-_.+_-_.+\.flac").unwrap();
        let file_name = Path::new(&self.apath).file_name().unwrap().to_str().unwrap();

        if re.is_match(file_name) {
            return true;
        } else if re2.is_match(file_name) {
            return true;
        } else {
            return false;
        }
    }



    pub fn split_base_dir_filename(&self) -> (String, String) {
        let path = Path::new(&self.apath);
        let dir_path = path.parent().unwrap();
        let filename = path.file_name().unwrap();

        (
            dir_path.to_string_lossy().to_string(),
            filename.to_string_lossy().to_string(),
        )
    }

    pub fn split_artist_album(&self) -> (String, String) {
        let path = Path::new(&self.apath);
        let basedir = path.parent().unwrap();
        let basedirpath = Path::new(&basedir);
        let album = basedirpath.file_name().unwrap();
        let basedirpath2 = basedirpath.parent().unwrap();
        let bdp3 = Path::new(&basedirpath2);
        let artist = bdp3.file_name().unwrap();
        let album_string = album.to_string_lossy().to_string();
        let artist_string = artist.to_string_lossy().to_string();

        let album_final = album_string.replace("_", " ");
        let artist_final = artist_string.replace("_", " ");

        ( artist_final, album_final )
    }

    pub fn get_tag_info(&self) -> Result<(String, String, String, String), std::io::Error> {
        let tag = match Tag::read_from_path(&self.apath) {
            Ok(tag) => tag,
            Err(_) => {
                let target_dir = Path::new("/home/pi/needs_work");
                if !target_dir.exists() {
                    fs::create_dir_all(target_dir)?;
                }
                fs::rename(&self.apath, target_dir.join(Path::new(&self.apath).file_name().unwrap()))?;
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "No ID3 tag found"));
            }
        };
    
        let artist = tag.artist().expect(&self.apath);
        let album = tag.album().expect(&self.apath);
        let song = tag.title().expect(&self.apath);
        let track = tag.track().expect(&self.apath);
    
        Ok((artist.to_string(), album.to_string(), song.to_string(), track.to_string()))
    }

    // pub fn split_ext(&self) -> String {
    //     let path = Path::new(&self.apath);
    //     let boo_results = path.extension();
    //     let boo = match boo_results {
    //         Some(b) => b.to_string_lossy().to_string(),
    //         None => "split_ext did not work".to_string(),
    //     };
    //     let ext = ".".to_string() + boo.as_str();

    //     ext
    // }

    // pub fn get_file_size(&self) -> String {
    //     let path = Path::new(&self.apath);

    //     path.size_on_disk().unwrap().to_string()
    // }

    // pub fn get_dims(&self) -> (u32, u32) {
    //     let dims = get_image_dims(&self.apath);

    //     dims
    // }
    // pub fn artist_starts_with(&self) -> String {
    //     let tag = Tag::read_from_path(&self.apath).expect(&self.apath);
    //     let artist = tag.artist().expect(&self.apath);
    //     let first_letter = artist.chars().next().unwrap();

    //     first_letter.to_string()
    // }

    // pub fn album_starts_with(&self) -> String {
    //     let tag = Tag::read_from_path(&self.apath).expect(&self.apath);
    //     let album = tag.album().expect(&self.apath);
    //     let first_letter = album.chars().next().unwrap();

    //     first_letter.to_string()
    // }

    // pub fn song_starts_with(&self) -> String {
    //     let tag = Tag::read_from_path(&self.apath).expect(&self.apath);
    //     let song = tag.title().expect(&self.apath);
    //     let first_letter = song.chars().next().unwrap();

    //     first_letter.to_string()
    // }

    // pub fn create_mp3_play_path(&self) -> String {
    //     let psplit = self.apath.split("/").skip(3).collect::<Vec<&str>>();
    //     let assend = psplit.join("/");

    //     let myhttpd = env::var("RUSIC_HTTP_ADDR").unwrap();
    //     let myport = env::var("RUSIC_PORT").unwrap();

    //     // let playpath = myhttpd + &myport + "/Music/" + assend.as_str();
    //     let playpath = myhttpd + &myport + "/" + assend.as_str();

    //     playpath
    // }
}