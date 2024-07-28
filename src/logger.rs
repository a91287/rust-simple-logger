// src/logger.rs

use std::fs::{OpenOptions, File, metadata, rename};
use std::io::Write;
use std::sync::RwLock;
use std::time::{SystemTime, UNIX_EPOCH};

enum LogLevel {
    Info,
    Warn,
    Error,
}

impl LogLevel {
    fn as_str(&self) -> &str {
        match self {
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
        }
    }
}

pub struct Logger {
    file: RwLock<File>,
    file_path: String,
    max_size: u64,
}

impl Logger {
    fn new(file_path: &str, max_size: u64) -> Self {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path)
            .unwrap();
        Logger {
            file: RwLock::new(file),
            file_path: file_path.to_string(),
            max_size,
        }
    }

    pub fn init(file_path: &str, max_size: u64) {
        unsafe {
            LOGGER = Some(Logger::new(file_path, max_size));
        }
    }

    pub fn reinit(file_path: &str, max_size: u64) {
        unsafe {
            if let Some(ref mut logger) = LOGGER {
                let new_file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(file_path)
                    .unwrap();
                let mut file = logger.file.write().unwrap();
                *file = new_file;
                logger.file_path = file_path.to_string();
                logger.max_size = max_size;
            } else {
                Logger::init(file_path, max_size);
            }
        }
    }

    fn rotate_file(&self) {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let new_path = format!("{}.{}", self.file_path, timestamp);
        rename(&self.file_path, &new_path).unwrap();
        let new_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)
            .unwrap();
        let mut file = self.file.write().unwrap();
        *file = new_file;
    }

    fn log(&self, level: LogLevel, msg: &str) {
        let mut file = self.file.write().unwrap();
        let file_size = metadata(&self.file_path).unwrap().len();
        if file_size >= self.max_size {
            drop(file); // Explicitly drop the file lock before rotating
            self.rotate_file();
            file = self.file.write().unwrap(); // Reacquire the file lock after rotating
        }
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        writeln!(file, "[{}][{}] {}", timestamp, level.as_str(), msg).unwrap();
    }

    pub fn info(msg: &str) {
        unsafe {
            if let Some(ref logger) = LOGGER {
                logger.log(LogLevel::Info, msg);
            }
        }
    }

    pub fn warn(msg: &str) {
        unsafe {
            if let Some(ref logger) = LOGGER {
                logger.log(LogLevel::Warn, msg);
            }
        }
    }

    pub fn error(msg: &str) {
        unsafe {
            if let Some(ref logger) = LOGGER {
                logger.log(LogLevel::Error, msg);
            }
        }
    }
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        Logger::info(&format!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        Logger::warn(&format!($($arg)*));
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
    Logger::error(&format!($($arg)*));
    };
}

static mut LOGGER: Option<Logger> = None;

#[cfg(test)]
mod tests {
    use super::*;

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
}
