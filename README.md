# Simple Logger

A lightweight logging library for Rust that supports file rotation. This library provides a straightforward way to log messages to a file with automatic file rotation based on size.

## Features

- **Log Levels**: Supports INFO, WARN, and ERROR log levels.
- **File Rotation**: Automatically rotates the log file when it exceeds a specified size.
- **Reinitialization**: Allows reinitialization of the logger with a new file path and size limit.

## Installation

1. Save the `logger.rs` file in your project's `src` directory or another directory of your choice.
2. Include the following line in your `main.rs` or `lib.rs` file to use the logger:

    ```rust
    mod logger;
    ```

## Usage

### Initialization

Initialize the logger with a file path and maximum size for the log file:

```rust
use crate::logger::Logger;

fn main() {
    Logger::init("app.log", 1024 * 1024); // Rotate after 1MB

    log_info!("Application started");
    log_warn!("This is a warning");
    log_error!("This is an error");

    Logger::reinit("new_app.log", 2 * 1024 * 1024); // Rotate after 2MB

    log_info!("Logger reinitialized");
    log_warn!("This is a warning after reinit");
    log_error!("This is an error after reinit");
}
