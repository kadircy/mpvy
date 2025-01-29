use crate::log::*;
use config::Config;
use dirs;
use std::collections::HashMap;
use std::fs;

// CONFIGURATION KEYS //
/// Change max file count for saved audios. Default: `15`
pub const MAX_FILE_COUNT: &str = "max_file_count";
/// Change audio quality for `yt-dlp` download. Default: `0` - Best: `0` - Worst: `10`
pub const AUDIO_QUALITY: &str = "audio_quality";
/// Number of concurrent fragments for `yt-dlp` download. Default `4`
pub const CONCURRENT_FRAGMENTS: &str = "concurrent_fragments";

/// Get configuration file which is at `$XDG_CONFIG_HOME/mpvy/config.toml`
/// If there is no file, it will create a blank file.
pub fn get_config() -> Result<HashMap<String, String>, String> {
    let path: String = format!("{}/mpvy/config.toml", dirs::config_dir().unwrap().display());

    // Check if config file exists
    if fs::exists(&path).unwrap_or(false) == false {
        // If there is no configuration file, create a new empty file.
        let result: Result<(), std::io::Error> = fs::write(&path, "");

        if result.is_err() {
            error(
                "Mpvy Config",
                "Unable to write empty configuration file. Please create the file manually.",
            );
            return Err("Unable to write empty configuration file.".to_string());
        }
    }

    let config: Config = Config::builder()
        .add_source(config::File::with_name(&path))
        .add_source(config::Environment::with_prefix("MPVY")) // Allow users to pass config values as environment values
        .build()
        .unwrap();

    // Transform config into a HashMap
    let result = config.try_deserialize::<HashMap<String, String>>();

    if result.is_err() {
        error(
            "Mpvy Config",
            "An error occured while converting config into an HashMap.",
        );
        return Err("Unable to convert config into an HashMap.".to_string());
    }

    return Ok(result.unwrap());
}
