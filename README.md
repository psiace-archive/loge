# loge

[![Crates.io](https://img.shields.io/crates/v/loge.svg)](https://crates.io/crates/loge)
[![Docs](https://docs.rs/loge/badge.svg)](https://docs.rs/loge)
[![MIT/APACHE-2.0](https://img.shields.io/crates/l/loge.svg)](https://crates.io/crates/loge)
[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/PsiACE/loge/Check%20Code?label=workflow)](https://github.com/PsiACE/loge/actions)

> It helps to be diligent in recording and willing to review.

A simple logger with formatted output for easy analysis. Free your productivity start with best practices logs.

`loge` is currently controlled by features and environment variables:

- If you need to record time, enable `chrono`;
- When you enable `json`, you also need to enable `chrono`;
- For pretty output, enable `colored`.
- By default, we turn on all three features.
- You can choose one of three formats: `target`, `fileline` or `json`. (Set environment variable `LOGE_FORMAT`.)

_**Examples**_:

- *target*: `date time [level] target ... message`(enable `chrono`) or `level target ... message`:

    ![loge output](./screenshot/loge-output.png)

- *fileline*: `date time [level] file - target (line) ... message`(enable `chrono`) or `level file - target (line) ... message`:

    ![loge fileline output](./screenshot/loge-fileline-output.png)

- *json*: You should enable `json` and `chrono`; for pretty output, enable `colored`.

    ```json
    {
      "time": "2020-02-02 18:33:33.645",
      "level": "ERROR",
      "message": "boom -> tests/simple-jsonified-log.rs:17",
      "service": {
        "name": "loge",
        "version": "0.4.1"
      },
      "location": {
        "file": "tests/simple-jsonified-log.rs",
        "line": 17,
        "target": "simple_jsonified_log::tests"
      }
    }
    ```

- *file*: If you want to print log to a file, enable it, and you will see output both in terminal and file.

## TODO

- [ ] Format
  - [x] Simple JSON Logger. // Unoptimized.
  - [x] Colorful, Intuitive.
- [ ] Analysis
  - [ ] Basic Parser.
  - [ ] Coarse-grained Chart.
- [ ] Others
  - [x] Lightweight - As few dependencies as possible.
  - [ ] Clear architecture and flexible configuration.
  - [ ] Asynchronous.
  - [x] Output to file.

## Usage

At first, you should add it to your `Cargo.toml` file.

```toml
[dependencies]
log = "0.4.8"
loge = "0.4.2"
```

After that, set the `RUST_LOG` & `LOGE_FORMAT` variable in your code and initialize the logger.

```rust
std::env::set_var("RUST_LOG", "trace");
std::env::set_var("LOGE_FORMAT", "target"); // `fileline` or `json`(need enable `json` and `chrono`)
loge::init(); // If you enable `file`, you can't use this func.
// Or `loge::init_with_file("your-app.log");`, but need enable `file`, and will see output both in terminal and file. 
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

- Thank you [Sean McArthur](https://seanmonstar.com) for [`pretty_env_log`](https://github.com/seanmonstar/pretty-env-logger).
- Thank you [Sam Clements](https://mastodon.social/@borntyping) for [`rust-simple_logger`](https://github.com/borntyping/rust-simple_logger).
- Thank you [Christopher Blanchard](https://github.com/lambdaxymox) for [`file-logger`](https://github.com/lambdaxymox/file-logger).
