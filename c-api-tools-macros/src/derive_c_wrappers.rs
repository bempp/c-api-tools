use darling::Error;
use darling::{ast::NestedMeta, FromDeriveInput, FromMeta};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[derive(Default, FromMeta)]
#[darling(default)]
struct CWrapperArgs {
    name: String,
    create: bool,
    free: bool,
    unpack: bool,
}

pub(crate) fn c_wrapper_impl(args: TokenStream, item: TokenStream) -> TokenStream {
    let syn::ItemStruct {
        attrs,
        vis,
        ident,
        generics,
        fields,
        ..
    } = parse_macro_input!(item as syn::ItemStruct);

    if !matches!(vis, syn::Visibility::Public(_)) {
        return syn::Error::new(
            ident.span(),
            "Only public structs can be wrapped with `#[cwrapper]`.",
        )
        .to_compile_error()
        .into();
    }

    if !generics.lt_token.is_none() {
        return syn::Error::new(ident.span(), "Generics are not supported in `#[cwrapper]`.")
            .to_compile_error()
            .into();
    }

    if !matches!(fields, syn::Fields::Unit) {
        return syn::Error::new(
            ident.span(),
            "Only unit structs are supported in `#[cwrapper]`.",
        )
        .to_compile_error()
        .into();
    }

    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(Error::from(e).write_errors());
        }
    };

    let args = match CWrapperArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let CWrapperArgs {
        name,
        create,
        free,
        unpack,
    } = args;

    if name.is_empty() {
        return syn::Error::new(
            ident.span(),
            "`name` must be a non-empty `cfunc` attribute.",
        )
        .to_compile_error()
        .into();
    }

    quote! {
        #[no_mangle]
        pub extern "C" fn hello_world() {
            println!("Object: {}", stringify!(#create));
        }

    }
    .into()
}
