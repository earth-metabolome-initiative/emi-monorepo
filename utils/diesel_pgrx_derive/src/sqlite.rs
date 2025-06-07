#![cfg(feature = "sqlite")]
//! Submodule providing the [`SQLite`] and the implementation of the
//! [`Backend`] trait.

use syn::Ident;

use crate::Backend;

/// Struct marker for the `SQLite` backend.
pub(crate) struct SQLite;

impl Backend for SQLite {
    fn sql_type_attribute(&self, _name: &str) -> proc_macro2::TokenStream {
        quote::quote! {
            #[diesel(sqlite_type(name = "Binary"))]
        }
    }

    fn to_sql_impl(&self, ident: &Ident) -> proc_macro2::TokenStream {
        quote::quote! {
            impl ::diesel_pgrx::diesel::serialize::ToSql<diesel_impls::#ident, ::diesel_pgrx::diesel::sqlite::Sqlite> for #ident
            {
                fn to_sql<'b>(&'b self, out: &mut ::diesel_pgrx::diesel::serialize::Output<'b, '_, ::diesel_pgrx::diesel::sqlite::Sqlite>,) -> ::diesel_pgrx::diesel::serialize::Result {
                    use std::io::Write;
                    let mut buffer = Vec::new();
                    ::diesel_pgrx::serde_cbor::to_writer(&mut buffer, self)?;
                    out.set_value(buffer);
                    Ok(::diesel_pgrx::diesel::serialize::IsNull::No)
                }
            }
        }
    }

    fn from_sql_impl(&self, ident: &Ident) -> proc_macro2::TokenStream {
        quote::quote! {
            impl ::diesel_pgrx::diesel::deserialize::FromSql<diesel_impls::#ident, ::diesel_pgrx::diesel::sqlite::Sqlite> for #ident
            {
                fn from_sql(mut raw: ::diesel_pgrx::diesel::sqlite::SqliteValue<'_, '_, '_>) -> ::diesel_pgrx::diesel::deserialize::Result<Self> {
                    Ok(::diesel_pgrx::serde_cbor::from_slice(raw.read_blob())?)
                }
            }
        }
    }
}
