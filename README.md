# npm_rs

A library to run npm commands from your Rust build script.

[Documentation](https://docs.rs/npm_rs)

This library will aid you in executing **npm** commands when building your crate/bin,
removing the burden on having to manually do so or by using a tool other than **Cargo**.

## Using npm_rs
```rust
// build.rs

fn main() -> Result<(), Box<dyn std::error:Error>>{
    npm_rs::NpmEnv::default()
                   .with_env("NODE_ENV", "production")
                   .init()
                   .install(None)
                   .run("build")
                   .exec()?;

    Ok(())
}
```

## Features
`NpmEnv` can be `Clone` when the feature `nightly` is enabled. This only works under a nightly toolchain.
```toml
# Cargo.toml

[build.dependencies]
npm_rs = { version = "*", features = ["nightly"] }
```

## Stability
Since this is a small library, I would like it to have all the needed features and to be usable before commiting to a **v1.0.0**.

Contributions are welcome!

## License
`npm_rs` is either distributed under **MIT** or **Apache-2.0** license. Choose as you please.