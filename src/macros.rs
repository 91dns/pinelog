/// Logs an informational message.
///
/// # Arguments
///
/// * `arg` - The message to log.
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::logger::Logger.info(format_args!($($arg)*));
    };
}

/// Logs a warning message.
///
/// # Arguments
///
/// * `arg` - The message to log.
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::logger::Logger.warn(format_args!($($arg)*));
    };
}

/// Logs an error message.
///
/// # Arguments
///
/// * `arg` - The message to log.
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::logger::Logger.error(format_args!($($arg)*));
    };
}
