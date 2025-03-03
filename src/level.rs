use colored::*;
use std::fmt;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
/// Represents the different log levels.
pub enum LogLevel {
    /// Informational messages.
    INFO,
    /// Warning messages.
    WARN,
    /// Error messages.
    ERROR,
}

impl LogLevel {
    /// Converts the log level to a colored string.
    ///
    /// # Returns
    ///
    /// A `ColoredString` representing the log level.
    pub fn to_colored_string(&self) -> ColoredString {
        match self {
            LogLevel::INFO => "INFO".green(),
            LogLevel::WARN => "WARN".yellow(),
            LogLevel::ERROR => "ERROR".red(),
        }
    }
}

impl fmt::Display for LogLevel {
    /// Formats the log level as a plain string.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter.
    ///
    /// # Returns
    ///
    /// A `fmt::Result`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::INFO => write!(f, "INFO"),
            LogLevel::WARN => write!(f, "WARN"),
            LogLevel::ERROR => write!(f, "ERROR"),
        }
    }
}
