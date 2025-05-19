#![doc = include_str!("../README.md")]

use quote::quote;
use syn::{DeriveInput, parse_macro_input};

mod traits;
use traits::Backend;
mod postgres;
mod sqlite;

#[proc_macro_derive(DieselPGRX)]
/// Derives the `DieselPGRX` trait for the given struct or enum.
pub fn diesel_pgrx_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);

    #[cfg(any(feature = "postgres", feature = "sqlite"))]
    let mut backends: Vec<Box<dyn Backend>> = Vec::new();
    #[cfg(not(any(feature = "postgres", feature = "sqlite")))]
    let backends: Vec<Box<dyn Backend>> = Vec::new();

    let name = ident.to_string();

    #[cfg(feature = "postgres")]
    backends.push(Box::new(postgres::Postgres));

    #[cfg(feature = "sqlite")]
    backends.push(Box::new(sqlite::SQLite));

    let attributes = backends.iter().map(|backend| {
        backend.sql_type_attribute(&name)
    });

    let to_sql_impls = backends.iter().map(|backend| {
        backend.to_sql_impl(&ident)
    });

    let from_sql_impls = backends.iter().map(|backend| {
        backend.from_sql_impl(&ident)
    });

    quote! {
        impl ::diesel_pgrx::DieselPGRX for #ident {}

        #[doc(hidden)]
        pub mod diesel_impls {
            #[derive(
                Debug, Clone, Copy, Default, ::diesel_pgrx::diesel::query_builder::QueryId, ::diesel_pgrx::diesel::sql_types::SqlType,
            )]
            #(#attributes)*
            pub struct #ident;
        }

        #(#to_sql_impls)*
        #(#from_sql_impls)*
    }.into()
}
