use darling::Error;
use darling::{ast::NestedMeta, FromMeta};
use itertools::{izip, Itertools};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{
    parse_macro_input, Expr, ExprCall, FnArg, LitStr, Pat, PatIdent, Path, Signature, Token, Type,
    TypePath, TypePtr,
};
use syn::{Ident, PatType};

fn replace_templates_with_types(names: &[String], templates: &[String]) -> Vec<String> {
    let ntypes = templates.len();

    let mut complete_types = templates.to_vec();

    for index in 0..ntypes - 1 {
        let replace_type = complete_types[index].clone();
        for ty in complete_types[index + 1..ntypes].iter_mut() {
            *ty = ty.replace(&("{{".to_owned() + &names[index] + "}}"), &replace_type);
        }
    }
    complete_types
}

fn create_if_let_condition(
    args: &ConcretiseTypeArgs,
    concrete_field_types: &[String],
    sig: &Signature,
    expr_call: &ExprCall,
) -> proc_macro2::TokenStream {
    let mut left_condition: String = if args.field.len() > 1 {
        "if let (".to_string()
    } else {
        "if let ".to_string()
    };

    let mut right_condition: String = if args.field.len() > 1 {
        "(".to_string()
    } else {
        "".to_string()
    };

    let mut is_first = true;
    for (field, concrete_field_type) in izip!(args.field.iter(), concrete_field_types.iter()) {
        if !is_first {
            left_condition += ", ";
            right_condition += ", ";
        } else {
            is_first = false;
        }

        let arg = sig
            .inputs
            .get(field.arg)
            .unwrap_or_else(|| panic!("Argument {} does not exist.", field.arg));

        let ident = get_function_arg_ident(arg);
        let mutability = function_arg_is_mutable(arg);

        left_condition += &("Some(".to_owned() + &ident.to_string() + ")");
        if mutability {
            right_condition +=
                &(ident.to_string() + ".downcast_mut::<" + concrete_field_type + ">()");
        } else {
            right_condition +=
                &(ident.to_string() + ".downcast_ref::<" + concrete_field_type + ">()");
        }
    }

    if args.field.len() > 1 {
        left_condition += ")";
        right_condition += ")";
    };

    let condition = left_condition + " = " + &right_condition;
    let condition = condition.parse::<proc_macro2::TokenStream>().unwrap();

    quote! {
      #condition {
          #expr_call
      } else
    }
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

fn function_arg_is_mutable(arg: &FnArg) -> bool {
    if let FnArg::Typed(arg) = arg {
        if let Type::Reference(ty) = arg.ty.as_ref() {
            ty.mutability.is_some()
        } else {
            panic!("Type to replace must be a reference type.")
        }
    } else {
        panic!("Argument must be typed.");
    }
}

fn get_function_arg_ident(arg: &FnArg) -> &Ident {
    if let FnArg::Typed(arg) = arg {
        if let Pat::Ident(pat_ident) = arg.pat.as_ref() {
            &pat_ident.ident
        } else {
            panic!("Pattern must describe an identifier.");
        }
    } else {
        panic!("Argument must be typed.");
    }
}

fn create_signature(args: &ConcretiseTypeArgs, old_signature: &Signature) -> Signature {
    let Signature {
        ident,
        mut inputs,
        output,
        ..
    } = old_signature.clone();

    for field in args.field.iter() {
        let arg = inputs
            .get_mut(field.arg)
            .unwrap_or_else(|| panic!("Argument {} does not exist.", field.arg));

        let ident = get_function_arg_ident(arg);

        *arg = FnArg::Typed(create_ptr_argument(&ident.to_string(), &field.wrapper));
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
        ident,
        // ident: Ident::new(&("c_".to_owned() + &ident.to_string()), Span::call_site()),
        generics: Default::default(),
        paren_token: Default::default(),
        inputs,
        variadic: None,
        output,
    }
}

fn create_function_call(sig: &Signature) -> ExprCall {
    // We go through the signature and build from it a function call sequence.

    let mut punctuated = Punctuated::<Expr, Token![,]>::new();
    let paren = sig.paren_token;
    // The following creates the function name
    let func = Box::new(Expr::Path(syn::ExprPath {
        path: Path {
            leading_colon: None,
            segments: {
                let mut punctuated = Punctuated::new();
                punctuated.push(syn::PathSegment {
                    ident: sig.ident.clone(),
                    arguments: syn::PathArguments::None,
                });
                punctuated
            },
        },
        attrs: Default::default(),
        qself: None,
    }));

    // This creates the arguments
    for arg in sig.inputs.iter() {
        punctuated.push(Expr::Path(syn::ExprPath {
            path: Path {
                leading_colon: None,
                segments: {
                    let mut punctuated = Punctuated::new();
                    punctuated.push(syn::PathSegment {
                        ident: get_function_arg_ident(arg).clone(),
                        arguments: syn::PathArguments::None,
                    });
                    punctuated
                },
            },
            attrs: Default::default(),
            qself: None,
        }));
    }

    ExprCall {
        attrs: Default::default(),
        func,
        paren_token: paren,
        args: punctuated,
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
        vis, sig, block, ..
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

    let ConcretiseTypeArgs { gen_type, field } = &args;

    let gen_keys = gen_type.iter().map(|x| x.name.clone()).collect_vec();
    let field_keys = field.iter().map(|x| x.name.clone()).collect_vec();

    // We are first preparing the new signature.
    // The new signature replaces template types with the wrapper pointer types.
    let new_signature = create_signature(&args, &sig);

    let call_expr = create_function_call(&new_signature);

    // We start preparing the output quote. This will contain the new signature

    // We are now doing a cartesian iterator over the gen types and within this a cartesion
    // iterator over the field types.

    let mut if_let_stream = quote! {};

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

            // We now have the complete field types. Let us build the corresponding if let statement.
            //

            let if_let = create_if_let_condition(&args, &complete_field_types, &sig, &call_expr);

            if_let_stream = quote! {
                #if_let_stream
                #if_let

            }

            //
        }
    }

    // Before we can finish we need to unwrap the input pointers into their corresponding inner types.

    let idents = sig
        .inputs
        .iter()
        .map(|x| get_function_arg_ident(x).clone())
        .collect_vec();

    let output = quote! {
       #vis #new_signature {
           #vis #sig
           #block

           #(
               let #idents = &(*#idents)._ptr;
           )*

           #if_let_stream
           {
               panic!("Unknown type.");
           }

       }

    };

    output.into()
    // replace_with.insert(0, create_ptr_argument("ty", "MyWrapper"));
    //
    // let sig = create_signature(&replace_with, &sig);

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
}
