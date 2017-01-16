#[macro_use] extern crate derive_error;

#[derive(Debug, Error)]
pub enum _Error1 { }

#[derive(Debug, Error)]
pub enum _Error2 {
    ErrorKind1,
    ErrorKind2(::std::io::Error),
}

/// Custom error
#[derive(Debug, Error)]
pub enum _Error3 {
    ErrorKind1,
    ErrorKind2(::std::io::Error),
}

/// Custom error
#[derive(Debug, Error)]
pub enum _Error4 {
    /// First error kind
    ErrorKind1,

    /// IO error
    ErrorKind2(::std::io::Error),

    /// Format error
    ErrorKind3{ error: ::std::fmt::Error },
}
