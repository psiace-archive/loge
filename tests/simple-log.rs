#[cfg(test)]
mod tests {
    extern crate loge;
    use log::{debug, error, info, trace, warn};
    #[test]
    fn test_loge() {
        use self::loge;
        use std::env;

        env::set_var("RUST_LOG", "trace");
        loge::init();

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
