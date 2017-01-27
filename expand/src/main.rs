#[macro_use] extern crate derive_error;

use std::error::Error;
use std::io;

#[derive(Result)]
pub struct Result<T, E>(::std::result::Result<T, E>);

#[derive(Debug, Error)]
pub struct _Error1;

#[derive(Debug, Error)]
pub enum _Error2 {
    ErrorKind1,
    ErrorKind2(io::Error),
}

/// Custom error
#[derive(Debug, Error)]
pub enum _Error3 {
    ErrorKind1,
    ErrorKind2(io::Error),
}

/// Custom error
#[derive(Debug, Error)]
pub enum _Error4 {
    /// First error kind
    ErrorKind1,

    /// IO error
    ErrorKind2(io::Error),

    /// Format error
    ErrorKind3{ error: ::std::fmt::Error },
}

fn main() {
    let err1 = _Error2::ErrorKind1;
    assert!(err1.cause().is_none());

    let error = io::Error::new(io::ErrorKind::Other, "oh no!");
    let err2 = _Error2::ErrorKind2(error);
    assert!(err2.cause().is_some());
}
