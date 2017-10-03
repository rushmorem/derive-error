# Derive Rust Errors

[![Build Status](https://travis-ci.org/rushmorem/derive-error.svg?branch=master)](https://travis-ci.org/rushmorem/derive-error) [![Latest Version](https://img.shields.io/crates/v/derive-error.svg)](https://crates.io/crates/derive-error) [![Docs](https://docs.rs/derive-error/badge.svg)](https://docs.rs/derive-error)

This crate uses macros 1.1 to derive custom errors.

## Getting Started

Add this crate to your dependencies section:-

```toml
[dependencies]
derive-error = "0.0.3"
```

Import it in your `main.rs` or `lib.rs`:-

```rust,ignore
#[macro_use]
extern crate derive_error;
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

## Configuration

Not all errors are exactly the same and need, for example, `From` implementations. derive-error supports attributes on Enum variants that allow you to enable/disable certain aspects of how Error deriving functions. There are 3 attributes that can be attached to variants of enums deriving `Error`: `non_std`, `msg_embedded`, `no_from`.

`msg_embedded` displays the error through an item in the variant, generally a `String`.

`no_from` skips the `From` impl for the variant of the enum, you want this if the error isn't coming from somewhere else, and your code is essentially the source of the error.

`non_std` displays the error from the variant via it's doc comment `///` or it's embedded message (if the variant has that attribute), otherwise it will try and use the cause of the underlying error inside the variant. This is usually combined with `no_from`.

```rust
#[derive(Debug, Error)]
enum Error {
	#[error(msg_embedded, no_from, non_std)]
	RuntimeError(String),

	Io(Io::Error),

	#[error(non_std, no_from)]
	Json(serde_json::Value)
}
```

This example showcases how to attach the attributes. `RuntimeError` has a `String` internally which in the case of an error will be used for displaying. `Io` is getting a `From` implementation for `Io::Error` and this is generally the most common usecase. It is also possible to embed values directly in errors, in the `Json` variant it's embedding a `Value` inside, to be used later on for other information.

These were adapted from the [the reql crate's](https://github.com/rust-rethinkdb/reql/blob/master/src/errors.rs) usage of derive-error
