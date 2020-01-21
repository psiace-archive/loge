# loge

[![Crates.io](https://img.shields.io/crates/v/loge.svg)](https://crates.io/crates/loge)
[![Docs](https://docs.rs/loge/badge.svg)](https://docs.rs/loge)
[![MIT/APACHE-2.0](https://img.shields.io/crates/l/loge.svg)](https://crates.io/crates/loge)
[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/PsiACE/loge/Check%20Code?label=workflow)](https://github.com/PsiACE/loge/actions)

> It helps to be diligent in recording and willing to review.

A simple logger with formatted output for easy analysis. Free your productivity start with best practices logs.

_**Note**_: Use improved log format from `0.2.0`, the log info will output as `date time [level] target - (line) ... message`:

![loge output](./loge-output.png)

## TODO

- [ ] Format
  - [ ] Simple JSON Logger.
  - [x] Colorful, Intuitive.
- [ ] Analysis
  - [ ] Basic Parser.
  - [ ] Coarse-grained Chart.

## Usage

At first, you should add it to your `Cargo.toml` file.

```toml
[dependencies]
log = "0.4.8"
loge = "0.2.1"
```

After that, set the `RUST_LOG` variable in your code and initialize the logger.

```rust
env::set_var("RUST_LOG", "trace");
loge::init();
```

Just run your project, you will get logs in the terminal.

## Contact

Chojan Shang - [@PsiACE](https://github.com/psiace) - <psiace@outlook.com>

Project Link: [https://github.com/psiace/loge](https://github.com/psiace/loge)

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](./LICENSE-APACHE) or [http://apache.org/licenses/LICENSE-2.0](http://apache.org/licenses/LICENSE-2.0))
- MIT license ([LICENSE-MIT](./LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))

## Acknowledge

- Thank you [Sean McArthur](https://seanmonstar.com) for [`pretty_env_log`](https://github.com/seanmonstar/pretty-env-logger), which this is based on.
