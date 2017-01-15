# Derive Rust Errors

[![Build Status](https://travis-ci.org/rushmorem/derive-error.svg?branch=master)](https://travis-ci.org/rushmorem/derive-error) [![Latest Version](https://img.shields.io/crates/v/derive-error.svg)](https://crates.io/crates/derive-error) [![Docs](https://docs.rs/derive-error/badge.svg)](https://docs.rs/derive-error)

This crate uses macros 1.1 to derive custom errors.

## Getting Started

Add this crate to your dependencies section:-

```text
[dependencies]
derive-error = "0.0.0"
```

Import it in your `main.rs` or `lib.rs`:-

```rust,ignore
#[macro_use]
extern crate derive-error;
```

Deriving errors is simple. Here is an example:

```rust,norun
#[derive(Error)]
pub enum Error {
  Io(::std::io::Error),
}
```

That's it!
