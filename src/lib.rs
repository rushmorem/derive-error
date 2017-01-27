//! Derive custom errors using macros 1.1

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
extern crate case;

mod error;

use proc_macro::TokenStream;
use error::Error;

#[proc_macro_derive(Error, attributes(error))]
pub fn derive_error(input: TokenStream) -> TokenStream {
    let source = input.to_string();
    let ast = syn::parse_macro_input(&source).expect("failed to parse the error defination");
    Error::new(ast).derive().parse().expect("failed to parse the derived output")
}
