use crate::log::*;
use config::Config;
use dirs;
use std::collections::HashMap;
use std::fs;

// CONFIGURATION KEYS
pub const MAX_FILE_COUNT: &str = "max_file_count";
pub const AUDIO_QUALITY: &str = "audio_quality";
pub const CONCURRENT_FRAGMENTS: &str = "concurrent_fragments";

pub fn get_config() -> Result<HashMap<String, String>, String> {
    let path = format!("{}/mpvy/config.toml", dirs::config_dir().unwrap().display());

    if fs::exists(&path).unwrap_or(false) == false {
        let result = fs::write(&path, "");

        if result.is_err() {
            error(
                "Mpvy Config".to_string(),
                "Unable to write empty configuration file. Please create the file manually."
                    .to_string(),
            );
            return Err("Unable to write empty configuration file".to_string());
        }
    }

    let config = Config::builder()
        .add_source(config::File::with_name(&path))
        .add_source(config::Environment::with_prefix("MPVY")) // Allow users to pass config values as environment values
        .build()
        .unwrap();

    let result = config.try_deserialize::<HashMap<String, String>>();

    if result.is_err() {
        error(
            "Mpvy Config".to_string(),
            "An error occured while converting config into an HashMap".to_string(),
        );
        return Err("Unable to convert config into an HashMap".to_string());
    }

    return Ok(result.unwrap());
}
