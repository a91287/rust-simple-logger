# Simple Logger

## Overview

This repository provides a basic logging mechanism implemented in Rust. The logging system includes features such as file rotation and different log levels (Info, Warn, Error). This README explains how to set up and use the logger.

## Features

- **Log Levels**: `INFO`, `WARN`, `ERROR`
- **File Rotation**: Automatically rotates log files based on size
- **Thread-Safe**: Uses `RwLock` to ensure thread safety

## How It Works

The `logger.rs` file defines a `Logger` struct and several macros to facilitate logging at different levels. The logger writes messages to a file, which is rotated when it exceeds a specified size.

### Key Components

- **`Logger` Struct**: Manages logging operations and file handling.
- **Log Levels**: Enum `LogLevel` defines the log levels used.
- **Macros**: `log_info!`, `log_warn!`, and `log_error!` for convenience in logging messages.

## Usage

### 1. Initialize the Logger

Before using the logger, you need to initialize it with a file path and a maximum file size:

```rust
Logger::init("path/to/logfile.log", 1024 * 1024); // Rotate after 1MB
```

### 2. Logging Messages
Use the provided macros to log messages at different levels:

```rust
log_info!("This is an info message");
log_warn!("This is a warning message");
log_error!("This is an error message");
```

### 3. Reinitialize the Logger

You can reinitialize the logger if you need to change the log file or its size:

```rust
Logger::reinit("path/to/new_logfile.log", 2 * 1024 * 1024); // Rotate after 2MB
```

## Example
Here is a simple example using the logger in a Rust application:

```rust
// src/main.rs

use rust_simple_logger::logger::Logger; // Import the Logger struct
use rust_simple_logger::{log_info, log_warn, log_error}; // Import the macros

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

```

## Testing
The provided test function in logger.rs demonstrates basic logging functionality and verifies that the logger can handle file rotation:

```rust
#[test]
fn test_logging() {
    Logger::init("test.log", 1024); // Rotate after 1KB
    for i in 0..1000 {
        log_info!("This is an info message {}", i);
    }

    Logger::reinit("new_test.log", 2048); // Rotate after 2KB
    for i in 0..1000 {
        log_info!("This is another info message {}", i);
    }
}
```