use crate::log::*;
use dirs;
use std::fs;

/// Returns the path of all playlists directory
pub fn playlists_path() -> String {
    return format!(
        "{}/mpvy/playlist/",
        dirs::config_dir()
            .expect("Unexpected Error: Unable to get config directory for playlists.")
            .display()
    );
}

/// Returns the path of given playlist file
pub fn playlist_path(name: &str) -> String {
    return format!(
        "{}/mpvy/playlist/{}.txt",
        dirs::config_dir()
            .expect("Unexpected Error: Unable to get config directory for playlists.")
            .display(),
        name
    );
}

/// Read the given querys from the playlist file (`$XDG_CONFIG_HOME/mpvy/playlist/{name}.txt`)
/// It is have error handling, so using this instead of directly using **std::fs** will be more safe
/// And will be more informative for users.
pub fn read_playlist(name: &str) -> Result<String, String> {
    let path: String = playlist_path(name);
    // Check if the playlist file exists
    if fs::exists(&path).unwrap_or(false) == false {
        error(
            "Playlist Read",
            &format!("Unable to find playlist with name: '{}'", name),
        );
        return Err("Invalid path or playlist name".to_string());
    }
    let content: Result<String, std::io::Error> = fs::read_to_string(&path);
    if content.is_err() {
        error(
            "Playlist Read",
            &format!(
                "An error occured while reading contents of file: '{}'",
                &path
            ),
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
            "Playlist Write",
            "An error occured while writing content to playlist file.",
        );
        return Err("Unable to write content to playlist file".to_string());
    }
    return Ok(());
}

/// Get all avaliable playlists in the directory of `$XDG_CONFIG_HOME/mpvy/playlist`
/// It has error handling too. Don't need more explanation.
pub fn get_playlists() -> Result<Vec<String>, String> {
    info("Playlist List", "Getting playlists.");
    let path: String = playlists_path();
    // Create an Vec for playlist titles.
    let mut entries = Vec::new();

    if fs::exists(&path).unwrap_or(false) == false {
        info(
            "Playlist List",
            "The playlist directory doesn't exists. Creating a new one.",
        );
        let result = fs::create_dir(&path);
        if result.is_err() {
            error(
                "Playlist List",
                "An error occured while creating directory of playlists.",
            );
            return Err("Unable to create directory".to_string());
        }
        return Ok(vec![]);
    }

    // Create an index for logging.
    let mut index: i32 = 0;
    // Read the directory `$XDG_CONFIG_HOME/mpvy/playlist` to get playlist files.
    for entry in fs::read_dir(path).expect("Unexpected Error: Unable to read playlists directory.")
    {
        info(
            "Playlist List",
            &format!("Reached index {} in loop.", index),
        );
        let entry = entry.expect("Unexpected Error: Unable to get entry in loop.");
        let path = entry.path();
        // Remove the extension from file (playlist.txt --> playlist).
        let title = path.file_stem()
            .expect("Unexpected Error: Unable to get file stem of playlist. Maybe the 'file' is a directory?")
            .to_str()
        .unwrap_or("Error");

        info("Playlist List", "Pushing playlist into the Vec.");
        entries.push(String::from(title));
        index = index + 1;
    }

    return Ok(entries);
}
