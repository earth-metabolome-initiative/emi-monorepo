//! Submodule providing a function to extract the arguments which
//! are not a diesel DB connection from a function signature.

use syn::Ident;

use crate::utils::is_conn_argument::is_conn_argument;

/// Returns a vector of strings representing the names of the arguments
/// that are not a diesel DB connection.
///
/// # Arguments
///
/// * `inputs` - A reference to a `Punctuated<FnArg, Comma>` representing
///   the inputs of the function.
pub(crate) fn non_conn_arguments_idents(
    inputs: &syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma>,
) -> Vec<&Ident> {
    let mut names = Vec::new();
    for input in inputs.iter() {
        if let syn::FnArg::Typed(syn::PatType { pat, ty, .. }) = input {
            if !is_conn_argument(ty) {
                if let syn::Pat::Ident(syn::PatIdent { ident, .. }) = &**pat {
                    names.push(ident);
                }
            }
        }
    }
    names
}

/// Returns the first non-connection argument's identifier, if any.
///
/// # Arguments
///
/// * `inputs` - A reference to a `Punctuated<FnArg, Comma>` representing
///   the inputs of the function.
pub(crate) fn first_non_conn_argument_ident(
    inputs: &syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma>,
) -> Option<&Ident> {
    let names = non_conn_arguments_idents(inputs);
    names.first().copied()
}
