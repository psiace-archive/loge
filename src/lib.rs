#![cfg_attr(test, deny(warnings))]
#![deny(missing_docs)]

//! A logger configured via an environment variable which writes cancer to
//! standard error with colored output for log levels.
//!
//! ## Example
//!
//! ```
//! extern crate loge;
//! #[macro_use] extern crate log;
//!
//! use std::env;
//!
//! fn main() {
//!     env::set_var("RUST_LOG", "trace");
//!     loge::init();
//!
//!     trace!("this is trace level");
//!     debug!("meet a note");
//!     info!("everything is normal");
//!     warn!("be careful");
//!     error!("something error");
//! }
//! ```

extern crate chrono;
pub extern crate env_logger;
extern crate log;

use chrono::Local;
use env_logger::{
    fmt::{Color, Style, StyledValue},
    Builder,
};
use log::Level;

fn colored_level<'a>(style: &'a mut Style, level: Level) -> StyledValue<'a, &'static str> {
    match level {
        Level::Trace => style.set_color(Color::Cyan).value("TRACE"),
        Level::Debug => style.set_color(Color::Blue).value("DEBUG"),
        Level::Info => style.set_color(Color::Green).value("INFO "),
        Level::Warn => style.set_color(Color::Yellow).value("WARN "),
        Level::Error => style.set_color(Color::Red).value("ERROR"),
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

/// Returns a `env_logger::Builder` for further customization.
///
/// This method will return a colored and formatted) `env_logger::Builder`
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
        let mut line_style = formatter.style();

        time_style.set_color(Color::Ansi256(59));
        line_style.set_color(Color::Magenta);

        writeln!(
            formatter,
            "{} [{}] {} - (line {}) ... {}",
            time_style.value(Local::now().format("%Y-%m-%d %H:%M:%S")),
            colored_level(&mut level_style, record.level()),
            record.target(),
            line_style.value(record.line().unwrap_or(0)),
            record.args()
        )
    });

    builder
}
