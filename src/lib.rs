use chrono::Local;
use lazy_static::lazy_static;
use std::fmt;
use std::sync::Arc;
use std::sync::Mutex;

lazy_static! {
    static ref LOGGER: Arc<Mutex<Pinelog>> = Arc::new(Mutex::new(Pinelog::new("DefaultProject")));
}

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
        let now = Local::now();
        let timestamp = now.format("%H:%M:%S");
        println!(
            "[{}] {} {}: {}",
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

pub fn init(project_name: &str) {
    let mut logger = LOGGER.lock().unwrap();
    *logger = Pinelog::new(project_name);
}

pub fn get_logger() -> Arc<Mutex<Pinelog>> {
    LOGGER.clone()
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        let logger = $crate::get_logger();
        let logger = logger.lock().unwrap();
        logger.info(&format!($($arg)*));
    }}
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {{
        let logger = $crate::get_logger();
        let logger = logger.lock().unwrap();
        logger.warn(&format!($($arg)*));
    }}
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        let logger = $crate::get_logger();
        let logger = logger.lock().unwrap();
        logger.error(&format!($($arg)*));
    }}
}
