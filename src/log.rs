use dirs;
use std::fs::OpenOptions;
use std::io::{Result, Write};

/// Function to write a log entry to a log file with a given level, module, and message
fn write_log(level: &str, module: &str, message: &str) -> Result<()> {
    if let Some(config_dir) = dirs::config_dir() {
        let log_dir = config_dir.join("mpvy").join("log");
        if !log_dir.exists() {
            std::fs::create_dir_all(&log_dir)?;
        }
        let log_file_path = log_dir.join("mpvy.log");
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_file_path)?;
        writeln!(file, "[{}] ({}) --> {}", level, module, message)?;
    }
    Ok(())
}

/// Log an error message for a given module
pub fn error(module: String, message: String) {
    if let Err(e) = write_log("ERROR", &module, &message) {
        eprintln!("Failed to write error log: {}", e);
    }
}

/// Log a warning message for a given module
pub fn warning(module: String, message: String) {
    if let Err(e) = write_log("WARNING", &module, &message) {
        eprintln!("Failed to write warning log: {}", e);
    }
}

/// Log an informational message for a given module
pub fn info(module: String, message: String) {
    if let Err(e) = write_log("INFO", &module, &message) {
        eprintln!("Failed to write info log: {}", e);
    }
}
