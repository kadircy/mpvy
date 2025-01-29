use crate::config;
use crate::log::*;
use dirs;
use std::process::Command;

#[derive(Debug)]
pub struct VideoInfo {
    pub duration: String, // Video duration
    pub title: String,    // Video title
    pub url: String,      // Video URL
    pub id: String,       // Video ID
}

// Returns the download path for the video, defaults to $HOME/.config if config directory is unavailable
pub fn get_download_path() -> String {
    let dotconfig = dirs::config_dir();
    if dotconfig.is_none() {
        warning(
            "YoutubeDLP DownloadPath".to_string(),
            "Unable to get user config directory. Using $HOME/.config".to_string(),
        );
    }
    // Form the download path, using .config directory or default $HOME/.config
    let path = format!(
        "{}/mpvy/mp3",
        dirs::config_dir()
            .unwrap_or(
                format!(
                    "{}/.config",
                    std::env::var("HOME")
                        .expect("!!! FATAL ERROR !!! Unable to get $HOME variable. It is missing.")
                )
                .into()
            )
            .display()
    );
    return path;
}

// Fetches video info from yt-dlp using the search query
// It parses the yt-dlp output and returns a VideoInfo struct with relevant data
pub fn get_info(query: &str) -> Result<VideoInfo, String> {
    let output = Command::new("yt-dlp")
        .arg(format!("ytsearch:{}", query)) // Search query with yt-dlp
        .arg("--print")
        .arg("%(duration>%H:%M:%S)s\n%(title)s\n%(id)s\n%(webpage_url)s") // Specify the format to retrieve relevant information
        .output();

    let output = match output {
        Ok(o) => o,
        Err(_) => {
            error(
                "YoutubeDLP Info".to_string(),
                "An error occurred while executing 'yt-dlp'.".to_string(),
            );
            return Err("Failed to execute 'yt-dlp'.".to_string());
        }
    };

    if output.status.success() == false {
        error(
            "YoutubeDLP Info".to_string(),
            "Unable to get successful output. Maybe 'yt-dlp' is not installed?".to_string(),
        );
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if result.is_empty() {
        info(
            "YoutubeDLP Info".to_string(),
            "The query result is empty. Unable to find video on YouTube.".to_string(),
        );
        return Err("Unable to find video on YouTube.".to_string());
    }

    // Parse the video details from the command output
    let mut lines = result.lines();
    let duration = lines.next().unwrap_or_default().to_string();
    let title = lines.next().unwrap_or_default().to_string();
    let url = lines.next().unwrap_or_default().to_string();
    let id = lines
        .next()
        .unwrap_or_default()
        .rsplit('=')
        .next()
        .unwrap_or_default()
        .to_string();
    Ok(VideoInfo {
        duration,
        title,
        url,
        id,
    })
}

// Downloads the video using yt-dlp and saves it in the specified format (mp3)
// The download path is provided by the get_download_path function
pub fn download(url: &String) -> Result<(), String> {
    let config = config::get_config();
    let mut quality = "0".to_string();
    let mut concurrent_fragments = "4".to_string();

    if config.is_ok() {
        if config.clone().unwrap().contains_key(config::AUDIO_QUALITY) {
            quality = config.clone().unwrap()[config::AUDIO_QUALITY]
                .parse::<String>()
                .unwrap();
        }

        if config.clone().unwrap().contains_key(config::CONCURRENT_FRAGMENTS) {
            concurrent_fragments = config.clone().unwrap()[config::CONCURRENT_FRAGMENTS]
                .parse::<String>()
                .unwrap();
        }
    };

    info(
        "YoutubeDLP Download".to_string(),
        format!("Requested a download with url: {}", &url).to_string(),
    );

    let path = get_download_path();

    let output = Command::new("yt-dlp")
        .arg(url) // The URL of the video to download
        .arg("-x") // Extract audio only
        .arg("--audio-format") // Set the audio format to mp3
        .arg("mp3")
        .arg("--audio-quality") // Set the audio quality to the highest (0 is the best)
        .arg(quality)
        .arg("--no-playlist") // Disable playlist downloading, only download a single video
        .arg("--output") // Specify the output file path
        .arg(format!("{}/%(title)s_%(id)s.%(ext)s", path)) // Path where to save the file
        .arg("--concurrent-fragments") // Download video fragments concurrently
        .arg(concurrent_fragments) // Number of concurrent fragments (adjust based on internet speed)
        .arg("--postprocessor-args") // Pass additional arguments to ffmpeg for processing
        .arg("ffmpeg:-preset ultrafast") // Set ffmpeg to use the ultrafast preset for faster processing
        .output();

    let output = match output {
        Ok(o) => o,
        Err(_) => {
            error(
                "YoutubeDLP Download".to_string(),
                "An error occurred while executing 'yt-dlp'".to_string(),
            );
            return Err("Failed to execute 'yt-dlp'".to_string());
        }
    };

    if output.status.success() == false {
        error(
            "YoutubeDLP Download".to_string(),
            "Unable to download video successfully. Maybe 'yt-dlp' is not installed?".to_string(),
        );
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    info(
        "YoutubeDLP Download".to_string(),
        "Video Downloaded successfully".to_string(),
    );
    return Ok(());
}
