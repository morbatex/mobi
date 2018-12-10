# WIP Libmobi Rust Wrapper

This crate is a WIP safe wrapper around the `libmobi` library. It applies the RAII pattern
and Rust lifetimes to ensure safe usage of the `libudev` functionality. The RAII pattern ensures
that all acquired resources are released when they're no longer needed, and Rust lifetimes ensure
that resources are released in a proper order.

## Dependencies
In order to use the `mobi` crate, you must have [`libmobi library`](https://www.fabiszewski.net/libmobi/index.html) or a compatible alternative installed. Visit the [libmobi github page](https://github.com/bfabiszewski/libmobi) for information on the installation process.

## Usage
Add `mobi` as a dependency in `Cargo.toml`:

```toml
[dependencies]
mobi = { git = "https://github.com/morbatex/mobi/" }
```

Import the `mobi` crate and use it's function.

```rust
extern crate mobi;
```
## License
Copyright © 2018 Max Böcker

Distributed under the [MIT License](LICENSE-MIT)/[Apache-v2](LICENSE-APACHE).
