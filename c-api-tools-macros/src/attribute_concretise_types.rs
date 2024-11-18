use std::collections::HashMap;

use darling::Error;
use darling::{ast::NestedMeta, FromMeta};
use itertools::{izip, Itertools};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Mut;
use syn::{
    parse_macro_input, FnArg, LitStr, Pat, PatIdent, Path, Signature, Token, Type, TypePath,
    TypePtr,
};
use syn::{Ident, PatType};

fn replace_templates_with_types(names: &Vec<String>, templates: &Vec<String>) -> Vec<String> {
    let ntypes = templates.len();

    let mut complete_types = templates.clone();

    for index in 0..ntypes - 1 {
        let replace_type = complete_types[index].clone();
        for ty in complete_types[index + 1..ntypes].iter_mut() {
            *ty = ty.replace(&("{{".to_owned() + &names[index] + "}}"), &replace_type);
        }
    }
    complete_types
}

fn create_ptr_argument(var_name: &str, ptr_type: &str) -> PatType {
    PatType {
        attrs: Default::default(),
        pat: Box::new(Pat::Ident(PatIdent {
            attrs: Vec::new(),
            by_ref: None,
            mutability: None,
            ident: Ident::new(var_name, Span::call_site()),
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
                            ident: Ident::new(ptr_type, Span::call_site()),
                            arguments: syn::PathArguments::None,
                        });
                        punctuated
                    },
                },
            })),
        })),
    }
}

fn create_signature(
    replace_args: &HashMap<usize, PatType>,
    old_signature: &Signature,
) -> Signature {
    let Signature {
        ident,
        mut inputs,
        output,
        ..
    } = old_signature.clone();

    for (arg_nr, pat) in replace_args.iter() {
        let arg = inputs
            .get_mut(*arg_nr)
            .expect(&format!("Argument {} does not exist.", arg_nr));

        *arg = FnArg::Typed(pat.clone());
    }

    Signature {
        constness: None,
        asyncness: None,
        unsafety: Some(Default::default()),
        abi: Some(syn::Abi {
            extern_token: Default::default(),
            name: Some(LitStr::new("C", Span::call_site())),
        }),
        fn_token: Default::default(),
        ident: Ident::new(&("c_".to_owned() + &ident.to_string()), Span::call_site()),
        generics: Default::default(),
        paren_token: Default::default(),
        inputs,
        variadic: None,
        output,
    }
}

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
    name: String,
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

    let ConcretiseTypeArgs { gen_type, field } = args;

    let mut replace_with: HashMap<usize, PatType> = HashMap::new();

    let gen_keys = gen_type.iter().map(|x| x.name.clone()).collect_vec();
    let field_keys = field.iter().map(|x| x.name.clone()).collect_vec();

    // We are first doing a cartesian iterator over the gen types and within this a cartesion
    // iterator over the field types.

    for gen_it in gen_type
        .iter()
        .map(|x| x.replace_with.iter().map(|x| x.value().clone()))
        .multi_cartesian_product()
        .peekable()
    {
        // Replace the generic types in order. A later type can only depend on earlier types.

        let complete_gen_types = replace_templates_with_types(&gen_keys, &gen_it);

        // Now we iterate over the field types. The field types are the types that get replaced
        // in the index list with the corresponding Wrapper types.

        for field_type_it in field
            .iter()
            .map(|x| x.replace_with.iter().map(|x| x.value().clone()))
            .multi_cartesian_product()
            .peekable()
        {
            // First we replace the generic keys in the field types.

            let mut complete_field_types = field_type_it.clone();

            for (key, complete_gen_type) in izip!(gen_keys.iter(), complete_gen_types.iter()) {
                for field in complete_field_types.iter_mut() {
                    *field = field.replace(&("{{".to_owned() + key + "}}"), complete_gen_type);
                }
            }

            // We now substitute within the field types.

            let complete_field_types =
                replace_templates_with_types(&field_keys, &complete_field_types);

            // We now have the completed types. We need to build the forward signature to the inner original function
            // and the if block that controls the down-cast.
        }
    }

    replace_with.insert(0, create_ptr_argument("ty", "MyWrapper"));
    let sig = create_signature(&replace_with, &sig);

    // let inputs = &mut sig.inputs;

    // let input = inputs.get_mut(0).unwrap();

    // let pat_ident = if let FnArg::Typed(pattern) = input {
    //     if let Pat::Ident(pat) = pattern.pat.as_mut() {
    //         pat
    //     } else {
    //         panic!();
    //     }
    // } else {
    //     panic!();
    // };

    // pat_ident.ident = Ident::new("bar", Span::call_site());

    // let pat: Box<Pat> = Box::new(Pat::Ident(PatIdent {
    //     attrs: Vec::new(),
    //     by_ref: None,
    //     mutability: None,
    //     ident: Ident::new("ptr", Span::call_site()),
    //     subpat: None,
    // }));

    quote! {

        pub fn test_macro() {
            println!("Pattern {} ", stringify!(#sig));
        }

        # vis #sig # block
    }
    .into()
}
