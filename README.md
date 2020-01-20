# LOGE

[![Crates.io](https://img.shields.io/crates/v/loge.svg)](https://crates.io/crates/loge)
[![Docs](https://docs.rs/loge/badge.svg)](https://docs.rs/loge)
[![MIT/APACHE-2.0](https://img.shields.io/crates/l/loge.svg)](https://crates.io/crates/loge)
[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/PsiACE/loge/Check%20Code?label=workflow)](https://github.com/PsiACE/loge/actions)

> It helps to be diligent in recording and willing to review.

A fork of [pretty-env-logger](https://github.com/seanmonstar/pretty-env-logger) with formatted output for easy analysis.

_**Note**_: Use improved log format from `0.2.0`.

## TODO

- [ ] Format
  - [ ] Simple JSON Logger.
  - [ ] Colorful, Intuitive.
- [ ] Analysis
  - [ ] Basic Parser.
  - [ ] Coarse-grained Chart.

## Usage

At first, you should add it to your `Cargo.toml` file.

```toml
[dependencies]
log = "0.4.8"
loge = "0.2.0"
```

After that, set the `RUST_LOG` variable in your code and initialize the logger.

```rust
env::set_var("RUST_LOG", "trace");
loge::init();
```

Just run your project, the log info will output as `date time [level] target - (line) ... message`:

```log
2020-01-19 22:29:06 [TRACE] simple_log::tests - (line 21) ... one level deep!
2020-01-19 22:29:06 [DEBUG] simple_log::tests - (line 14) ... deboogging
2020-01-19 22:29:06 [INFO ] simple_log::tests - (line 15) ... such information
2020-01-19 22:29:06 [WARN ] simple_log::tests - (line 16) ... o_O
2020-01-19 22:29:06 [ERROR] simple_log::tests - (line 17) ... boom
```

## Contact

Chojan Shang - [@PsiACE](https://github.com/psiace) - <psiace@outlook.com>

Project Link: [https://github.com/psiace/loge](https://github.com/psiace/loge)

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](./LICENSE-APACHE) or [http://apache.org/licenses/LICENSE-2.0](http://apache.org/licenses/LICENSE-2.0))
- MIT license ([LICENSE-MIT](./LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
