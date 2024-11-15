use darling::Error;
use darling::{ast::NestedMeta, FromMeta};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Mut;
use syn::{parse_macro_input, FnArg, Pat, PatIdent, Path, Token, Type, TypePath, TypePtr};
use syn::{Ident, PatType};

#[derive(Default, FromMeta)]
#[darling(default)]
struct GenType {
    name: String,
    replace_with: Vec<syn::LitStr>,
}

#[derive(Default, FromMeta)]
#[darling(default)]
struct Field {
    arg: usize,
    wrapper: String,
    is_mut: bool,
    replace_with: Vec<syn::LitStr>,
}

#[derive(Default, FromMeta)]
#[darling(default)]
struct ConcretiseTypeArgs {
    #[darling(multiple)]
    gen_type: Vec<GenType>,
    #[darling(multiple)]
    field: Vec<Field>,
}

pub(crate) fn concretise_type_impl(args: TokenStream, item: TokenStream) -> TokenStream {
    let syn::ItemFn {
        attrs,
        vis,
        mut sig,
        block,
        ..
    } = parse_macro_input!(item as syn::ItemFn);

    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(Error::from(e).write_errors());
        }
    };

    let args = match ConcretiseTypeArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let inputs = &mut sig.inputs;

    let input = inputs.get_mut(0).unwrap();

    let pat_ident = if let FnArg::Typed(pattern) = input {
        if let Pat::Ident(pat) = pattern.pat.as_mut() {
            pat
        } else {
            panic!();
        }
    } else {
        panic!();
    };

    pat_ident.ident = Ident::new("bar", Span::call_site());

    let pat: Box<Pat> = Box::new(Pat::Ident(PatIdent {
        attrs: Vec::new(),
        by_ref: None,
        mutability: None,
        ident: Ident::new("ptr", Span::call_site()),
        subpat: None,
    }));

    let pat_type = PatType {
        attrs: Default::default(),
        pat: Box::new(Pat::Ident(PatIdent {
            attrs: Vec::new(),
            by_ref: None,
            mutability: None,
            ident: Ident::new("ptr", Span::call_site()),
            subpat: None,
        })),
        colon_token: <Token![:]>::default(),
        ty: Box::new(Type::Ptr(TypePtr {
            star_token: Default::default(),
            const_token: None,
            mutability: Some(Default::default()),
            elem: Box::<Type>::new(Type::Path(TypePath {
                qself: None,
                path: Path {
                    leading_colon: None,
                    segments: {
                        let mut punctuated = Punctuated::new();
                        punctuated.push(syn::PathSegment {
                            ident: Ident::new("f64", Span::call_site()),
                            arguments: syn::PathArguments::None,
                        });
                        punctuated
                    },
                },
            })),
        })),
    };

    quote! {

        pub fn test_macro() {
            println!("Pattern {} ", stringify!(#pat_type));
        }

        # vis #sig # block
    }
    .into()
}
