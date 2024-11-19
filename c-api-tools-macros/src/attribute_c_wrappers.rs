use darling::Error;
use darling::{ast::NestedMeta, FromMeta};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::parse_macro_input;

#[derive(Default, FromMeta)]
#[darling(default)]
struct CWrapperArgs {
    name: String,
    create: bool,
    free: bool,
    unwrap: bool,
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

    if generics.lt_token.is_some() {
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
        unwrap,
    } = args;

    if name.is_empty() {
        return syn::Error::new(
            ident.span(),
            "`name` must be a non-empty `cfunc` attribute.",
        )
        .to_compile_error()
        .into();
    }

    let mut output = quote! {
        #(#attrs)*
        #vis struct #ident {
            _ptr: Box<dyn std::any::Any>,
        }
    };

    if create {
        let name = syn::Ident::new((name.clone() + "_create").as_str(), Span::call_site());

        output.extend(quote! {
            #[no_mangle]
            pub extern "C" fn #name() -> *mut #ident {
                let obj = #ident { _ptr: Box::new(()) };
                let ptr = Box::into_raw(Box::new(obj));
                ptr
            }
        });
    }

    if free {
        let name = syn::Ident::new((name.clone() + "_free").as_str(), Span::call_site());

        output.extend(quote! {
            #[no_mangle]
            pub unsafe extern "C" fn #name(ptr: *mut #ident) {
                if ptr.is_null() {
                    return;
                }
                unsafe {
                    drop(Box::from_raw(ptr));
                }
            }
        });
    }

    if unwrap {
        let name = syn::Ident::new((name.clone() + "_unwrap").as_str(), Span::call_site());

        output.extend(quote! {
            unsafe fn #name(ptr: *mut #ident) -> Option<&'static mut Box<dyn std::any::Any>> {
                if ptr.is_null() {
                    return None;
                }
                Some(&mut (*ptr)._ptr)
            }
        });
    }

    output.into()
}
