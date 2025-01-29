use crate::log::*;
use dirs;
use std::fs;

pub fn playlist_path(name: &String) -> String {
    return format!(
        "{}/mpvy/playlist/{}.txt",
        dirs::config_dir().unwrap().display(),
        name
    );
}

pub fn read_playlist(name: String) -> Result<String, String> {
    let path = playlist_path(&name);
    if fs::exists(&path).unwrap_or(false) == false {
        error(
            "Playlist Read".to_string(),
            format!("Unable to find playlist with name: {}", name),
        );
        return Err("Invalid path or playlist name".to_string());
    }
    let content = fs::read_to_string(&path);
    if content.is_err() {
        error(
            "Playlist Read".to_string(),
            format!("An error occured while reading contents of file: {}", &path),
        );
        return Err("Unexpected error while reading file contents.".to_string());
    }
    return Ok(content.unwrap());
}

pub fn write_playlist(name: &String, content: String) -> Result<(), String> {
    let path = playlist_path(&name);
    let result = fs::write(path, content);
    if result.is_err() {
        error(
            "Playlist Write".to_string(),
            "An error occured while writing content to playlist file.".to_string(),
        );
        return Err("Unable to write content to playlist file".to_string());
    }
    return Ok(());
}
