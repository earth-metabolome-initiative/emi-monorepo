//! Submodule implementing the methods relative to traits.

use diesel::PgConnection;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::{errors::WebCodeGenError, Table};

const ATTRIBUTE_TRAITS: &[(&str, &str)] = &[("Described", "description")];

impl Table {
    /// Returns the Syn `TokenStream` for the implementation of the `Described`
    /// trait.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * If the columns of the table cannot be retrieved.
    pub fn attribute_traits_impl(
        &self,
        conn: &mut PgConnection,
    ) -> Result<TokenStream, WebCodeGenError> {
        let columns = self.columns(conn)?;
        let struct_ident = self.struct_ident()?;
        Ok(ATTRIBUTE_TRAITS
            .iter()
            .map(|(trait_name, attribute_name)| {
                let Some(column) = columns
                    .iter()
                    .find(|column| &column.snake_case_name().unwrap_or_default() == attribute_name)
                else {
                    return Ok(TokenStream::new());
                };
                let reference_type = column.rust_ref_data_type(conn)?;
                let trait_ident = Ident::new(trait_name, proc_macro2::Span::call_site());
                let method_name = Ident::new(attribute_name, proc_macro2::Span::call_site());
                Ok(quote! {
                    impl web_common_traits::prelude::#trait_ident for #struct_ident {
                        fn #method_name(&self) -> #reference_type {
                            &self.#method_name
                        }
                    }
                })
            })
            .collect::<Result<TokenStream, WebCodeGenError>>()?)
    }
}
