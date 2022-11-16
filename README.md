# npm_rs

![License (MIT OR APACHE)](https://img.shields.io/crates/l/npm_rs?style=flat-square)
[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/upsettingboy/npm_rs/Rust?style=flat-square&logo=github&label=CI)](https://github.com/upsettingboy/npm_rs/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/npm_rs?style=flat-square)](https://crates.io/crates/npm_rs)
[![docs.rs](https://img.shields.io/static/v1?label=docs.rs&message=webpage&color=brightgreen&style=flat-square)](https://docs.rs/npm_rs)

A library to run `npm` commands from your Rust build script.

[Documentation](https://docs.rs/npm_rs)

This library will aid you in executing `npm` commands when building your crate/bin,
removing the burden of having to manually do so or by using a tool other than **Cargo**.

<!-- cargo-sync-readme start -->

This crate provides an abstraction over `Command` to use `npm`
in a simple and easy package with fluent API.

`npm_rs` exposes `NpmEnv` to configure the `npm` execution enviroment and `Npm` to use said enviroment to execute `npm` commands.

# Examples
## Manual `NODE_ENV` setup
```rust
// build.rs

use npm_rs::*;

let exit_status = NpmEnv::default()
       .with_node_env(&NodeEnv::Production)
       .with_env("FOO", "bar")
       .init_env()
       .install(None)
       .run("build")
       .exec()?;
```

## Automatic `NODE_ENV` setup
```rust
// build.rs

use npm_rs::*;

let exit_status = NpmEnv::default()
       .with_node_env(&NodeEnv::from_cargo_profile().unwrap_or_default())
       .with_env("FOO", "bar")
       .init_env()
       .install(None)
       .run("build")
       .exec()?;
```
<!-- cargo-sync-readme end -->

# License
`npm_rs` is either distributed under **MIT** or **Apache-2.0** license. Choose as you please.