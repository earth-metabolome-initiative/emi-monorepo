//! This module contains the implementation of the `Table` struct's
//! `foreign_key_methods` method, which implements the foreign key methods for a
//! table.
use diesel::PgConnection;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::{Column, Table, errors::WebCodeGenError, traits::TableLike};

impl Table {
    /// Returns the Syn `TokenStream` for the foreign key methods.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`.
    ///
    /// # Errors
    ///
    /// * If the connection to the database fails.
    /// * If the foreign key methods cannot be generated.
    pub fn foreign_key_methods(
        &self,
        conn: &mut PgConnection,
    ) -> Result<TokenStream, WebCodeGenError> {
        let mut foreign_key_methods = TokenStream::new();

        for foreign_key_constraint in self.foreign_keys(conn)? {
            if foreign_key_constraint.is_same_as_constraint(conn)?.is_some() {
                continue;
            }

            let foreign_key_table = foreign_key_constraint.foreign_table(conn)?;

            let columns = foreign_key_constraint.columns(conn)?;

            let method_ident: Ident = foreign_key_constraint.constraint_ident(conn)?;

            let foreign_key_struct_path = foreign_key_table.import_struct_path()?;

            let (return_statement, optional) = if columns.iter().any(Column::is_nullable) {
                (syn::parse_quote! { Option<#foreign_key_struct_path> }, quote! { .map(Some) })
            } else {
                (foreign_key_struct_path.clone(), TokenStream::new())
            };

            // Analogously, we check before executing the query whether the current column
            // is None. If so, we return None as well.
            let mut column_values_retrieval = TokenStream::new();

            for column in &columns {
                let current_column_ident: Ident = column.snake_case_ident()?;

                if column.is_nullable() {
                    column_values_retrieval.extend(quote! {
                        let Some(#current_column_ident) = self.#current_column_ident else {
                            return Ok(None);
                        };
                    });
                }
            }

            let bool_expression_methods = foreign_key_constraint
                .is_composite(conn)?
                .then(|| quote! { use diesel::BoolExpressionMethods; });

            if foreign_key_constraint.is_foreign_primary_key(conn)? {
                let primary_key = columns
                    .iter()
                    .map(|c| {
                        let column_ident = c.snake_case_ident()?;
                        if c.is_nullable() {
                            Ok(quote! { #column_ident })
                        } else {
                            Ok(quote! { self.#column_ident })
                        }
                    })
                    .collect::<Result<Vec<_>, WebCodeGenError>>()?;

                let formatted_primary_key = if primary_key.len() == 1 {
                    quote! { #(#primary_key)* }
                } else {
                    quote! { (#(#primary_key),*) }
                };

                foreign_key_methods.extend(quote! {
                    pub fn #method_ident<C: diesel::connection::LoadConnection>(
                        &self, conn: &mut C
                    ) -> Result<#return_statement, diesel::result::Error>
                        where
                            #foreign_key_struct_path: diesel::Identifiable,
                            <#foreign_key_struct_path as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<<#foreign_key_struct_path as diesel::Identifiable>::Id>,
                            <<#foreign_key_struct_path as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<<#foreign_key_struct_path as diesel::Identifiable>::Id>>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
                            <<<#foreign_key_struct_path as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<<#foreign_key_struct_path as diesel::Identifiable>::Id>>::Output as diesel::query_dsl::methods::LimitDsl>::Output:
                                for<'a> diesel::query_dsl::LoadQuery<'a, C, #foreign_key_struct_path>,
                    {
                        use diesel::associations::HasTable;
                        use diesel::{RunQueryDsl, QueryDsl};
                        #column_values_retrieval
                        RunQueryDsl::first(QueryDsl::find(#foreign_key_struct_path::table(), #formatted_primary_key), conn)#optional
                    }
                });
            } else {
                let where_statement = foreign_key_constraint.where_statement(true, true, conn)?;
                foreign_key_methods.extend(quote! {
                    #[cfg(feature = "postgres")]
                    pub fn #method_ident(
                        &self, conn: &mut diesel::PgConnection
                    ) -> Result<#return_statement, diesel::result::Error> {
                        use diesel::RunQueryDsl;
                        use diesel::associations::HasTable;
                        use diesel::{QueryDsl, ExpressionMethods};
                        #bool_expression_methods
                        #column_values_retrieval
                        #foreign_key_struct_path::table()
                            .filter(#where_statement)
                            .first::<#foreign_key_struct_path>(conn)
                            #optional
                    }
                });
            }
        }

        Ok(foreign_key_methods)
    }
}
