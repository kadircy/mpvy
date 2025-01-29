use crate::log::*;
use dirs;
use std::fs;

/// Returns the path of given playlist file
pub fn playlist_path(name: &String) -> String {
    return format!(
        "{}/mpvy/playlist/{}.txt",
        dirs::config_dir().unwrap().display(),
        name
    );
}

/// Read the given querys from the playlist file (`$XDG_CONFIG_HOME/mpvy/playlist/{name}.txt`)
/// It is have error handling, so using this instead of directly using **std::fs** will be more safe
/// And will be more informative for users.
pub fn read_playlist(name: String) -> Result<String, String> {
    let path: String = playlist_path(&name);
    // Check if the playlist file exists
    // ? Why we don't exit when unable to find playlist
    // Because we need more logging on main, mpvy does this on main.
    // NOTE: Consider moving exit(1) to here.
    if fs::exists(&path).unwrap_or(false) == false {
        error(
            "Playlist Read".to_string(),
            format!("Unable to find playlist with name: {}", name),
        );
        return Err("Invalid path or playlist name".to_string());
    }
    let content: Result<String, std::io::Error> = fs::read_to_string(&path);
    if content.is_err() {
        error(
            "Playlist Read".to_string(),
            format!("An error occured while reading contents of file: {}", &path),
        );
        return Err("Unexpected error while reading file contents.".to_string());
    }
    return Ok(content.unwrap());
}

/// Write the given content to the playlist file (`$XDG_CONFIG_HOME/mpvy/playlist/{name}.txt`)
/// It has error handling too, so using this instead of directly using **std::fs** will be more safe
/// And will be more informative for users
pub fn write_playlist(name: &String, content: String) -> Result<(), String> {
    let path: String = playlist_path(&name);
    let result: Result<(), std::io::Error> = fs::write(path, content);
    if result.is_err() {
        error(
            "Playlist Write".to_string(),
            "An error occured while writing content to playlist file.".to_string(),
        );
        return Err("Unable to write content to playlist file".to_string());
    }
    return Ok(());
}
