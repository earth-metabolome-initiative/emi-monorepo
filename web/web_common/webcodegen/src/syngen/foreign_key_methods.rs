//! This module contains the implementation of the `Table` struct's
//! `foreign_key_methods` method, which implements the foreign key methods for a
//! table.
use diesel::PgConnection;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::{errors::WebCodeGenError, Table};

impl Table {
    /// Returns all of the implementations of `Foreign<F>` for the table struct.
    /// 
    /// # Arguments
    /// 
    /// * `conn` - A mutable reference to a `PgConnection`.
    /// 
    pub fn foreign_key_traits(
        &self,
        conn: &mut PgConnection,
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
                // If the foreign key is nullable, we don't want to write the trait for it.
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

        foreign_keys_and_tables
            .into_iter()
            .map(|(foreign_key_table, column)| {
                let foreign_key_struct_path = foreign_key_table.import_struct_path()?;

                let method_name: Ident = if column.column_name.ends_with("_id") {
                    Ident::new(&column.column_name[..column.column_name.len() - 3], proc_macro2::Span::call_site())
                } else {
                    Ident::new(&column.column_name, proc_macro2::Span::call_site())
                };

                Ok(quote! {
                    impl web_common_traits::prelude::Foreign<#foreign_key_struct_path> for #struct_path {
                        #[cfg(feature = "diesel")]
                        async fn foreign(&self, conn: &mut web_common_traits::prelude::DBConn) -> Result<Option<#foreign_key_struct_path>, diesel::result::Error> {
                            self.#method_name(conn).await
                        }
                    }
                })
            }).collect::<Result<TokenStream, WebCodeGenError>>()
    }

    /// Returns the Syn `TokenStream` for the foreign key methods.
    pub fn foreign_key_methods(
        &self,
        conn: &mut PgConnection,
    ) -> Result<TokenStream, WebCodeGenError> {
        self
            .foreign_keys(conn)?
            .into_iter()
            .map(|column| {
                let (foreign_key_table, _) = column.foreign_table(conn).unwrap().unwrap();
                let method_name: Ident = if column.column_name.ends_with("_id") {
                    Ident::new(&column.column_name[..column.column_name.len() - 3], proc_macro2::Span::call_site())
                } else {
                    Ident::new(&column.column_name, proc_macro2::Span::call_site())
                };
                let current_column_ident: Ident = column.sanitized_snake_case_ident()?;
                let foreign_key_struct_path = foreign_key_table.import_struct_path()?;

                // If the current column has a Nullable (Option) type, the return type of the method should be an Option
                let return_type_ident = if column.is_nullable() {
                    quote! { Option<#foreign_key_struct_path> }
                } else {
                    quote! { #foreign_key_struct_path }
                };

                // Analogously, we check before executing the query whether the current column is None. If so,
                // we return None as well.
                let column_value_retrieval = if column.is_nullable() {
                    quote! {
                        self.#current_column_ident.as_ref() else {
                            return Ok(None);
                        }
                    }
                } else {
                    quote! {
                        &self.#current_column_ident
                    }
                };

                let stricter_flag_name = if self.columns(conn)?.len() > foreign_key_table.columns(conn)?.len() {
                    self.diesel_feature_flag_name(conn)?
                } else {
                    foreign_key_table.diesel_feature_flag_name(conn)?
                };

                Ok(quote! {
                    #[cfg(feature = #stricter_flag_name)]
                    pub async fn #method_name(&self, conn: &mut web_common_traits::prelude::DBConn) -> Result<Option<#return_type_ident>, diesel::result::Error> {
                        <#foreign_key_struct_path as web_common_traits::prelude::Loadable>::load(#column_value_retrieval, conn).await
                    }
                })
            }).collect::<Result<TokenStream, WebCodeGenError>>()
    }
}
