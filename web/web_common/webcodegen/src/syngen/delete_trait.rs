use diesel::PgConnection;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::{errors::WebCodeGenError, Table};

impl Table {
    /// Returns the Syn `TokenStream` for the delete method of the table.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A `TokenStream` representing the delete method.
    ///
    /// # Errors
    ///
    /// * If the snake case name cannot be generated.
    /// * If the primary key columns cannot be loaded from the database.
    /// * If the diesel feature flag name cannot be generated.
    pub fn delete_method(&self, conn: &mut PgConnection) -> Result<TokenStream, WebCodeGenError> {
        let sanitized_table_name =
            Ident::new(&self.snake_case_name()?, proc_macro2::Span::call_site());
        let primary_key_columns = self.primary_key_columns(conn).unwrap();

        let where_clause = primary_key_columns
            .iter()
            .map(|column| {
                let column_name: Ident =
                    Ident::new(&column.column_name, proc_macro2::Span::call_site());
                quote! {
                    #sanitized_table_name::#column_name.eq(&self.#column_name)
                }
            })
            .collect::<Vec<_>>();

        // Join the where clauses with an and
        let where_clause = where_clause
            .into_iter()
            .reduce(|a, b| quote! { diesel::BoolExpressionMethods::and(#a, #b) })
            .unwrap();

        let columns_feature_flag_name = self.diesel_feature_flag_name(conn)?;

        Ok(quote! {
            #[cfg(feature = #columns_feature_flag_name)]
            pub async fn delete(&self, conn: &mut web_common_traits::prelude::DBConn) -> Result<usize, diesel::result::Error> {
                diesel::delete(#sanitized_table_name::table.filter(#where_clause)).execute(conn).await
            }
        })
    }
}
