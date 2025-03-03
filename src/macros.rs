/// Logs an informational message.
///
/// # Arguments
///
/// * `arg` - The message to log.
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        {
            let mut logger = $crate::logger::SYNC_LOGGER.lock().unwrap();
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                logger.info(format_args!($($arg)*)).await;
            });
        }
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
        {
            let mut logger = $crate::logger::SYNC_LOGGER.lock().unwrap();
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                logger.warn(format_args!($($arg)*)).await;
            });
        }
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
        {
            let mut logger = $crate::logger::SYNC_LOGGER.lock().unwrap();
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                logger.error(format_args!($($arg)*)).await;
            });
        }
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
            let mut logger = $crate::logger::ASYNC_LOGGER.lock().await;
            logger.info(format_args!($($arg)*)).await;
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
            let mut logger = $crate::logger::ASYNC_LOGGER.lock().await;
            logger.warn(format_args!($($arg)*)).await;
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
            let mut logger = $crate::logger::ASYNC_LOGGER.lock().await;
            logger.error(format_args!($($arg)*)).await;
        }
    };
}
