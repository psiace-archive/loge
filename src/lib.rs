#![cfg_attr(test, deny(warnings))]
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
//! loge::init();
//! // Or loge::init_fileline();
//! // Or loge::init_jsonified();
//!
//! trace!("this is trace level");
//! debug!("meet a note");
//! info!("everything is normal");
//! warn!("be careful");
//! error!("something error");
//! ```

extern crate chrono;
pub extern crate env_logger;
extern crate json;
extern crate log;

use chrono::Local;
use env_logger::{
    fmt::{Color, Style, StyledValue},
    Builder,
};
use json::object;
use log::Level;
use std::{env, str};

fn colored_level<'a>(style: &'a mut Style, level: Level) -> StyledValue<'a, &'static str> {
    match level {
        Level::Trace => style.set_color(Color::Cyan).value("TRACE"),
        Level::Debug => style.set_color(Color::Blue).value("DEBUG"),
        Level::Info => style.set_color(Color::Green).value("INFO "),
        Level::Warn => style.set_color(Color::Yellow).value("WARN "),
        Level::Error => style.set_color(Color::Red).value("ERROR"),
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

/// Initializes the global logger with a logger named `loge`.
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

/// Initializes the global logger with a file line logger named `loge`.
///
/// This should be called early in the execution of a Rust program, and the
/// global logger may only be initialized once. Future initialization attempts
/// will return an error.
///
/// # Panics
///
/// This function fails to set the global logger if one has already been set.
#[inline]
pub fn init_fileline() {
    try_init_fileline().unwrap();
}

/// Initializes the global logger with a jsonified logger named `loge`.
///
/// This should be called early in the execution of a Rust program, and the
/// global logger may only be initialized once. Future initialization attempts
/// will return an error.
///
/// # Panics
///
/// This function fails to set the global logger if one has already been set.
#[inline]
pub fn init_jsonified() {
    try_init_jsonified().unwrap();
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
    try_init_custom_env("RUST_LOG")
}

/// Initializes the global logger with a file line logger named `loge`.
///
/// This should be called early in the execution of a Rust program, and the
/// global logger may only be initialized once. Future initialization attempts
/// will return an error.
///
/// # Errors
///
/// This function fails to set the global logger if one has already been set.
pub fn try_init_fileline() -> Result<(), log::SetLoggerError> {
    try_init_fileline_custom_env("RUST_LOG")
}

/// Initializes the global logger with a jsonified logger named `loge`.
///
/// This should be called early in the execution of a Rust program, and the
/// global logger may only be initialized once. Future initialization attempts
/// will return an error.
///
/// # Errors
///
/// This function fails to set the global logger if one has already been set.
pub fn try_init_jsonified() -> Result<(), log::SetLoggerError> {
    try_init_jsonified_custom_env("RUST_LOG")
}

/// Initialized the global logger with a logger named `loge`, with a custom variable
/// name.
///
/// This should be called early in the execution of a Rust program, and the
/// global logger may only be initialized once. Future initialization attempts
/// will return an error.
///
/// # Panics
///
/// This function fails to set the global logger if one has already been set.
pub fn init_custom_env(environment_variable_name: &str) {
    try_init_custom_env(environment_variable_name).unwrap();
}

/// Initialized the global logger with a logger named `loge`, with a custom variable
/// name.
///
/// This should be called early in the execution of a Rust program, and the
/// global logger may only be initialized once. Future initialization attempts
/// will return an error.
///
/// # Errors
///
/// This function fails to set the global logger if one has already been set.
pub fn try_init_custom_env(environment_variable_name: &str) -> Result<(), log::SetLoggerError> {
    let mut builder = formatted_builder();

    if let Ok(s) = ::std::env::var(environment_variable_name) {
        builder.parse_filters(&s);
    }

    builder.try_init()
}

/// Initialized the global logger with a file line logger named `loge`, with a custom variable
/// name.
///
/// This should be called early in the execution of a Rust program, and the
/// global logger may only be initialized once. Future initialization attempts
/// will return an error.
///
/// # Errors
///
/// This function fails to set the global logger if one has already been set.
pub fn try_init_fileline_custom_env(
    environment_variable_name: &str,
) -> Result<(), log::SetLoggerError> {
    let mut builder = formatted_fileline_builder();

    if let Ok(s) = ::std::env::var(environment_variable_name) {
        builder.parse_filters(&s);
    }

    builder.try_init()
}

/// Initialized the global logger with a jsonified logger named `loge`, with a custom variable
/// name.
///
/// This should be called early in the execution of a Rust program, and the
/// global logger may only be initialized once. Future initialization attempts
/// will return an error.
///
/// # Errors
///
/// This function fails to set the global logger if one has already been set.
pub fn try_init_jsonified_custom_env(
    environment_variable_name: &str,
) -> Result<(), log::SetLoggerError> {
    let mut builder = formatted_jsonified_builder();

    if let Ok(s) = ::std::env::var(environment_variable_name) {
        builder.parse_filters(&s);
    }

    builder.try_init()
}

/// Returns a `env_logger::Builder` for further customization.
///
/// This method will return a colored and formatted `env_logger::Builder`
/// for further customization. Refer to env_logger::Build crate documentation
/// for further details and usage.
///
/// This should be called early in the execution of a Rust program, and the
/// global logger may only be initialized once. Future initialization attempts
/// will return an error.
///
/// # Errors
///
/// This function fails to set the global logger if one has already been set.
pub fn formatted_builder() -> Builder {
    use std::io::Write;

    let mut builder = Builder::new();

    builder.format(|formatter, record| {
        let mut time_style = formatter.style();
        let mut level_style = formatter.style();
        let mut target_style = formatter.style();

        time_style.set_color(Color::Ansi256(59));
        target_style.set_bold(true);

        let msg = error_trace_message(record);

        writeln!(
            formatter,
            "{} [{}] {} ... {}",
            time_style.value(Local::now().format("%Y-%m-%d %H:%M:%S")),
            colored_level(&mut level_style, record.level()),
            target_style.value(record.target()),
            msg
        )
    });

    builder
}

/// Returns a `env_logger::Builder` for further customization.
///
/// This method will return a colored and file line formatted `env_logger::Builder`
/// for further customization. Refer to env_logger::Build crate documentation
/// for further details and usage.
///
/// This should be called early in the execution of a Rust program, and the
/// global logger may only be initialized once. Future initialization attempts
/// will return an error.
///
/// # Errors
///
/// This function fails to set the global logger if one has already been set.
pub fn formatted_fileline_builder() -> Builder {
    use std::io::Write;

    let mut builder = Builder::new();

    builder.format(|formatter, record| {
        let mut time_style = formatter.style();
        let mut level_style = formatter.style();
        let mut file_style = formatter.style();
        let mut line_style = formatter.style();

        time_style.set_color(Color::Ansi256(59));
        file_style.set_bold(true);
        line_style.set_color(Color::Magenta);

        writeln!(
            formatter,
            "{} [{}] {} -  (line {}) ... {}",
            time_style.value(Local::now().format("%Y-%m-%d %H:%M:%S")),
            colored_level(&mut level_style, record.level()),
            file_style.value(record.file().unwrap_or("<unknown>")),
            line_style.value(record.line().map_or(-1, |v| v as i32)),
            record.args()
        )
    });

    builder
}

/// Returns a `env_logger::Builder` for further customization.
///
/// This method will return a colored and jsonified formatted `env_logger::Builder`
/// for further customization. Refer to env_logger::Build crate documentation
/// for further details and usage.
///
/// This should be called early in the execution of a Rust program, and the
/// global logger may only be initialized once. Future initialization attempts
/// will return an error.
///
/// # Errors
///
/// This function fails to set the global logger if one has already been set.
pub fn formatted_jsonified_builder() -> Builder {
    use std::io::Write;

    let mut builder = Builder::new();

    builder.format(|formatter, record| {
        let time = Local::now().format("%Y-%m-%d %H:%M:%S");
        let level = record.level();
        let target = record.target();
        let file = record.file();
        let line = record.line();
        let msg = error_trace_message(record);
        // Get crate name from env.
        let name = env::var("SERVICE_NAME")
            .or_else(|_| env::var("CARGO_PKG_NAME"))
            .unwrap_or_else(|_| String::new());
        // Get crate version from env.
        let version = env::var("SERVICE_VERSION")
            .or_else(|_| env::var("CARGO_PKG_VERSION"))
            .unwrap_or_else(|_| String::new());

        let json_log = object! {
            "time" => time.to_string(),
            "level" => level.to_string(),
            "message" => msg,
            "service" => object! {
                "name" => name,
                "version" => version,
            },
            "location" => object! {
                "file" => file,
                "line"=> line,
                "target"=> target,
            },
        };

        writeln!(formatter, "{}", json_log)
    });

    builder
}
