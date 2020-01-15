# LOGE

> Imitation is the fastest way to learn.

Just a fork of [pretty-env-logger](https://github.com/seanmonstar/pretty-env-logger). Formatted output, analysis friendly.

Almost a copy, including code.

## Usage

At first, you should add it to your `Cargo.toml` file.

```toml
[dependencies]
log = "0.4.8"
loge = "0.1.1"
```

After that, set the `RUST_LOG` variable in your code and initialize the logger.

```rust
env::set_var("RUST_LOG", "info");
loge::init();
```

Just run your project, the log info will output as `date time [level] (target): infomation`:

```log
2020-01-15 14:03:51 [INFO] (actix_server::builder:263): Starting 8 workers
2020-01-15 14:03:51 [INFO] (actix_server::builder:275): Starting "actix-web-service-127.0.0.1:8087" service on 127.0.0.1:8087
```

## Contact

Chojan Shang - [@PsiACE](https://github.com/psiace) - <psiace@outlook.com>

Project Link: [https://github.com/psiace/loge](https://github.com/psiace/loge)

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](./LICENSE-APACHE) or [http://apache.org/licenses/LICENSE-2.0](http://apache.org/licenses/LICENSE-2.0))
- MIT license ([LICENSE-MIT](./LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
