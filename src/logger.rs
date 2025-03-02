use crate::level::LogLevel;
use chrono::Local;
use std::fmt::Arguments;

/// A simple logger for synchronous logging.
pub struct Logger;

impl Logger {
    /// Logs a message with the specified log level.
    ///
    /// # Arguments
    ///
    /// * `level` - The log level.
    /// * `args` - The message to log.
    pub fn log(&self, level: LogLevel, args: Arguments) {
        let now = Local::now();
        let time_str = now.format("%H:%M:%S").to_string();
        let level_str = level.to_colored_string();
        println!("[{}] {}: {}", time_str, level_str, args);
    }

    /// Logs an informational message.
    ///
    /// # Arguments
    ///
    /// * `args` - The message to log.
    pub fn info(&self, args: Arguments) {
        self.log(LogLevel::INFO, args);
    }

    /// Logs a warning message.
    ///
    /// # Arguments
    ///
    /// * `args` - The message to log.
    pub fn warn(&self, args: Arguments) {
        self.log(LogLevel::WARN, args);
    }

    /// Logs an error message.
    ///
    /// # Arguments
    ///
    /// * `args` - The message to log.
    pub fn error(&self, args: Arguments) {
        self.log(LogLevel::ERROR, args);
    }
}
