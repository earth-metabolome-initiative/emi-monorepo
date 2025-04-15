//! Primary method to convert a Table to a struct and associated impls.

use diesel::PgConnection;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::{Table, errors::WebCodeGenError};

impl Table {
    /// Returns the Syn `TokenStream` for the struct definition.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * If the provided connection is not valid.
    /// * If the number of columns exceeds 128.
    pub fn to_syn(&self, conn: &mut PgConnection) -> Result<TokenStream, WebCodeGenError> {
        if self.columns(conn)?.len() > 128 {
            return Err(WebCodeGenError::ExcessiveNumberOfColumns(
                Box::new(self.clone()),
                self.columns(conn)?.len(),
            ));
        }

        let table_path = self.import_diesel_path()?;
        let struct_name: Ident = self.struct_ident()?;

        let attributes = self
            .columns(conn)?
            .into_iter()
            .map(|column| {
                let column_attribute: Ident = column.snake_case_ident()?;
                let column_type = column.rust_data_type(conn)?;
                Ok(quote! {
                    pub #column_attribute: #column_type
                })
            })
            .collect::<Result<Vec<TokenStream>, WebCodeGenError>>()?;

        let diesel_derives_decorator = self.diesel_derives_decorator(conn)?;
        let primary_key_decorator = self.primary_key_decorator(conn)?;

        Ok(quote! {
            #[derive(Debug, Clone)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            #diesel_derives_decorator
            #primary_key_decorator
            #[diesel(table_name = #table_path)]
            pub struct #struct_name {
                #(#attributes),*
            }
        })
    }
}
