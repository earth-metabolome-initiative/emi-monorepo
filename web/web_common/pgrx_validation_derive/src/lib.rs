#![doc = include_str!("../README.md")]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, ExprCall, ExprPath, parse_macro_input, spanned::Spanned, visit_mut::VisitMut};

struct MethodRenamer;

/// Valid prefixes for the validation functions.
const VALID_PREFIXES: [&str; 4] =
    ["must_be_", "must_not_be_", "must_contain_", "must_not_contain_"];

impl VisitMut for MethodRenamer {
    fn visit_expr_call_mut(&mut self, node: &mut ExprCall) {
        if let Expr::Path(ExprPath { path, .. }) = node.func.as_mut() {
            if let Some(ident) = path.get_ident() {
                for prefix in VALID_PREFIXES {
                    if let Some(ident) = ident.to_string().strip_prefix(prefix) {
                        path.segments[0].ident =
                            syn::Ident::new(&format!("__inner_{prefix}{ident}"), path.span());
                        break;
                    }
                }
            }
        }

        // Continue visiting other expressions within the method call.
        syn::visit_mut::visit_expr_call_mut(self, node);
    }
}

#[proc_macro_attribute]
/// Transforms the provided validation function depending on the enabled crate
/// features.
///
/// # Implementation details
///
/// The [`validation`] attrobute macro is meant to be used on functions that
/// return a `Result<(), Error>`, where the error type is an enum defined in
/// [`validation_errors::Error`].
///
/// When the feature flag `pgrx` is enabled, the function will be transformed
/// into a `pg_extern` function which returns `bool` instead of `Result<(),
/// validation_errors::Error>`. The function will return `true` if the
/// validation passes, and will return `false` if the validation fails. If the
/// validation fails, the function will also log an error message using the
/// `pgrx::error!` macro. The Error
///
/// The function is left unchanged when the `pgrx` feature flag is not enabled.
pub fn validation(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input_fn = parse_macro_input!(item as syn::ItemFn);

    // Extract parts of the function signature
    let vis = &input_fn.vis;
    let sig = &input_fn.sig;
    let fn_name = &sig.ident;

    // Methods wrapped with a `validation` attribute must be `pub` to be exposed as
    // a pg_extern.
    if !matches!(vis, syn::Visibility::Public(_)) {
        let error_message = format!(
            "Function `{fn_name}` must be public to be decorated with `validation` - add a `pub` keyword before the function definition if appropriate.",
        );
        return syn::Error::new(vis.span(), &error_message).to_compile_error().into();
    }

    // Methods wrapped with a `validation` attribute must not be async.
    if sig.asyncness.is_some() {
        let error_message =
            format!("Function `{fn_name}` must not be async to be decorated with `validation`.",);
        return syn::Error::new(sig.asyncness.span(), &error_message).to_compile_error().into();
    }

    // Since Postgres does not support generics, we need to ensure that the function
    // does not have any.
    if !sig.generics.params.is_empty() {
        let error_message = format!(
            "Function `{fn_name}` must not have any generic parameters to be decorated with `validation`.",
        );
        return syn::Error::new(sig.generics.span(), &error_message).to_compile_error().into();
    }

    // A validation only makes sense when the function receives one or more
    // arguments.
    if sig.inputs.is_empty() {
        let error_message = format!(
            "Function `{fn_name}` must have at least one argument to be decorated with `validation`.",
        );
        return syn::Error::new(sig.inputs.span(), &error_message).to_compile_error().into();
    }

    // We expect the function to return a `Result<(),
    // validation_errors::SingleFieldError>` or `Result<(),
    // validation_errors::DoubleFieldError>`.
    if !is_result_unit_pgrx_error(&sig.output) {
        let error_message = format!(
            "Function `{fn_name}` must return a `Result<(), validation_errors::SingleFieldError>` or `Result<(), validation_errors::DoubleFieldError>` to be decorated with `validation`, but returns `{output}`.",
            output = match &sig.output {
                syn::ReturnType::Type(_, ty) => quote!(#ty).to_string(),
                syn::ReturnType::Default => "()".to_string(),
            },
        );
        return syn::Error::new(sig.output.span(), &error_message).to_compile_error().into();
    }

    // We expect the function name to be snake_case.
    let function_name = fn_name.to_string();
    if !function_name.chars().all(|character: char| {
        character.is_ascii_lowercase() || character.is_numeric() || character == '_'
    }) {
        let error_message =
            format!("Function `{fn_name}` must be snake_case to be decorated with `validation`.",);
        return syn::Error::new(fn_name.span(), &error_message).to_compile_error().into();
    }

    // We expect the function name to start with `must_be_` or `must_not_be_`.
    if !VALID_PREFIXES.iter().any(|prefix| function_name.starts_with(prefix)) {
        let error_message = format!(
            "Function `{fn_name}` must start with any of {VALID_PREFIXES:?} to be decorated with `validation`.",
        );
        return syn::Error::new(fn_name.span(), &error_message).to_compile_error().into();
    }

    // We expect that a `validation` method has a doc comment.
    if input_fn.attrs.is_empty() {
        let error_message = format!(
            "Function `{fn_name}` must have a doc comment to be decorated with `validation`.",
        );
        return syn::Error::new(fn_name.span(), &error_message).to_compile_error().into();
    }

    // If the pgrx feature is enabled, transform the function.
    if cfg!(feature = "pgrx") {
        let inputs = &sig.inputs;

        // Collect argument names to be passed along when calling the inner function.
        let arg_names = inputs.iter().map(|arg| {
            match arg {
                syn::FnArg::Receiver(_) => quote! { self },
                syn::FnArg::Typed(pat_type) => {
                    let pat = &pat_type.pat;
                    quote! { #pat }
                }
            }
        });

        // Create a new identifier for the inner (renamed) function.
        let inner_fn_name = syn::Ident::new(&format!("__inner_{fn_name}"), fn_name.span());
        let mut inner_fn = input_fn.clone();
        inner_fn.sig.ident = inner_fn_name.clone();

        // It may be the case that the function calls other `must_be_` or `must_not_be_`
        // functions, which need to be converted into the `__inner_` version of
        // the function. We need to replace the function calls with the new
        // identifier, which is why we need to traverse the function body.
        let mut renamer = MethodRenamer;
        renamer.visit_item_fn_mut(&mut inner_fn);

        // Generate the wrapper function which will be exposed as a pg_extern.
        // It calls the inner function and converts the Result into a bool.
        let wrapper_fn = quote! {
            #[pgrx::pg_extern]
            pub fn #fn_name (#inputs) -> bool {
                match #inner_fn_name(#(#arg_names),*) {
                    Ok(()) => true,
                    Err(e) => {
                        pgrx::error!("Validation failed: {:?}", e);
                    }
                }
            }
        };

        // Combine the inner function and the new wrapper function.
        let output = quote! {
            #inner_fn
            #wrapper_fn
        };

        output.into()
    } else {
        // When the `pgrx` feature is not enabled, simply return the function unchanged.
        quote!(#input_fn).into()
    }
}

/// Determines if the return type is `Result<(),
/// validation_errors::SingleFieldError>` or `Result<(),
/// validation_errors::DoubleFieldError>`.
fn is_result_unit_pgrx_error(output: &syn::ReturnType) -> bool {
    if let syn::ReturnType::Type(_, ty) = output {
        if let syn::Type::Path(type_path) = &**ty {
            // Ensure it's a `Result<T, E>`
            if let Some(last_segment) = type_path.path.segments.last() {
                if last_segment.ident == "Result" {
                    if let syn::PathArguments::AngleBracketed(args) = &last_segment.arguments {
                        let mut iter = args.args.iter();
                        // First generic argument: Must be `()`
                        if let Some(syn::GenericArgument::Type(syn::Type::Tuple(tuple))) =
                            iter.next()
                        {
                            if !tuple.elems.is_empty() {
                                return false;
                            }
                        } else {
                            return false;
                        }

                        // Second generic argument: Must be
                        // `validation_errors::SingleFieldError`
                        // or `validation_errors::DoubleFieldError`
                        if let Some(syn::GenericArgument::Type(syn::Type::Path(error_path))) =
                            iter.next()
                        {
                            return is_pgrx_validation_error(error_path);
                        }
                    }
                }
            }
        }
    }
    false
}

/// Checks if a `TypePath` matches `validation_errors::Error`
fn is_pgrx_validation_error(path: &syn::TypePath) -> bool {
    path.path.segments.len() == 2
        && path.path.segments[0].ident == "validation_errors"
        && (path.path.segments[1].ident == "SingleFieldError"
            || path.path.segments[1].ident == "DoubleFieldError")
}
