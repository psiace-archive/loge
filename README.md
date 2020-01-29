# loge

[![Crates.io](https://img.shields.io/crates/v/loge.svg)](https://crates.io/crates/loge)
[![Docs](https://docs.rs/loge/badge.svg)](https://docs.rs/loge)
[![MIT/APACHE-2.0](https://img.shields.io/crates/l/loge.svg)](https://crates.io/crates/loge)
[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/PsiACE/loge/Check%20Code?label=workflow)](https://github.com/PsiACE/loge/actions)

> It helps to be diligent in recording and willing to review.

A simple logger with formatted output for easy analysis. Free your productivity start with best practices logs.

_**Note**_: Split for clearer logs from `0.2.3`, the log info will output as `date time [level] target ... message`:

![loge output](./loge-output.png)

 or `date time [level] file -  (line) ... message`:

![loge fileline output](./loge-fileline-output.png)

## TODO

- [ ] Format
  - [x] Simple JSON Logger. // Temporary, unoptimized.
  - [x] Colorful, Intuitive.
- [ ] Analysis
  - [ ] Basic Parser.
  - [ ] Coarse-grained Chart.

## Usage

At first, you should add it to your `Cargo.toml` file.

```toml
[dependencies]
log = "0.4.8"
loge = "0.3.1"
```

After that, set the `RUST_LOG` variable in your code and initialize the logger.

```rust
env::set_var("RUST_LOG", "trace");
loge::init();
// Or `loge::init_fileline();` for file line logger.
// Or `loge::init_jsonified();` for jsonified logger without any color.
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
