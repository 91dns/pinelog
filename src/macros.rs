/// Logs an informational message.
///
/// # Arguments
///
/// * `arg` - The message to log.
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::logger::LOGGER.lock().unwrap().info(format_args!($($arg)*));
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
        $crate::logger::LOGGER.lock().unwrap().warn(format_args!($($arg)*));
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
        $crate::logger::LOGGER.lock().unwrap().error(format_args!($($arg)*));
    };
}

/// Logs an informational message asynchronously.
///
/// # Arguments
///
/// * `arg` - The message to log.
#[macro_export]
macro_rules! async_info {
    ($($arg:tt)*) => {
        async {
            if let Some(ref mut logger) = *$crate::logger::ASYNC_LOGGER.lock().await {
                logger.info(format_args!($($arg)*)).await;
            }
        }
    };
}

/// Logs a warning message asynchronously.
///
/// # Arguments
///
/// * `arg` - The message to log.
#[macro_export]
macro_rules! async_warn {
    ($($arg:tt)*) => {
        async {
            if let Some(ref mut logger) = *$crate::logger::ASYNC_LOGGER.lock().await {
                logger.warn(format_args!($($arg)*)).await;
            }
        }
    };
}

/// Logs an error message asynchronously.
///
/// # Arguments
///
/// * `arg` - The message to log.
#[macro_export]
macro_rules! async_error {
    ($($arg:tt)*) => {
        async {
            if let Some(ref mut logger) = *$crate::logger::ASYNC_LOGGER.lock().await {
                logger.error(format_args!($($arg)*)).await;
            }
        }
    };
}
