#![doc = include_str!("../README.md")]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Item, Ident};

#[proc_macro_attribute]
/// Derive the `Basic` trait for a struct.
pub fn basic(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as Item);
    let name: Ident = match &input {
        Item::Struct(s) => s.ident.clone(),
        Item::Enum(e) => e.ident.clone(),
        _ => panic!("Only `structs` and `enums` are supported"),
    };

    let expanded = quote! {
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
        #input

        impl common_traits::basic::Basic for #name {}
    };

    TokenStream::from(expanded)
}
