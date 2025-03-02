use colored::*;

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
