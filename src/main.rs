use std::fs::File;
use std::io::{self, Write};
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use std::{env, fs};

pub mod log;
pub mod playlist;
pub mod service;
pub mod yt_dlp;
use crate::log::*;

fn clear_console() {
    print!("\x1b[2J\x1b[H");
    io::stdout().flush().unwrap();
}

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

fn clean_old_mp3_files() {
    info(
        "Mpvy CleanOldFiles".to_string(),
        "Deleting old mp3 files to reach max count (15 files)".to_string(),
    );
    let mp3_dir = dirs::config_dir().unwrap().join("mpvy/mp3");
    if !mp3_dir.exists() {
        return;
    }
    let mut files = fs::read_dir(mp3_dir)
        .unwrap()
        .filter_map(|entry| entry.ok())
        .filter(|e| e.file_type().unwrap().is_file())
        .collect::<Vec<_>>();
    files.sort_by(|a, b| {
        a.metadata()
            .unwrap()
            .modified()
            .unwrap()
            .cmp(&b.metadata().unwrap().modified().unwrap())
    });
    if files.len() > 15 {
        let files_to_remove = files.len() - 15;
        for file in files.iter().take(files_to_remove) {
            info(
                "Mpvy CleanOldFiles".to_string(),
                format!("Deleting file: {}", file.path().display()),
            );
            fs::remove_file(file.path()).unwrap_or_else(|e| {
                error(
                    "Mpvy CleanOldFiles".to_string(),
                    format!("Failed to delete file {}: {}", file.path().display(), e),
                );
            });
        }
    }
}

fn clean_log_files() {
    info(
        "Mpvy CleanLogFiles".to_string(),
        "Deleting old log files".to_string(),
    );
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
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let playlist = args
        .iter()
        .position(|arg| arg == "--playlist")
        .and_then(|i| args.get(i + 1).map(|s| s.to_string()));
    let cava_enabled = args.contains(&"--cava".to_string());
    let save_playlist = args
        .iter()
        .position(|arg| arg == "--save-playlist")
        .and_then(|i| args.get(i + 1).map(|s| s.to_string()));

    clean_log_files();
    clean_old_mp3_files();
    clear_console();
    info("Mpvy Main".to_string(), "Getting user input".to_string());

    let mut input = String::new();

    if let None = playlist {
        print!("search (separated by commas) --> ");
        std::io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input from terminal");
    } else {
        let playlist_content = playlist::read_playlist(playlist.unwrap());
        if playlist_content.is_err() {
            error("Mpvy PlaylistCheck".to_string(), "Playlist Content returned an Err value. Exiting with code 1 because nothing to play.".to_string());
            println!("Playlist not found. Please check logs for more information.");
            std::process::exit(1);
        }
        input = playlist_content.unwrap();
    }

    if let Some(playlist_name) = save_playlist {
        let result = playlist::write_playlist(&playlist_name, input.trim().to_string());

        if result.is_err() {
            error(
                "Mpvy PlaylistCheck".to_string(),
                "An error occured while writing content to playlist file.".to_string(),
            );
        }

        info(
            "Mpvy SavePlaylist".to_string(),
            format!("Playlist saved as: {}", playlist_name),
        );
    }

    clear_console();

    if input.trim().is_empty() {
        info(
            "Mpvy Main".to_string(),
            "User input is empty. Exiting with code 0.".to_string(),
        );
        println!("An empty input given. Exiting with code 0.");
        std::process::exit(0);
    }

    let titles = input.trim().split(",");
    let cava_process = if cava_enabled {
        info(
            "Mpvy Cava".to_string(),
            "Cava is enabled. Starting child process".to_string(),
        );
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
            "Mpvy TitleLoop".to_string(),
            format!("Reached title: {}", title.trim()),
        );
        let video_info = service::play(title.trim()).unwrap();
        let duration_in_seconds = duration_to_seconds(&video_info.duration);
        info(
            "Mpvy TitleLoop".to_string(),
            "Waiting for audio to end.".to_string(),
        );
        sleep(Duration::new(duration_in_seconds, 0));
    }

    if let Some(mut cava) = cava_process {
        if let Err(e) = cava.kill() {
            error(
                "Mpvy Cava".to_string(),
                format!("Failed to kill 'cava': {}", e),
            );
        } else {
            info(
                "Mpvy Cava".to_string(),
                "Cava process terminated".to_string(),
            );
        }
    }
}
