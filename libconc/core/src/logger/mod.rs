// ANSI color codes
const RESET: &str = "\x1b[0m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";

#[derive(Debug)]
pub enum LogLevel {
    Info,
    Debug,
    Warn,
    Error,
}

pub fn log(level: LogLevel, message: &str) {
    let (color, label) = match level {
        LogLevel::Info => (GREEN, "INFO"),
        LogLevel::Debug => (BLUE, "DEBUG"),
        LogLevel::Warn => (YELLOW, "WARN"),
        LogLevel::Error => (RED, "ERROR"),
    };

    println!("{}[{}]{} {}", color, label, RESET, message);
}

pub fn info(msg: &str)  { log(LogLevel::Info, msg); }
pub fn debug(msg: &str) { log(LogLevel::Debug, msg); }
pub fn warn(msg: &str)  { log(LogLevel::Warn, msg); }
pub fn error(msg: &str) { log(LogLevel::Error, msg); }