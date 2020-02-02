#![deny(warnings)]
#![deny(missing_docs)]

//! A logger configured via an environment variable which writes cancer to
//! standard error with colored output for logs.
//!
//! ## Example
//!
//! ```
//! # extern crate loge;
//! # #[macro_use] extern crate log;
//!
//! # use std::env;
//!
//! env::set_var("RUST_LOG", "trace");
//! env::set_var("LOGE_FORMAT", "target");
//! loge::init();
//!
//! trace!("this is trace level");
//! debug!("meet a note");
//! info!("everything is normal");
//! warn!("be careful");
//! error!("something error");
//! ```

#[cfg(windows)]
extern crate atty;
#[cfg(feature = "chrono")]
extern crate chrono;
#[cfg(feature = "colored")]
extern crate colored;
#[cfg(feature = "json")]
extern crate json_color;
extern crate log;
#[cfg(feature = "json")]
extern crate serde_json;
#[cfg(windows)]
extern crate winapi;

#[cfg(feature = "chrono")]
use chrono::Local;
#[cfg(feature = "colored")]
use colored::{ColoredString, Colorize};
use log::{Level, Log, Metadata, Record, SetLoggerError};

struct LogeLogger {
    level: Level,
    config: LogeFormat,
}

/// `LogeFormat` is used to choose log format.
pub enum LogeFormat {
    /// log format with `record.target()`
    Target,
    /// log format with `record.file()` and `record.line()`
    Fileline,
    /// json log format
    #[cfg(feature = "json")]
    Json,
}

impl Log for LogeLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        let level = record.level();
        let target = if !record.target().is_empty() {
            record.target()
        } else {
            record.module_path().unwrap_or_default()
        };
        let file = record.file().unwrap_or("<unknown>");
        let line = record.line().map_or(-1, |v| v as i32);
        let msg = error_trace_message(record);
        #[cfg(feature = "colored")]
        {
            let color_level = colored_level(record.level());
            match self.config {
                LogeFormat::Target => {
                    #[cfg(feature = "chrono")]
                    {
                        let time = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
                        println!("{} [{:<5}] {} ... {}", time, color_level, target.bold(), msg)
                    }
                    #[cfg(not(feature = "chrono"))]
                    {
                        println!("{:<5} {} ... {}", color_level, target.bold(), msg)
                    }
                }
                LogeFormat::Fileline => {
                    #[cfg(feature = "chrono")]
                    {
                        let time = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
                        println!(
                            "{} [{:<5}] {} -  (line {}) ... {}",
                            time,
                            color_level,
                            file.bold(),
                            line.to_string().magenta(),
                            msg
                        )
                    }
                    #[cfg(not(feature = "chrono"))]
                    {
                        println!(
                            "{:<5} {} -  (line {}) ... {}",
                            color_level,
                            file.bold(),
                            line.to_string().magenta(),
                            msg
                        )
                    }
                }
                #[cfg(feature = "json")]
                LogeFormat::Json => {
                    use json_color::{Color, Colorizer};
                    use serde_json::json;

                    let colorizer = Colorizer::new()
                        .number(Color::Magenta)
                        .string(Color::Green)
                        .key(Color::Cyan)
                        .build();

                    let time = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
                    // Get crate name from env.
                    let name = ::std::env::var("SERVICE_NAME")
                        .or_else(|_| ::std::env::var("CARGO_PKG_NAME"))
                        .unwrap_or_else(|_| String::new());
                    // Get crate version from env.
                    let version = ::std::env::var("SERVICE_VERSION")
                        .or_else(|_| ::std::env::var("CARGO_PKG_VERSION"))
                        .unwrap_or_else(|_| String::new());

                    let jsonified = json!({
                        "time" : time,
                        "level" : level.to_string(),
                        "message" : msg,
                        "service" : json!({
                            "name" : name,
                            "version" : version,
                        }),
                        "location" : json!({
                            "file" : file,
                            "line" : line,
                            "target" : target,
                        }),
                    });

                    if let Ok(jsonified_log) = colorizer.colorize_json_str(&jsonified.to_string()) {
                        println!("{}", jsonified_log);
                    }
                }
            }
        }
        #[cfg(not(feature = "colored"))]
        {
            match self.config {
                LogeFormat::Target => {
                    #[cfg(feature = "chrono")]
                    {
                        let time = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
                        println!("{} [{:<5}] {} ... {}", time, level, target, msg)
                    }
                    #[cfg(not(feature = "chrono"))]
                    {
                        println!("{:<5} {} ... {}", level, target, msg)
                    }
                }
                LogeFormat::Fileline => {
                    #[cfg(feature = "chrono")]
                    {
                        let time = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
                        println!("{} [{:<5}] {} -  (line {}) ... {}", time, level, file, line, msg)
                    }
                    #[cfg(not(feature = "chrono"))]
                    {
                        println!("{:<5} {} -  (line {}) ... {}", level, file, line, msg)
                    }
                }
                #[cfg(feature = "json")]
                LogeFormat::Json => {
                    use serde_json::json;

                    let time = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
                    // Get crate name from env.
                    let name = ::std::env::var("SERVICE_NAME")
                        .or_else(|_| ::std::env::var("CARGO_PKG_NAME"))
                        .unwrap_or_else(|_| String::new());
                    // Get crate version from env.
                    let version = ::std::env::var("SERVICE_VERSION")
                        .or_else(|_| ::std::env::var("CARGO_PKG_VERSION"))
                        .unwrap_or_else(|_| String::new());

                    let jsonified = json!({
                        "time" : time,
                        "level" : level.to_string(),
                        "message" : msg,
                        "service" : json!({
                            "name" : name,
                            "version" : version,
                        }),
                        "location" : json!({
                            "file" : file.to_string(),
                            "line" : line.to_string(),
                            "target" : target.to_string(),
                        }),
                    });

                    println!("{}", jsonified);
                }
            }
        }
    }

    fn flush(&self) {}
}

// Set up color terminal for windows.
#[cfg(windows)]
fn set_up_color_terminal() {
    use atty::Stream;

    if atty::is(Stream::Stdout) {
        unsafe {
            use winapi::um::consoleapi::*;
            use winapi::um::handleapi::*;
            use winapi::um::processenv::*;
            use winapi::um::winbase::*;
            use winapi::um::wincon::*;

            let stdout = GetStdHandle(STD_OUTPUT_HANDLE);

            if stdout == INVALID_HANDLE_VALUE {
                return;
            }

            let mut mode: winapi::shared::minwindef::DWORD = 0;

            if GetConsoleMode(stdout, &mut mode) == 0 {
                return;
            }

            SetConsoleMode(stdout, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
        }
    }
}

// Provide color for `level`.
#[cfg(feature = "colored")]
fn colored_level(level: Level) -> ColoredString {
    match level {
        Level::Trace => level.to_string().cyan(),
        Level::Debug => level.to_string().blue(),
        Level::Info => level.to_string().green(),
        Level::Warn => level.to_string().yellow(),
        Level::Error => level.to_string().red(),
    }
}

// Error messages also have a pseudo stack trace.
fn error_trace_message(record: &log::Record) -> String {
    match record.level() {
        Level::Error => format!(
            "{} -> {}:{}",
            record.args(),
            record.file().unwrap_or("<unknown>"),
            record.line().map_or(-1, |v| v as i32)
        ),
        _ => format!("{}", record.args()),
    }
}

// Parse level filters from `&str`.
fn parse_filters(environment_variable_name: &str) -> Level {
    match environment_variable_name {
        "trace" => Level::Trace,
        "debug" => Level::Debug,
        "info" => Level::Info,
        "warn" => Level::Warn,
        "error" => Level::Error,
        _ => Level::Error,
    }
}

// Parse format filters from `&str`.
fn parse_formats(environment_variable_name: &str) -> LogeFormat {
    match environment_variable_name {
        "target" => LogeFormat::Target,
        "fileline" => LogeFormat::Fileline,
        #[cfg(feature = "json")]
        "json" => LogeFormat::Json,
        _ => LogeFormat::Target,
    }
}

/// Initializes the global logger.
///
/// This should be called early in the execution of a Rust program, and the
/// global logger may only be initialized once. Future initialization attempts
/// will return an error.
///
/// # Panics
///
/// This function fails to set the global logger if one has already been set.
#[inline]
pub fn init() {
    try_init().unwrap();
}

/// Initializes the global logger with a logger named `loge`.
///
/// This should be called early in the execution of a Rust program, and the
/// global logger may only be initialized once. Future initialization attempts
/// will return an error.
///
/// # Errors
///
/// This function fails to set the global logger if one has already been set.
pub fn try_init() -> Result<(), log::SetLoggerError> {
    try_init_custom_env("RUST_LOG", "LOGE_FORMAT")
}

/// Initialized the global logger with a logger named `loge`, with a custom config
///
/// This should be called early in the execution of a Rust program, and the
/// global logger may only be initialized once. Future initialization attempts
/// will return an error.
///
/// # Panics
///
/// This function fails to set the global logger if one has already been set.
pub fn init_custom(level: Level, config: LogeFormat) {
    try_init_custom(level, config).unwrap();
}

/// Initialized the global logger with a logger named `loge`, with a custom config
///
/// This should be called early in the execution of a Rust program, and the
/// global logger may only be initialized once. Future initialization attempts
/// will return an error.
///
/// # Errors
///
/// This function fails to set the global logger if one has already been set.
pub fn try_init_custom(level: Level, config: LogeFormat) -> Result<(), SetLoggerError> {
    #[cfg(all(windows, feature = "colored"))]
    set_up_color_terminal();

    let logger = LogeLogger { level, config };
    log::set_max_level(level.to_level_filter());
    log::set_boxed_logger(Box::new(logger))?;
    Ok(())
}

/// Initialized the global logger with a logger named `loge`, with a custom custom variable
/// name.
///
/// This should be called early in the execution of a Rust program, and the
/// global logger may only be initialized once. Future initialization attempts
/// will return an error.
///
/// # Panics
///
/// This function fails to set the global logger if one has already been set.
pub fn init_custom_env(environment_variable_log: &str, environment_variable_format: &str) {
    try_init_custom_env(environment_variable_log, environment_variable_format).unwrap();
}

/// Initialized the global logger with a logger named `loge`, with a custom custom variable
/// name.
///
/// This should be called early in the execution of a Rust program, and the
/// global logger may only be initialized once. Future initialization attempts
/// will return an error.
///
/// # Errors
///
/// This function fails to set the global logger if one has already been set.
pub fn try_init_custom_env(
    environment_variable_log: &str,
    environment_variable_format: &str,
) -> Result<(), SetLoggerError> {
    #[cfg(all(windows, feature = "colored"))]
    set_up_color_terminal();

    if let Ok(s1) = ::std::env::var(environment_variable_log) {
        if let Ok(s2) = ::std::env::var(environment_variable_format) {
            let level = parse_filters(&s1);
            let config = parse_formats(&s2);

            try_init_custom(level, config).unwrap();
        }
    }
    Ok(())
}
