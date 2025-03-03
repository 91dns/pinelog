use crate::level::LogLevel;

use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub enum Timestamp {
    DATE,
    TIME,
    FULL,
}

#[derive(Deserialize)]
pub struct Settings {
    pub min_level: LogLevel,
    pub file_path: Option<String>,
    pub timestamp: Option<Timestamp>,
}

impl Settings {
    /// Load the logger settings from a TOML file.
    ///
    ///  # Arguments
    ///
    /// * `settings_path`- The path to the TOML settings file.
    pub fn load(settings_path: &str) -> Self {
        let settings_str = fs::read_to_string(settings_path).expect("Failed to read settings file");
        toml::from_str(&settings_str).expect("Failed to parse settings file")
    }
}
