//! Submodule providing methods to write out the schema of a table.

use diesel::PgConnection;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::{Table, errors::WebCodeGenError, traits::TableLike};

impl Table {
    /// Returns the Syn `TokenStream` for the diesel schema definition for the
    /// table.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A `TokenStream` representing the diesel schema definition for the table.
    ///
    /// # Errors
    ///
    /// * If the snake case name cannot be generated.
    /// * If the columns cannot be loaded from the database.
    /// * If the diesel type for a column cannot be loaded.
    /// * If the primary key columns cannot be loaded from the database.
    /// * If the diesel feature flag name cannot be generated.
    pub fn to_schema(&self, conn: &mut PgConnection) -> Result<TokenStream, WebCodeGenError> {
        let original_table_name = &self.table_name;
        let sanitized_table_name_ident =
            Ident::new(&self.snake_case_name()?, proc_macro2::Span::call_site());

        let mut columns = Vec::new();
        for column in self.columns(conn)? {
            let original_column_name = &column.column_name;
            let column_attribute: Ident = column.snake_case_ident()?;
            let column_type = column.diesel_type(conn)?;
            columns.push(if original_column_name == &column_attribute.to_string() {
                quote! {
                    #column_attribute -> #column_type
                }
            } else {
                quote! {
                    #[sql_name = #original_column_name]
                    #column_attribute -> #column_type
                }
            });
        }
        let primary_key_names = self
            .primary_key_columns(conn)?
            .into_iter()
            .map(|column| Ident::new(&column.column_name, proc_macro2::Span::call_site()))
            .collect::<Vec<Ident>>();

        let primary_key_names = if primary_key_names.is_empty() {
            TokenStream::new()
        } else {
            quote! {
                (#(#primary_key_names),*)
            }
        };

        let sql_name = if self.has_snake_case_name()? {
            TokenStream::new()
        } else {
            quote! {#[sql_name = #original_table_name]}
        };

        Ok(quote! {
            diesel::table! {
                #sql_name
                #sanitized_table_name_ident #primary_key_names {
                    #(#columns),*
                }
            }
        })
    }
}
