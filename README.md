# Pinelog

Pinelog is a simple logging library for Rust, designed to be used in a synchronous context.

## Features

- INFO, WARN, ERROR levels.
- Supports formatted strings.
- Output format: `[TIME] LOGLEVEL(COLOR): "Message"`

## Usage

Add Pinelog to your `Cargo.toml`:

```toml
[dependencies]
pinelog = "0.1.0"
```

Use the logging macros in your code:

```rust
use pinelog::{info, warn, error};

fn main() {
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
