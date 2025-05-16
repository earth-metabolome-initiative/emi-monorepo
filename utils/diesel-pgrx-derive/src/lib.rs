#![doc = include_str!("../README.md")]

use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(DieselPGRX)]
/// Derives the `DieselPGRX` trait for the given struct or enum.
pub fn diesel_pgrx_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);

    let ident_as_string = ident.to_string().to_lowercase();

    quote! {
        impl diesel_pgrx::DieselPGRX for #ident {}

        pub mod diesel_impls {
            #[derive(
                Debug, Clone, Copy, Default, diesel_pgrx::diesel::query_builder::QueryId, diesel_pgrx::diesel::sql_types::SqlType,
            )]
            #[cfg_attr(
                feature = "postgres",
                diesel(postgres_type(name = #ident_as_string))
            )]
            #[cfg_attr(feature = "sqlite", diesel(sqlite_type(name = "TEXT")))]
            pub struct #ident;
        }

        #[cfg(feature = "postgres")]
        impl diesel_pgrx::diesel::serialize::ToSql<diesel_impls::#ident, diesel_pgrx::diesel::pg::Pg> for #ident
        {
            fn to_sql<'b>(&'b self, out: &mut diesel_pgrx::diesel::serialize::Output<'b, '_, diesel_pgrx::diesel::pg::Pg>) -> diesel_pgrx::diesel::serialize::Result {
                use diesel_pgrx::cbor_encode;
                use std::io::Write;

                let buf: Vec<u8> = cbor_encode(self);
                out.write_all(&buf)?;

                Ok(diesel::serialize::IsNull::No)
            }
        }

        #[cfg(feature = "postgres")]
        impl diesel_pgrx::diesel::deserialize::FromSql<diesel_impls::#ident, diesel_pgrx::diesel::pg::Pg> for #ident
        {
            fn from_sql(raw: diesel_pgrx::diesel::pg::PgValue<'_>) -> diesel_pgrx::diesel::deserialize::Result<Self> {
                todo!("Implement the from_sql method for {}", stringify!(#ident));
            }
        }
    }.into()
}
