//! C API tools macros
#![cfg_attr(feature = "strict", deny(warnings), deny(unused_crate_dependencies))]
#![warn(missing_docs)]

mod attribute_c_wrappers;
mod attribute_concretise_types;

use attribute_c_wrappers::c_wrapper_impl;
use attribute_concretise_types::concretise_type_impl;
use proc_macro::TokenStream;

/// C functions
#[proc_macro_attribute]
pub fn cfuncs(args: TokenStream, item: TokenStream) -> TokenStream {
    c_wrapper_impl(args, item)
}

/// Concretise types
#[proc_macro_attribute]
pub fn concretise_types(args: TokenStream, item: TokenStream) -> TokenStream {
    concretise_type_impl(args, item)
}
