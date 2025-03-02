### `README.md`

```md
# Pinelog

Pinelog is a simple logging library for Rust, designed to be used in both synchronous and asynchronous contexts.

## Features

- INFO (Green), WARN (Yellow), ERROR (Red) levels.
- Supports formatted strings.
- Output format: `[TIME] LOGLEVEL(COLOR) project_name: "Message"`

## Usage

Add Pine to your `Cargo.toml`:

```toml
[dependencies]
pinelog = "0.2.0"
```

Initialize the logger and use the logging macros:

```rust
use pinelog::{init, info, warn, error};

#[tokio::main]
async fn main() {
    pine::init("my_project").await;

    info!("This is an info message.").await;
    warn!("This is a warning message.").await;
    error!("This is an error message.").await;

    info!("This is a formatted info message: {}", 42).await;
    warn!("This is a formatted warning message: {}", 42).await;
    error!("This is a formatted error message: {}", 42).await;
}
```

## License

Pinelog is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.
