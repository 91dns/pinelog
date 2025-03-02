### `README.md`

```md
# Pinelog

Pinelog is a simple logging library for Rust, designed to be used in a synchronous context.

## Features

- INFO, WARN, ERROR levels.
- Supports formatted strings.
- Output format: `[TIME] LOGLEVEL(COLOR) project_name: "Message"`

## Usage

Add Pine to your `Cargo.toml`:

```toml
[dependencies]
pinelog = "0.1.0"
```

Initialize the logger and use the logging macros:

```rust
use pinelog::{init, info, warn, error};

fn main() {
    pine::init("my_project").await;

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
