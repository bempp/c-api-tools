mod attribute_c_wrappers;
mod eval_concrete_type;

use attribute_c_wrappers::c_wrapper_impl;
use eval_concrete_type::eval_with_concrete_type_impl;
use proc_macro::TokenStream;

#[proc_macro]
pub fn eval_with_concrete_type(item: TokenStream) -> TokenStream {
    eval_with_concrete_type_impl(item)
}

#[proc_macro_attribute]
pub fn cfuncs(args: TokenStream, item: TokenStream) -> TokenStream {
    c_wrapper_impl(args, item)
}
