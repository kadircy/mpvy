use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;
use std::str::Split;
use std::thread::sleep;
use std::time::Duration;
use std::{env, fs};

pub mod config;
pub mod log;
pub mod playlist;
pub mod service;
pub mod yt_dlp;
use crate::log::*;

/// Clear the console with some unicode char.
fn clear_console() {
    print!("\x1b[2J\x1b[H");
    io::stdout().flush().unwrap();
}

/// Convert `yt-dlp` video duration from string to u64 (seconds).
/// Supports both of **HH:MM** and **HH:MM:SS** formats.
/// If an unsupported format given, returns 0.
fn duration_to_seconds(duration: &str) -> u64 {
    let parts: Vec<&str> = duration.split(':').collect();
    match parts.len() {
        2 => {
            let minutes: u64 = parts[0].parse().unwrap_or(0);
            let seconds: u64 = parts[1].parse().unwrap_or(0);
            minutes * 60 + seconds
        }
        3 => {
            let hours: u64 = parts[0].parse().unwrap_or(0);
            let minutes: u64 = parts[1].parse().unwrap_or(0);
            let seconds: u64 = parts[2].parse().unwrap_or(0);
            hours * 3600 + minutes * 60 + seconds
        }
        _ => 0,
    }
}

/// If audio file count is more than the **MAX_FILE_COUNT** (refer to `src/config.rs`)
/// It deletes the files from oldest.
/// The **MAX_FILE_COUNT** can be defined by user.
fn clean_old_mp3_files() {
    let config = config::get_config();
    let mut count: usize = 15;
    if config.is_ok() && config.clone().unwrap().contains_key(config::MAX_FILE_COUNT) {
        count = config.unwrap()[config::MAX_FILE_COUNT]
            .parse::<usize>()
            .unwrap();
    }
    info(
        "Mpvy CleanOldFiles",
        &format!(
            "Deleting old mp3 files to reach max count ({} files)",
            count
        ),
    );
    let mp3_dir: PathBuf = dirs::config_dir().unwrap().join("mpvy/mp3");
    if !mp3_dir.exists() {
        return;
    }
    // Collect all files in directory
    let mut files = fs::read_dir(mp3_dir)
        .unwrap()
        .filter_map(|entry| entry.ok())
        .filter(|e| e.file_type().unwrap().is_file())
        .collect::<Vec<_>>();
    // Sort files by their last modified date
    files.sort_by(|a, b| {
        a.metadata()
            .unwrap()
            .modified()
            .unwrap()
            .cmp(&b.metadata().unwrap().modified().unwrap())
    });
    if files.len() > count {
        let files_to_remove: usize = files.len() - count;
        for file in files.iter().take(files_to_remove) {
            info(
                "Mpvy CleanOldFiles",
                &format!("Deleting file: {}", file.path().display()),
            );
            fs::remove_file(file.path()).unwrap_or_else(|e| {
                error(
                    "Mpvy CleanOldFiles",
                    &format!("Failed to delete file {}: {}", file.path().display(), e),
                );
            });
        }
    }
}

/// Recreate all the log files (`mpv.log` and `mpvy.log`) from scratch
/// For removing old log messages
/// When `mpvy` runned newly.
fn clean_log_files() {
    let log_dir = dirs::config_dir().unwrap().join("mpvy/log");
    if !log_dir.exists() {
        return;
    }
    let mpv_log_path = log_dir.join("mpv.log");
    let mpvy_log_path = log_dir.join("mpvy.log");
    if mpv_log_path.exists() {
        File::create(mpv_log_path).unwrap();
    }
    if mpvy_log_path.exists() {
        File::create(mpvy_log_path).unwrap();
    }

    info("Mpvy CleanLogFiles", "Old log files deleted.");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let playlist: Option<String> = args
        .iter()
        .position(|arg| arg == "--playlist")
        .and_then(|i| args.get(i + 1).map(|s| s.to_string()));
    let cava_enabled: bool = args.contains(&"--cava".to_string());
    let save_playlist: Option<String> = args
        .iter()
        .position(|arg| arg == "--save-playlist")
        .and_then(|i| args.get(i + 1).map(|s| s.to_string()));

    clean_log_files();
    clean_old_mp3_files();
    clear_console();
    info("Mpvy Main", "Getting user input");

    let mut input: String = String::new();

    if let None = playlist {
        print!("search (separated by commas) --> ");
        std::io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input from terminal");
    } else {
        // If there is some playlist to play, read it content and write it to the input for playing.
        let playlist_content: Result<String, String> = playlist::read_playlist(&playlist.unwrap());
        if playlist_content.is_err() {
            error("Mpvy PlaylistCheck", "Playlist Content returned an Err value. Exiting with code 1 because nothing to play.");
            println!("Playlist not found (or another error occured). Please check logs for more information.");
            std::process::exit(1);
        }
        input = playlist_content.unwrap();
    }

    // If there is some playlist to save, write it to the file.
    if let Some(playlist_name) = save_playlist {
        let result: Result<(), String> =
            playlist::write_playlist(&playlist_name, input.trim().to_string());

        if result.is_err() {
            error(
                "Mpvy PlaylistCheck",
                "An error occured while writing content to playlist file.",
            );
        }

        info(
            "Mpvy SavePlaylist",
            &format!("Playlist saved as: {}", playlist_name),
        );
    }

    clear_console();

    if input.trim().is_empty() {
        info("Mpvy Main", "User input is empty. Exiting with code 0.");
        println!("An empty input given. Exiting with code 0.");
        std::process::exit(0);
    }

    // Split querys with commas
    let titles: Split<'_, &str> = input.trim().split(",");
    let cava_process: Option<std::process::Child> = if cava_enabled {
        info("Mpvy Cava", "Cava is enabled. Starting child process");
        Some(
            Command::new("cava")
                .spawn()
                .expect("Failed to start 'cava' process"),
        )
    } else {
        None
    };

    for title in titles {
        info(
            "Mpvy TitleLoop",
            &format!("Reached title: {}", title.trim()),
        );
        let video_info = service::play(title.trim()).unwrap();
        let duration_in_seconds = duration_to_seconds(&video_info.duration);
        info("Mpvy TitleLoop", "Waiting for audio to end.");
        sleep(Duration::new(duration_in_seconds, 0));
    }

    // If there is a Cava process, kill it.
    if let Some(mut cava) = cava_process {
        if let Err(e) = cava.kill() {
            error("Mpvy Cava", &format!("Failed to kill 'cava': {}", e));
        } else {
            info("Mpvy Cava", "Cava process terminated");
        }
    }
}
