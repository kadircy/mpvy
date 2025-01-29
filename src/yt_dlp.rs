use crate::config;
use crate::log::*;
use dirs;
use std::process::Command;
use std::str::Lines;

/// An struct which have information about video.
/// Values:
///
/// ```txt
/// duration:     Video duration in this format: HH:MM:SS
/// title:        Video title
/// url:          Video webpage url (https://youtube.com/watch?v={id})
/// id:           Video spesific ID
/// ```
#[derive(Debug)]
pub struct VideoInfo {
    pub duration: String, // Video duration
    pub title: String,    // Video title
    pub url: String,      // Video URL
    pub id: String,       // Video ID
}

/// Returns the path of downloaded files. (`$XDG_CONFIG_HOME/mpvy/mp3`)
/// If the `dirs` crate fails (like not founding `$XDG_CONFIG_HOME` on Linux), it automaticlly fallbacks to `$HOME/.config`
/// Which is risky on Windows (if even runs) because the **mpvy** config path
/// Is different on different operating systems.
pub fn get_download_path() -> String {
    let dotconfig = dirs::config_dir();
    if dotconfig.is_none() {
        warning(
            "YoutubeDLP DownloadPath",
            "Unable to get user config directory. Using $HOME/.config",
        );
    }
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

/// Get information about video with an query.
/// Returns VideoInfo struct with given values.
/// Uses `ytsearch:` schema for finding videos.
pub fn get_info(query: &str) -> Result<VideoInfo, String> {
    let output = Command::new("yt-dlp")
        .arg(format!("ytsearch:{}", query))
        .arg("--print")
        .arg("%(duration>%H:%M:%S)s\n%(title)s\n%(id)s\n%(webpage_url)s")
        .output();

    let output = match output {
        Ok(o) => o,
        Err(_) => {
            error(
                "YoutubeDLP Info",
                "An error occurred while executing 'yt-dlp'.",
            );
            return Err("Failed to execute 'yt-dlp'.".to_string());
        }
    };

    if output.status.success() == false {
        error(
            "YoutubeDLP Info",
            "Unable to get successful output. Maybe 'yt-dlp' is not installed?",
        );
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    let result: String = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if result.is_empty() {
        info(
            "YoutubeDLP Info",
            "The query result is empty. Unable to find video on YouTube.",
        );
        return Err("Unable to find video on YouTube.".to_string());
    }

    let mut lines: Lines<'_> = result.lines();
    let duration: String = lines.next().unwrap_or_default().to_string();
    let title: String = lines.next().unwrap_or_default().to_string();
    let url: String = lines.next().unwrap_or_default().to_string();
    let id: String = lines
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

/// Downloads the video with given url. Nothing more to say.
pub fn download(url: &String) -> Result<(), String> {
    let config = config::get_config();
    let mut quality: String = "0".to_string();
    let mut concurrent_fragments: String = "4".to_string();

    if config.is_ok() {
        if config.clone().unwrap().contains_key(config::AUDIO_QUALITY) {
            quality = config.clone().unwrap()[config::AUDIO_QUALITY]
                .parse::<String>()
                .unwrap();
        }

        if config
            .clone()
            .unwrap()
            .contains_key(config::CONCURRENT_FRAGMENTS)
        {
            concurrent_fragments = config.clone().unwrap()[config::CONCURRENT_FRAGMENTS]
                .parse::<String>()
                .unwrap();
        }
    };

    info(
        "YoutubeDLP Download",
        &format!("Requested a download with url: {}", &url),
    );

    let path: String = get_download_path();

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
                "YoutubeDLP Download",
                "An error occurred while executing 'yt-dlp'",
            );
            return Err("Failed to execute 'yt-dlp'".to_string());
        }
    };

    if output.status.success() == false {
        error(
            "YoutubeDLP Download",
            "Unable to download video successfully. Maybe 'yt-dlp' is not installed?",
        );
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    info("YoutubeDLP Download", "Video Downloaded successfully");
    return Ok(());
}
