use crate::level::LogLevel;

use chrono::Local;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::{fmt::Arguments, fs::OpenOptions, io::Write};

lazy_static! {
    pub static ref LOGGER: Mutex<Pinelog> = Mutex::new(Pinelog::new(LogLevel::INFO, None));
}

/// A simple logger for synchronous logging.
pub struct Pinelog {
    min_level: LogLevel,
    file: Option<std::fs::File>,
}

impl Pinelog {
    /// Creates a new logger with the specified minimum log level.
    ///
    /// # Arguments
    ///
    /// * `min_level` - The minimum log level to log.
    /// * `file_path` - The path to the log file, if any.
    pub fn new(min_level: LogLevel, file_path: Option<&str>) -> Self {
        let file = file_path.map(|path| {
            OpenOptions::new()
                .append(true)
                .create(true)
                .open(path)
                .unwrap()
        });

        Self { min_level, file }
    }

    /// Initializes the global logger with the specified minimum log level and optional file path.
    ///
    /// # Arguments
    ///
    /// * `min_level` - The minimum log level to log.
    /// * `file_path` - The path to the log file, if any.
    pub fn init(min_level: LogLevel, file_path: Option<&str>) {
        let mut logger = LOGGER.lock().unwrap();
        *logger = Pinelog::new(min_level, file_path);
    }

    /// Logs a message with the specified log level.
    ///
    /// # Arguments
    ///
    /// * `level` - The log level.
    /// * `args` - The message to log.
    pub fn log(&mut self, level: LogLevel, args: Arguments) {
        if level >= self.min_level {
            let now = Local::now();
            let time_str = now.format("%H:%M:%S").to_string();
            let level_str = level.to_colored_string();
            println!("{} [{}] {}", time_str, level_str, args);

            // Log to file if provided
            if let Some(ref mut file) = self.file {
                writeln!(file, "{} [{}] {}", time_str, level, args).unwrap();
            }
        }
    }

    /// Logs an informational message.
    ///
    /// # Arguments
    ///
    /// * `args` - The message to log.
    pub fn info(&mut self, args: Arguments) {
        self.log(LogLevel::INFO, args);
    }

    /// Logs a warning message.
    ///
    /// # Arguments
    ///
    /// * `args` - The message to log.
    pub fn warn(&mut self, args: Arguments) {
        self.log(LogLevel::WARN, args);
    }

    /// Logs an error message.
    ///
    /// # Arguments
    ///
    /// * `args` - The message to log.
    pub fn error(&mut self, args: Arguments) {
        self.log(LogLevel::ERROR, args);
    }
}
