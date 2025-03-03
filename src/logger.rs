use crate::level::LogLevel;
use crate::settings::{Settings, Timestamp};

use chrono::Local;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::{fmt::Arguments, fs::OpenOptions, io::Write};
use tokio::fs::OpenOptions as AsyncOpenOptions;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex as AsyncMutex;

lazy_static! {
    pub static ref SYNC_LOGGER: Mutex<Pinelog> =
        Mutex::new(Pinelog::new(LogLevel::INFO, None, None, Mode::Sync));
    pub static ref ASYNC_LOGGER: AsyncMutex<Pinelog> =
        AsyncMutex::new(Pinelog::new(LogLevel::INFO, None, None, Mode::Async));
}

/// Represents the mode of the logger (synchronous or asynchronous).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Mode {
    Sync,
    Async,
}

/// A simple logger that can handle both synchronous and asynchronous logging.
pub struct Pinelog {
    min_level: LogLevel,
    file: Option<std::fs::File>,
    async_file: Option<tokio::fs::File>,
    timestamp: Option<Timestamp>,
    mode: Mode,
}

impl Pinelog {
    /// Creates a new logger with the specified minimum log level and mode.
    ///
    /// # Arguments
    ///
    /// * `min_level` - The minimum log level to log.
    /// * `file_path` - The path to the log file, if any.
    /// * `timestamp` - The timestamp format to use, if any.
    /// * `mode` - The mode of the logger (synchronous or asynchronous).
    pub fn new(
        min_level: LogLevel,
        file_path: Option<&str>,
        timestamp: Option<Timestamp>,
        mode: Mode,
    ) -> Self {
        let file = if mode == Mode::Sync {
            file_path.map(|path| {
                OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(path)
                    .unwrap()
            })
        } else {
            None
        };

        let async_file = None;

        Self {
            min_level,
            file,
            async_file,
            timestamp,
            mode,
        }
    }

    /// Initializes the global synchronous logger with the specified minimum log level and optional file path.
    ///
    /// # Arguments
    ///
    /// * `min_level` - The minimum log level to log.
    /// * `file_path` - The path to the log file, if any.
    pub fn init_sync(settings_path: &str) {
        let settings = Settings::load(settings_path);
        let mut logger = SYNC_LOGGER.lock().unwrap();
        *logger = Pinelog::new(
            settings.min_level,
            settings.file_path.as_deref(),
            settings.timestamp,
            Mode::Sync,
        );
    }

    /// Initializes the global asynchronous logger with the specified minimum log level and optional file path.
    ///
    /// # Arguments
    ///
    /// * `min_level` - The minimum log level to log.
    /// * `file_path` - The path to the log file, if any.
    pub async fn init_async(settings_path: &str) {
        let settings = Settings::load(settings_path);
        let async_file = if let Some(path) = settings.file_path {
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

        let mut logger = ASYNC_LOGGER.lock().await;
        *logger = Pinelog {
            min_level: settings.min_level,
            file: None,
            async_file,
            timestamp: settings.timestamp,
            mode: Mode::Async,
        };
    }

    /// Logs a message with the specified log level.
    ///
    /// # Arguments
    ///
    /// * `level` - The log level.
    /// * `args` - The message to log.
    pub async fn log(&mut self, level: LogLevel, args: Arguments<'_>) {
        if level >= self.min_level {
            let now = Local::now();
            let time_str = match self.timestamp {
                Some(Timestamp::DATE) => now.format("%Y-%m-%d").to_string(),
                Some(Timestamp::TIME) => now.format("%H:%M:%S").to_string(),
                Some(Timestamp::FULL) => now.format("%Y-%m-%d %H:%M:%S").to_string(),
                None => String::new(),
            };
            let level_str = level.to_colored_string();
            println!("{} [{}] {}", time_str, level_str, args);

            match self.mode {
                Mode::Sync => {
                    if let Some(ref mut file) = self.file {
                        writeln!(file, "{} [{}] {}", time_str, level, args).unwrap();
                    }
                }
                Mode::Async => {
                    if let Some(ref mut async_file) = self.async_file {
                        async_file
                            .write_all(format!("{} [{}] {}\n", time_str, level, args).as_bytes())
                            .await
                            .unwrap();
                    }
                }
            }
        }
    }

    /// Logs an informational message.
    ///
    /// # Arguments
    ///
    /// * `args` - The message to log.
    pub async fn info(&mut self, args: Arguments<'_>) {
        self.log(LogLevel::INFO, args).await;
    }

    /// Logs a warning message.
    ///
    /// # Arguments
    ///
    /// * `args` - The message to log.
    pub async fn warn(&mut self, args: Arguments<'_>) {
        self.log(LogLevel::WARN, args).await;
    }

    /// Logs an error message.
    ///
    /// # Arguments
    ///
    /// * `args` - The message to log.
    pub async fn error(&mut self, args: Arguments<'_>) {
        self.log(LogLevel::ERROR, args).await;
    }
}
