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
    ///
    /// # Errors
    ///
    /// * If the connection to the database fails.
    /// * If the foreign key traits cannot be generated.
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
            let Some((foreign_key_table, _)) = foreign_key.foreign_table(conn)? else {
                continue;
            };

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
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`.
    /// * `syntax` - The syntax to use for the code generation.
    ///
    /// # Errors
    ///
    /// * If the connection to the database fails.
    /// * If the foreign key methods cannot be generated.
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
                let Some((foreign_key_table, foreign_key_column)) = column.foreign_table(conn)? else {
                    return Ok(TokenStream::new());
                };
                let method_ident: Ident = column.getter_ident()?;

                let current_column_ident: Ident = column.snake_case_ident()?;
                let foreign_key_column_ident: Ident = foreign_key_column.snake_case_ident()?;
                let foreign_key_struct_path = foreign_key_table.import_struct_path()?;
                let foreign_key_diesel_path = foreign_key_table.import_diesel_path()?;

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

                // We build the where statement to filter for the `from_method_ident`
                let self_where_statement = quote!{
                    #foreign_key_diesel_path::dsl::#foreign_key_column_ident.eq(#column_attribute)
                };

                Ok(quote! {
                    #feature_flag
                    pub async fn #method_ident(&self, conn: &mut #connection) -> Result<#return_statement, diesel::result::Error> {
                        use diesel_async::RunQueryDsl;
                        use diesel::associations::HasTable;
                        use diesel::{QueryDsl, ExpressionMethods};
                        #column_value_retrieval
                        #foreign_key_struct_path::table()
                            .filter(#self_where_statement)
                            .first::<#foreign_key_struct_path>(conn)
                            .await
                            #optional
                    }
                })
            }).collect::<Result<TokenStream, WebCodeGenError>>()
    }

    /// Returns the Syn `TokenStream` for the from foreign key methods.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`.
    /// * `syntax` - The syntax to use for the code generation.
    ///
    /// # Errors
    ///
    /// * If the connection to the database fails.
    /// * If the foreign key methods cannot be generated.
    pub fn from_foreign_key_methods(
        &self,
        conn: &mut PgConnection,
        syntax: &Syntax,
    ) -> Result<TokenStream, WebCodeGenError> {
        let feature_flag = syntax.as_feature_flag();
        let connection = syntax.as_connection_type();
        let current_table_diesel_path = self.import_diesel_path()?;

        self
            .foreign_keys(conn)?
            .into_iter()
            .map(|column| {
                let Some((foreign_key_table, foreign_key_column)) = column.foreign_table(conn)? else {
                    return Ok(TokenStream::new());
                };
                let from_method_ident: Ident = Ident::new(&format!("from_{}", column.column_name), proc_macro2::Span::call_site());

                let current_column_ident: Ident = column.snake_case_ident()?;
                let foreign_key_column_ident: Ident = foreign_key_column.snake_case_ident()?;
                let foreign_key_struct_path = foreign_key_table.import_struct_path()?;

                // We build the where statement to filter for the `from_method_ident`
                let where_statement = if foreign_key_column.supports_copy(conn)? {
                    quote!{
                        #current_table_diesel_path::dsl::#current_column_ident.eq(#current_column_ident.#foreign_key_column_ident)
                    }
                } else {
                    quote!{
                        #current_table_diesel_path::dsl::#current_column_ident.eq(&#current_column_ident.#foreign_key_column_ident)
                    }
                };

                // Depending on whether the column represents a unique constraint (or a primary key)
                // or something which does not have a 1-1 relationship, we need to return either a
                // `Self` (when it is not optional and it is unique) or an `Option<Self>` (when it is
                // optional and unique) or yet again a `Vec<Self>` (when it is not unique).

                let (use_statement, return_type, loading_statement) = match (column.is_nullable(), column.is_unique(conn)?) {
                    (true, true) => {
                        (
                            quote! { use diesel::{QueryDsl, ExpressionMethods, OptionalExtension};},
                            quote! { Option<Self> },
                            quote! { .first::<Self>(conn).await.optional() }
                        )
                    }
                    (false, true) => {
                        (
                            quote! { use diesel::{QueryDsl, ExpressionMethods};},
                            quote! { Self }, quote! { .first::<Self>(conn).await }
                        )
                    }
                    (_, false) => {
                        (
                            quote! { use diesel::{QueryDsl, ExpressionMethods};},
                            quote! { Vec<Self> },
                            quote! { .load::<Self>(conn).await }
                        )
                    }
                };

                Ok(quote! {
                    #feature_flag
                    pub async fn #from_method_ident(conn: &mut #connection, #current_column_ident: &#foreign_key_struct_path) -> Result<#return_type, diesel::result::Error> {
                        use diesel_async::RunQueryDsl;
                        use diesel::associations::HasTable;
                        #use_statement
                        Self::table()
                            .filter(#where_statement)
                            #loading_statement
                    }
                })
            }).collect::<Result<TokenStream, WebCodeGenError>>()
    }
}
