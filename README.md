# Pinelog

Pinelog is a simple logging library for Rust, designed to be used in both synchronous and asynchronous contexts.

## Features

- Global logger instance.
- Multiple log levels.
- Supports formatted strings.
- Configurable minimum log level.
- Optional log file support.
- Output format: `[TIMESTAMP](optional) LOGLEVEL(COLOR): "Message"`
- Asynchronous logging support with `tokio`.
- **Thread-safe**: Safe to use in multi-threaded environments.
- Configurable via a TOML settings file.

## Usage

Add Pinelog to your `Cargo.toml`:

```toml
[dependencies]
pinelog = "0.1.0"
```

Alternatively, you can add Pinelog using the `cargo add` command:

```sh
cargo add pinelog
```

### Synchronous Logging

Initialize the global logger with a minimum log level and optional log file path:

```rust
use pinelog::prelude::*;

fn main() {
    // Initialize the logger with settings from a TOML file
    Pinelog::init_sync("settings.toml");

    info!("This is an info message.");
    warn!("This is a warning message.");
    error!("This is an error message.");

    info!("This is a formatted info message: {}", 42);
    warn!("This is a formatted warning message: {}", 42);
    error!("This is a formatted error message: {}", 42);
}
```

### Asynchronous Logging

> **Note:** Asynchronous logging is useful in scenarios where you have an asynchronous application, such as a web server or any application that performs I/O-bound tasks. Using asynchronous logging helps to avoid blocking the main thread, ensuring that your application remains responsive.

Initialize the global asynchronous logger with a minimum log level and optional log file path:

```rust
use pinelog::prelude::*;
use tokio;

#[tokio::main]
async fn main() {
    // Initialize the asynchronous logger with settings from a TOML file
    Pinelog::init_async("settings.toml").await;

    async_info!("This is an async info message").await;
    async_warn!("This is an async warning message").await;
    async_error!("This is an async error message").await;

    async_info!("This is a formatted async info message: {}", 42).await;
    async_warn!("This is a formatted async warning message: {}", 42).await;
    async_error!("This is a formatted async error message: {}", 42).await;
}
```

### Example Settings File

You must configure the logger using a TOML settings file. Create a `settings.toml` file with the following content:

```toml
min_level = "INFO"        # Options: INFO, WARN, ERROR (mandatory)
file_path = "logfile.log" # (optional)
timestamp = "FULL"        # Options: FULL, TIME, DATE (optional)
```

In this example, the `file_path` field is optional. If you do not want to log to a file, you can omit this field.

> **Note:** settings.toml needs to be created in root of your project folder. You can name the file whatever you want.

## Dependencies

Pinelog depends on the following crates:

```toml
[dependencies]
chrono = "0.4.40"
colored = "3.0.0"
lazy_static = "1.5.0"
tokio = { version = "1.43.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
toml = "0.5.8"
```

## License

Pinelog is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.
