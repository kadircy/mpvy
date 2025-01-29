use crate::log::*;
use crate::yt_dlp;
use crate::yt_dlp::VideoInfo;
use dirs;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

/// Returns the path of Mpv IPC file.
/// `mpvy` don't use this directly but giving this as argument to **mpv**
/// Will be more good for users when controlling **mpv** via **IPC**
pub fn ipc_path() -> String {
    format!(
        "{}/mpv/socket",
        dirs::config_dir()
            .expect("Unexpected Error: Unable to get config directory.")
            .display()
    )
}

/// Spawns `mpv` command with some default arguments to prevent issues.
/// We are using **local files** because we are want to play audio **downloaded (or cached)**
/// Even user is offline. Also you can copy your musics to anywhere.
fn mpv(path: &str) -> Result<(), String> {
    info("Service Mpv", "Playing audio with 'mpv'");
    let command = Command::new("mpv")
        .arg("--no-terminal") // Prevent terminal output from mpv
        .arg(format!(
            "--log-file={}/mpvy/log/mpv.log",
            dirs::config_dir()
                .expect("Unexpected Error: Unable to get config directory.")
                .display()
        )) // Change log file path to 'mpvy' log directory
        .arg(format!("--input-ipc-server={}", ipc_path())) // Set up IPC server for controlling mpv
        .arg(path) // Path to the video file to be played
        .spawn(); // Start the mpv process

    if command.is_err() {
        error(
            "Service Mpv",
            "Unable to play audio with mpv. Maybe mpv is not downloaded?",
        );
        return Err(command.unwrap_err().to_string());
    }
    Ok(())
}

/// An function which merges `yt-dlp` APIs for easy usage in coding.
/// Also uses some optimizations for better user experience.
/// And logs informative messages
pub fn play(title: &str, duration: u64) -> Result<yt_dlp::VideoInfo, String> {
    info("Service Play", &format!("Trying to play '{}'.", title));

    let video_result: Result<yt_dlp::VideoInfo, String> = yt_dlp::get_info(title); // Get video information (such as duration, title, id)
    let video: VideoInfo = match video_result {
        Ok(info) => info,
        Err(err) => {
            error("Service Play", "Unable to get audio info.");
            return Err(err.to_string());
        }
    };

    let path: String = format!(
        "{}/{}_{}.mp3", // $HOME/.config/mpvy/mp3/<video_title>_<video_id>.mp3
        yt_dlp::get_download_path(),
        video.title,
        video.id
    );

    // If the audio is already downloaded, play it directly
    if std::fs::exists(&path).unwrap_or(false) {
        info(
            "Service Play",
            "Audio found in the mp3 directory, skipping download.",
        );
        let seconds: Duration = Duration::from_secs(duration);
        // Wait for previous audio to end
        info("Service Play", "Waiting for previous audio to end.");
        sleep(seconds);
        info("Service Play", "Previous audio ended, playing next audio.");
        let result: Result<(), String> = mpv(&path);
        if result.is_err() {
            return Err(String::from(result.unwrap_err()));
        }
        return Ok(video);
    }

    // If the audio is not downloaded, download it first
    info("Service Play", "Downloading audio.");
    let output: Result<(), String> = yt_dlp::download(&video.id);
    if output.is_err() {
        error("Service Play", "Unable to download audio.");
        return Err(output.unwrap_err().to_string());
    }
    info("Service Play", "Video downloaded successfuly, now playing.");
    let result: Result<(), String> = mpv(&path);
    if result.is_err() {
        return Err(String::from(result.unwrap_err()));
    }

    Ok(video)
}
