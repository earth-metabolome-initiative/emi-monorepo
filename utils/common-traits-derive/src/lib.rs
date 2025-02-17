#![doc = include_str!("../README.md")]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Generics, ImplGenerics, Item, TypeGenerics};

#[proc_macro_attribute]
/// Derive the `Basic` trait for a struct or enum.
pub fn basic(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as Item);

    let (name, mut generics) = match input.clone() {
        Item::Struct(s) => (s.ident, s.generics),
        Item::Enum(e) => (e.ident, e.generics),
        _ => panic!("Only `structs` and `enums` are supported"),
    };

    let (impl_generics, ty_generics, where_clause) = add_trait_constraints(&mut generics);

    let expanded = quote! {
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
        #input

        impl #impl_generics common_traits::basic::Basic for #name #ty_generics #where_clause {}
    };

    TokenStream::from(expanded)
}

/// Add necessary trait constraints to generic parameters.
fn add_trait_constraints<'a>(
    generics: &'a mut Generics,
) -> (ImplGenerics<'a>, TypeGenerics<'a>, Option<&'a syn::WhereClause>) {
    for param in &mut generics.params {
        if let syn::GenericParam::Type(ty) = param {
            ty.bounds.push(syn::parse_quote!(common_traits::basic::Basic));
        }
    }
    generics.split_for_impl()
}
