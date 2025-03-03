# Pinelog

Pinelog is a simple logging library for Rust, designed to be used in a synchronous context.

## Features

- Global logger instance.
- Multiple log levels.
- Supports formatted strings.
- Configurable minimum log level.
- Optional log file support.
- Output format: `[TIME] LOGLEVEL(COLOR): "Message"`

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

Initialize the global logger with a minimum log level and optional log file path:

```rust
use pinelog::prelude::*;

fn main() {
    // Initialize the logger with minimum log level and no log file
    Pinelog::init(LogLevel::INFO, None);

    // Initialize the logger with minimum log level and a log file
    // Pinelog::init(LogLevel::WARN, Some("logfile.log"));

    info!("This is an info message.");
    warn!("This is a warning message.");
    error!("This is an error message.");

    info!("This is a formatted info message: {}", 42);
    warn!("This is a formatted warning message: {}", 42);
    error!("This is a formatted error message: {}", 42);
}
```

## License

Pinelog is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.
