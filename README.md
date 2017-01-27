# Derive Rust Errors

[![Build Status](https://travis-ci.org/rushmorem/derive-error.svg?branch=master)](https://travis-ci.org/rushmorem/derive-error) [![Latest Version](https://img.shields.io/crates/v/derive-error.svg)](https://crates.io/crates/derive-error) [![Docs](https://docs.rs/derive-error/badge.svg)](https://docs.rs/derive-error)

This crate uses macros 1.1 to derive custom errors.

## Getting Started

Add this crate to your dependencies section:-

```toml
[dependencies]
derive-error = "0.0.2"
```

Import it in your `main.rs` or `lib.rs`:-

```rust,ignore
#[macro_use]
extern crate derive-error;
```

Deriving errors is simple. Simply create an enum for your errors as suggested [in the Rust book](https://doc.rust-lang.org/book/error-handling.html#error-handling-with-a-custom-type), add short descriptions for the enum variants using doc comments, throw in a `#[derive(Debug, Error)]` and you are done. Here is the example in the book implemented using this library:-

```rust,norun
#[derive(Debug, Error)]
enum CliError {
  /// IO Error
  Io(io::Error),
  /// Failed to parse the CSV file
  Csv(csv::Error),
  /// No matching cities with a population were found
  NotFound,
}
```

This will derive implementations for `Display`, `Error` and `From`. See [the reql crate](https://github.com/rust-rethinkdb/reql/blob/master/src/errors.rs) for a real world example of how to use this crate.
