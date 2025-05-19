#![cfg(feature = "postgres")]
//! Submodule providing the [`Postgres`] and the implementation of the
//! [`Backend`] trait.

use syn::Ident;

use crate::Backend;

/// Struct marker for the Postgres backend.
pub(crate) struct Postgres;

impl Backend for Postgres {
    fn sql_type_attribute(&self, name: &str) -> proc_macro2::TokenStream {
        let lowercase_name = name.to_lowercase();
        quote::quote! {
            #[diesel(postgres_type(name = #lowercase_name))]
        }
    }

    fn to_sql_impl(&self, ident: &Ident) -> proc_macro2::TokenStream {
        quote::quote! {
            impl ::diesel_pgrx::diesel::serialize::ToSql<diesel_impls::#ident, ::diesel_pgrx::diesel::pg::Pg> for #ident
            {
                fn to_sql<'b>(&'b self, out: &mut ::diesel_pgrx::diesel::serialize::Output<'b, '_, ::diesel_pgrx::diesel::pg::Pg>) -> ::diesel_pgrx::diesel::serialize::Result {
                    use std::io::Write;
                    ::diesel_pgrx::serde_cbor::to_writer(out, self)?;
                    Ok(::diesel_pgrx::diesel::serialize::IsNull::No)
                }
            }
        }
    }

    fn from_sql_impl(&self, ident: &Ident) -> proc_macro2::TokenStream {
        quote::quote! {
            impl ::diesel_pgrx::diesel::deserialize::FromSql<diesel_impls::#ident, ::diesel_pgrx::diesel::pg::Pg> for #ident
            {
                fn from_sql(raw: ::diesel_pgrx::diesel::pg::PgValue<'_>) -> ::diesel_pgrx::diesel::deserialize::Result<Self> {
                    Ok(::diesel_pgrx::serde_cbor::from_slice(raw.as_bytes())?)
                }
            }
        }
    }
}
