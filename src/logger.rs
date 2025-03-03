use crate::level::LogLevel;

use chrono::Local;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::{fmt::Arguments, fs::OpenOptions, io::Write};
use tokio::fs::OpenOptions as AsyncOpenOptions;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex as AsyncMutex;

lazy_static! {
    pub static ref LOGGER: Mutex<Pinelog> = Mutex::new(Pinelog::new(LogLevel::INFO, None));
    pub static ref ASYNC_LOGGER: AsyncMutex<Option<AsyncPinelog>> = AsyncMutex::new(None);
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

/// A simple logger for asynchronous logging.
pub struct AsyncPinelog {
    min_level: LogLevel,
    file: Option<tokio::fs::File>,
}

impl AsyncPinelog {
    /// Creates a new asynchronous logger with the specified minimum log level.
    ///
    /// # Arguments
    ///
    /// * `min_level` - The minimum log level to log.
    /// * `file_path` - The path to the log file, if any.
    pub async fn new(min_level: LogLevel, file_path: Option<&str>) -> Self {
        let file = if let Some(path) = file_path {
            Some(
                AsyncOpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(path)
                    .await
                    .unwrap(),
            )
        } else {
            None
        };

        Self { min_level, file }
    }

    /// Initializes the global asynchronous logger with the specified minimum log level and optional file path.
    ///
    /// # Arguments
    ///
    /// * `min_level` - The minimum log level to log.
    /// * `file_path` - The path to the log file, if any.
    pub async fn init(min_level: LogLevel, file_path: Option<&str>) {
        let mut logger = ASYNC_LOGGER.lock().await;
        *logger = Some(AsyncPinelog::new(min_level, file_path).await);
    }

    /// Logs a message with the specified log level.
    ///
    /// # Arguments
    ///
    /// * `level` - The log level.
    /// * `args` - The message to log.
    pub async fn log<'a>(&mut self, level: LogLevel, args: Arguments<'a>) {
        if level >= self.min_level {
            let now = Local::now();
            let time_str = now.format("%H:%M:%S").to_string();
            let level_str = level.to_colored_string();
            println!("{} [{}] {}", time_str, level_str, args);

            // Log to file if provided
            if let Some(ref mut file) = self.file {
                file.write_all(format!("{} [{}] {}\n", time_str, level, args).as_bytes())
                    .await
                    .unwrap();
            }
        }
    }

    /// Logs an informational message.
    ///
    /// # Arguments
    ///
    /// * `args` - The message to log.
    pub async fn info<'a>(&mut self, args: Arguments<'a>) {
        self.log(LogLevel::INFO, args).await;
    }

    /// Logs a warning message.
    ///
    /// # Arguments
    ///
    /// * `args` - The message to log.
    pub async fn warn<'a>(&mut self, args: Arguments<'a>) {
        self.log(LogLevel::WARN, args).await;
    }

    /// Logs an error message.
    ///
    /// # Arguments
    ///
    /// * `args` - The message to log.
    pub async fn error<'a>(&mut self, args: Arguments<'a>) {
        self.log(LogLevel::ERROR, args).await;
    }
}
