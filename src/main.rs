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
