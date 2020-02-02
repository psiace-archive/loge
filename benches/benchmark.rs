#![feature(test)]

extern crate loge;
extern crate test;
#[macro_use]
extern crate log;

use test::Bencher;

#[bench]
fn b10_no_logger_active(b: &mut Bencher) {
    b.iter(use_error);
}

#[bench]
fn b20_initialize_logger(_: &mut Bencher) {
    ::std::env::set_var("RUST_LOG", "trace");
    ::std::env::set_var("LOGE_FORMAT", "target");
    loge::init();
}

#[bench]
fn b30_relevant_logs(b: &mut Bencher) {
    b.iter(use_error);
}

#[bench]
fn b40_suppressed_logs(b: &mut Bencher) {
    b.iter(use_trace);
}

fn use_error() {
    for _ in 1..100 {
        error!("This is an error message");
    }
}
fn use_trace() {
    for _ in 1..100 {
        trace!("This is a trace message");
    }
}
