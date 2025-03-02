use std::fmt;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex;

static LOGGER: once_cell::sync::Lazy<Arc<Mutex<Option<Pinelog>>>> =
    once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(None)));

pub struct Pinelog {
    project_name: String,
}

impl Pinelog {
    pub fn new(project_name: &str) -> Self {
        Self {
            project_name: project_name.to_string(),
        }
    }

    pub fn log(&self, level: LogLevel, message: &str) {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let timestamp = now.as_secs();
        println!(
            "[{}] {} {}: \"{}\"",
            timestamp, level, self.project_name, message
        );
    }

    pub fn info(&self, message: &str) {
        self.log(LogLevel::Info, message);
    }

    pub fn warn(&self, message: &str) {
        self.log(LogLevel::Warn, message);
    }

    pub fn error(&self, message: &str) {
        self.log(LogLevel::Error, message);
    }
}

#[derive(Debug)]
pub enum LogLevel {
    Info,
    Warn,
    Error,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let level_str = match self {
            LogLevel::Info => "\x1b[32mINFO\x1b[0m",   // Green
            LogLevel::Warn => "\x1b[33mWARN\x1b[0m",   // Yellow
            LogLevel::Error => "\x1b[31mERROR\x1b[0m", // Red
        };
        write!(f, "{}", level_str)
    }
}

pub async fn init(project_name: &str) {
    let mut logger = LOGGER.lock().await;
    *logger = Some(Pinelog::new(project_name));
}

pub async fn get_logger() -> Arc<Mutex<Option<Pinelog>>> {
    LOGGER.clone()
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => ({
        let logger = $crate::get_logger().await;
        let logger = logger.lock().await;
        if let Some(ref logger) = *logger {
            logger.info(&format!($($arg)*));
        }
    })
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => ({
        let logger = $crate::get_logger().await;
        let logger = logger.lock().await;
        if let Some(ref logger) = *logger {
            logger.warn(&format!($($arg)*));
        }
    })
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => ({
        let logger = $crate::get_logger().await;
        let logger = logger.lock().await;
        if let Some(ref logger) = *logger {
            logger.error(&format!($($arg)*));
        }
    })
}
