extern crate loge;
use log::{debug, error, info, trace, warn};
use std::env;

fn main() {
    env::set_var("RUST_LOG", "trace");
    loge::init_jsonified();

    self::deep();
    debug!("deboogging");
    info!("such information");
    warn!("o_O");
    error!("boom");
}

pub fn deep() {
    trace!("one level deep!");
}
