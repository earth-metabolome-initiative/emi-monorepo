//! This module contains the implementation of the `Table` struct's
//! `foreign_key_methods` method, which implements the foreign key methods for a
//! table.
use diesel::PgConnection;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::{codegen::Syntax, errors::WebCodeGenError, Table};

impl Table {
    /// Returns all of the implementations of `Foreign<F>` for the table struct.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`.
    /// * `syntax` - The syntax to use for the code generation.
    pub fn foreign_key_traits(
        &self,
        conn: &mut PgConnection,
        syntax: &Syntax,
    ) -> Result<TokenStream, WebCodeGenError> {
        let struct_path = self.import_struct_path()?;
        let foreign_keys = self.foreign_keys(conn)?;

        // We only want to write the trait for foreign keys for tables
        // that appear ONCE, as otherwise the column of the trait would be ambiguous.
        let mut foreign_keys_and_tables = Vec::new();
        for foreign_key in foreign_keys {
            let (foreign_key_table, _) = foreign_key.foreign_table(conn).unwrap().unwrap();
            if &foreign_key_table == self {
                continue;
            }

            if foreign_key.is_nullable() {
                continue;
            }

            if foreign_keys_and_tables.iter().any(|(table, _)| table == &foreign_key_table) {
                // If we have already written the trait for this table, we remove the other
                // occurrences.
                foreign_keys_and_tables.retain(|(table, _)| table != &foreign_key_table);
                continue;
            }
            foreign_keys_and_tables.push((foreign_key_table, foreign_key));
        }

        let syntax_feature_flag = syntax.as_feature_flag();
        let connection = syntax.as_connection_type();

        foreign_keys_and_tables
            .into_iter()
            .map(|(foreign_key_table, column)| {
                let foreign_key_struct_path = foreign_key_table.import_struct_path()?;
                let method_ident: Ident = column.getter_ident()?;

                Ok(quote! {
                    #syntax_feature_flag
                    impl web_common_traits::prelude::Foreign<#foreign_key_struct_path> for #struct_path {
                        type Conn = #connection;

                        async fn foreign(&self, conn: &mut Self::Conn) -> Result<#foreign_key_struct_path, diesel::result::Error> {
                            self.#method_ident(conn).await
                        }
                    }
                })
            }).collect::<Result<TokenStream, WebCodeGenError>>()
    }

    /// Returns the Syn `TokenStream` for the foreign key methods.
    pub fn foreign_key_methods(
        &self,
        conn: &mut PgConnection,
        syntax: &Syntax,
    ) -> Result<TokenStream, WebCodeGenError> {
        let feature_flag = syntax.as_feature_flag();
        let connection = syntax.as_connection_type();

        self
            .foreign_keys(conn)?
            .into_iter()
            .map(|column| {
                let (foreign_key_table, _) = column.foreign_table(conn).unwrap().unwrap();
                let method_ident: Ident = column.getter_ident()?;
                let current_column_ident: Ident = column.snake_case_ident()?;
                let foreign_key_struct_path = foreign_key_table.import_struct_path()?;

                let optional = if column.is_nullable() {
                    quote! { .map(Some) }
                } else {
                    TokenStream::new()
                };

                let return_statement: syn::Type = if column.is_nullable() {
                    syn::parse_quote! { Option<#foreign_key_struct_path> }
                } else {
                    foreign_key_struct_path.clone()
                };

                // Analogously, we check before executing the query whether the current column is None.
                // If so, we return None as well.

                let (column_value_retrieval, column_attribute) = if column.is_nullable() {
                    (quote! {
                        let Some(#current_column_ident) = self.#current_column_ident.as_ref() else {
                            return Ok(None);
                        };
                    }, quote! { #current_column_ident })
                } else {
                    (TokenStream::new(), quote! { &self.#current_column_ident })
                };

                Ok(quote! {
                    #feature_flag
                    pub async fn #method_ident(&self, conn: &mut #connection) -> Result<#return_statement, diesel::result::Error> {
                        use diesel_async::RunQueryDsl;
                        use diesel::associations::HasTable;
                        use diesel::QueryDsl;
                        #column_value_retrieval
                        #foreign_key_struct_path::table()
                            .find(#column_attribute)
                            .first::<#foreign_key_struct_path>(conn)
                            .await
                            #optional
                    }
                })
            }).collect::<Result<TokenStream, WebCodeGenError>>()
    }
}
