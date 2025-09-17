//! Submodule  providing a method to generate the trait `From` impl to convert
//! the builder of the associated table to an `IdOrBuilder` enum.

use diesel::PgConnection;
use proc_macro2::TokenStream;
use quote::quote;

use crate::{Codegen, Table, errors::WebCodeGenError};

impl Codegen<'_> {
    pub(super) fn from_builder_to_id_or_builder_impl(
        &self,
        table: &Table,
        conn: &mut PgConnection,
    ) -> Result<TokenStream, WebCodeGenError> {
        // If the table has a composite primary key, we do not generate this impl.
        if table.has_composite_primary_key(conn)? {
            return Ok(TokenStream::new());
        }

        let primary_key_type = table.primary_key_type(conn)?;
        let builder_ident = table.insertable_builder_ident()?;

        Ok(quote! {
            impl From<#builder_ident> for web_common_traits::database::IdOrBuilder<#primary_key_type, #builder_ident> {
                fn from(builder: #builder_ident) -> Self {
                    Self::Builder(builder)
                }
            }
        })
    }
}
