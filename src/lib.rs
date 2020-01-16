extern crate chrono;
pub extern crate env_logger;
extern crate log;

use chrono::Local;
use env_logger::Builder;

#[inline]
pub fn init() {
    try_init().unwrap();
}

pub fn try_init() -> Result<(), log::SetLoggerError> {
    try_init_custom_env("RUST_LOG")
}

pub fn init_custom_env(environment_variable_name: &str) {
    try_init_custom_env(environment_variable_name).unwrap();
}

pub fn try_init_custom_env(environment_variable_name: &str) -> Result<(), log::SetLoggerError> {
    let mut builder = formatted_builder();

    if let Ok(s) = ::std::env::var(environment_variable_name) {
        builder.parse_filters(&s);
    }

    builder.try_init()
}

pub fn formatted_builder() -> Builder {
    use std::io::Write;

    let mut builder = Builder::new();

    builder.format(|formatter, record| {
        writeln!(
            formatter,
            "{} [{:<5}] ({}:{}): {}",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            formatter.default_styled_level(record.level()),
            record.target(),
            record.line().unwrap_or(0),
            record.args()
        )
    });

    builder
}

#[cfg(test)]
mod tests {
    use log::{debug, error, info, trace, warn};
    #[test]
    fn test_loge() {
        use crate::init;
        use std::env;

        env::set_var("RUST_LOG", "trace");
        init();

        self::deep();
        debug!("deboogging");
        info!("such information");
        warn!("o_O");
        error!("boom");
    }

    pub fn deep() {
        trace!("one level deep!");
    }
}
