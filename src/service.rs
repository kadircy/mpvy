use crate::log::*;
use crate::yt_dlp;
use dirs;
use std::process::Command;

// Returns the path to the mpv IPC socket, which is used for communication with mpv
pub fn ipc_path() -> String {
    format!(
        "{}/mpv/socket",
        dirs::config_dir()
            .expect("!!! FATAL ERROR !!! Unable to get or find config directory for mpv IPC")
            .display()
    )
}

// Spawns the "mpv" process to play a video from the given path
pub fn mpv(path: &str) -> Result<(), String> {
    let command = Command::new("mpv")
        .arg("--no-terminal") // Prevent terminal output from mpv
        .arg(format!(
            "--log-file={}/mpvy/log/mpv.log",
            dirs::config_dir()
                .expect("!!! FATAL ERROR !!! Unable to get or find config directory for mpv log")
                .display()
        )) // Change log file path to 'mpvy' log directory
        .arg(format!("--input-ipc-server={}", ipc_path())) // Set up IPC server for controlling mpv
        .arg(path) // Path to the video file to be played
        .spawn(); // Start the mpv process

    if command.is_err() {
        error(
            "Service Mpv".to_string(),
            "Unable to play video with mpv. Maybe mpv is not downloaded?".to_string(),
        );
        return Err(command.unwrap_err().to_string());
    }
    Ok(())
}

// Handles the process of fetching video info and playing it
// If the video is already downloaded, it will directly play it, otherwise, it will download first
pub fn play(title: &str) -> Result<yt_dlp::VideoInfo, String> {
    info(
        "Service Play".to_string(),
        format!("Requested a play with query '{}'", title).to_string(),
    );

    let video_result = yt_dlp::get_info(title);
    let video = match video_result {
        Ok(info) => info,
        Err(err) => {
            error(
                "Service Play".to_string(),
                "Unable to get video info".to_string(),
            );
            return Err(err.to_string());
        }
    };

    let path = format!(
        "{}/{}_{}.mp3", // $HOME/.config/mpvy/mp3/<video_title>_<video_id>.mp3
        yt_dlp::get_download_path(),
        video.title,
        video.id
    );

    // If the video is already downloaded, play it directly
    if std::fs::exists(&path).unwrap_or(false) {
        let result = mpv(&path);
        if result.is_err() {
            return Err(String::from(result.unwrap_err()));
        }
        return Ok(video);
    }

    // If the video is not downloaded, download it first
    let output = yt_dlp::download(&video.id);
    if output.is_err() {
        error(
            "Service Play".to_string(),
            "Unable to download video".to_string(),
        );
        return Err(output.unwrap_err().to_string());
    }

    let result = mpv(&path);
    if result.is_err() {
        return Err(String::from(result.unwrap_err()));
    }

    Ok(video)
}
