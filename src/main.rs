use std::fs::File;
use std::io::{self, Write};
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use std::{env, fs};
pub mod log;
pub mod service;
pub mod yt_dlp;
use crate::log::*;

fn clear_console() {
    print!("\x1b[2J\x1b[H");
    io::stdout().flush().unwrap();
}

fn duration_to_seconds(duration: &str) -> u64 {
    let parts: Vec<&str> = duration.split(':').collect();
    if parts.len() == 2 {
        let minutes: u64 = parts[0].parse().unwrap_or(0);
        let seconds: u64 = parts[1].parse().unwrap_or(0);
        return minutes * 60 + seconds;
    }
    0
}

fn clean_old_mp3_files() {
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
            fs::remove_file(file.path()).unwrap_or_else(|e| {
                eprintln!("Failed to delete file {}: {}", file.path().display(), e);
            });
        }
    }
}

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
}

fn main() {
    let cava_enabled = env::args().any(|arg| arg == "--cava");

    clean_old_mp3_files();
    clean_log_files();

    clear_console();

    let mut input = String::new();
    print!("search (separated by commas) --> ");
    std::io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input from terminal");

    clear_console();

    let titles = input.trim().split(",");

    let cava_process = if cava_enabled {
        info(
            "Mpvy Cava".to_string(),
            "Cava is enabled. Starting child process".to_string(),
        );

        let cava = Command::new("cava")
            .spawn()
            .expect("Failed to start 'cava' process");
        Some(cava)
    } else {
        None
    };

    for title in titles {
        let video_info = service::play(title.trim()).unwrap();
        let duration_in_seconds = duration_to_seconds(&video_info.duration);
        // Wait for audio to end.
        sleep(Duration::new(duration_in_seconds, 0));
    }

    // If "cava" was enabled, kill the cava process after video playback is finished
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
