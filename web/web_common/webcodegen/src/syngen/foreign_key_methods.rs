//! This module contains the implementation of the `Table` struct's `foreign_key_methods` method,
//! which implements the foreign key methods for a table.
use diesel::PgConnection;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::{errors::WebCodeGenError, Table};

impl Table {
    /// Returns the Syn `TokenStream` for the foreign key methods.
    pub fn foreign_key_methods<'a>(
        &'a self,
        conn: &'a mut PgConnection,
    ) -> Result<Vec<TokenStream>, WebCodeGenError> {
        self
            .foreign_keys(conn)?
            .into_iter()
            .map(|column| {
                let (foreign_key_table, foreign_key_column) = column.foreign_table(conn).unwrap().unwrap();
                let foreign_key_table_name = Ident::new(&foreign_key_table.snake_case_name()?, proc_macro2::Span::call_site());
                let foreign_key_column_name: Ident = Ident::new(&foreign_key_column.column_name, proc_macro2::Span::call_site());
                let method_name: Ident = if column.column_name.ends_with("_id") {
                    Ident::new(&column.column_name[..column.column_name.len() - 3], proc_macro2::Span::call_site())
                } else {
                    Ident::new(&column.column_name, proc_macro2::Span::call_site())
                };
                let current_column_ident: Ident = column.sanitized_snake_case_ident()?;
                let foreign_key_struct_name: Ident = Ident::new(&foreign_key_table.struct_name()?, proc_macro2::Span::call_site());

                // If the current column has a Nullable (Option) type, the return type of the method should be an Option
                let return_type_ident = if column.is_nullable() {
                    quote! { Option<#foreign_key_struct_name> }
                } else {
                    quote! { #foreign_key_struct_name }
                };

                // Analogously, we check before executing the query whether the current column is None. If so,
                // we return None as well.
                let column_value_retrieval = if column.is_nullable() {
                    quote! {
                        let #current_column_ident = if let Some(#current_column_ident) = self.#current_column_ident.as_ref() {
                            #current_column_ident
                        } else {
                            return Ok(None);
                        };
                    }
                } else {
                    TokenStream::new()
                };

                // It follows that we need to determine whether the right term of the equality for
                // the filter should be prefixed with 'self.' or not (if the column is nullable).
                let filter_statement = if column.is_nullable() {
                    quote! { #foreign_key_table_name::#foreign_key_column_name.eq(&#current_column_ident) }
                } else {
                    quote! { #foreign_key_table_name::#foreign_key_column_name.eq(&self.#current_column_ident) }
                };

                // Finally, when we are returning a Result<Option<TableStructType>, ...>, 
                // we need to wrap the result of the query in a Some.
                let map_ops = if column.is_nullable() {
                    quote! { .map(Some) }
                } else {
                    TokenStream::new()
                };

                let stricter_flag_name = if self.columns(conn)?.len() > foreign_key_table.columns(conn)?.len() {
                    self.diesel_feature_flag_name(conn)?
                } else {
                    foreign_key_table.diesel_feature_flag_name(conn)?
                };

                Ok(quote! {
                    #[cfg(feature = #stricter_flag_name)]
                    pub async fn #method_name(&self, conn: &mut diesel_async::AsyncPgConnection) -> Result<#return_type_ident, diesel::result::Error> {
                        #column_value_retrieval
                        #foreign_key_table_name::table
                            .filter(#filter_statement)
                            .select(<#foreign_key_struct_name as diesel::SelectableHelper<diesel::pg::Pg>>::as_select())
                            .first::<#foreign_key_struct_name>(conn)
                            .await
                            #map_ops
                    }
                })
            }).collect::<Result<Vec<TokenStream>, WebCodeGenError>>()
    }
}
