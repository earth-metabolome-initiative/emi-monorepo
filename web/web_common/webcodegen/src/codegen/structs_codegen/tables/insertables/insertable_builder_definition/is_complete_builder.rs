//! Submodule providing the generation of the `try_insert` method for the
//! insertable builder struct.

use diesel::PgConnection;
use proc_macro2::TokenStream;
use quote::quote;

use crate::{Codegen, Table, errors::WebCodeGenError, traits::TableLike};

impl Codegen<'_> {
    /// Returns the implementation of the `IsCompleteBuilder` trait for the
    /// insertable builder of the provided table.
    ///
    /// # Arguments
    ///
    /// * `table` - The table for which the implementation is generated.
    /// * `conn` - The database connection to use for querying the database.
    ///
    /// # Errors
    ///
    /// If the provided connection to the database fails.
    pub(super) fn generate_is_complete_builder_implementation(
        &self,
        table: &Table,
        conn: &mut PgConnection,
    ) -> Result<TokenStream, WebCodeGenError> {
        let builder_ident = table.insertable_builder_ty()?;
        let mut completeness_statements = Vec::new();
        let mut where_constraints = Vec::new();
        let mut table_generics = Vec::new();

        // For each extension foreign key, we need to call recursively the `is_complete`
        // method
        for foreign_key in table.extension_foreign_keys(conn)? {
            let foreign_table = foreign_key.foreign_table(conn)?;
            let foreign_table_ident = foreign_table.struct_ident()?;
            table_generics.push(quote! { #foreign_table_ident });
            let foreign_key_ident = foreign_key.constraint_ident(conn)?;
            completeness_statements.push(quote! {
                self.#foreign_key_ident.is_complete()
            });
            where_constraints.push(quote! {
                #foreign_table_ident: common_traits::builder::IsCompleteBuilder
            });
        }

        // For each of the other columns, if they are not optional, we need to check if
        // they are complete
        let insertable_columns = table.insertable_columns(conn, false)?;
        let number_of_insertable_columns = insertable_columns.len();
        for column in insertable_columns {
            // If the column is nullable, we do not need to check its completeness
            if column.is_nullable() {
                continue;
            }

            // Otherwise, we need to check if the column is complete, i.e. whether
            // it has been set in the builder
            let column_ident = column.snake_case_ident()?;

            let foreign_definer_idents = column
                .foreign_definer_columns(conn)?
                .into_iter()
                .map(|c| {
                    let ident = c.snake_case_ident()?;

                    Ok(
                        if let Some((_partial_builder_kind, _, partial_builder_constraint)) =
                            c.requires_partial_builder(conn)?
                        {
                            let foreign_table = partial_builder_constraint.foreign_table(conn)?;
                            let foreign_builder_type = foreign_table.insertable_builder_ty()?;
                            where_constraints.push(quote! {
                                #foreign_builder_type: common_traits::builder::IsCompleteBuilder
                            });
                            quote! { self.#ident.is_complete() }
                        } else {
                            quote! { self.#ident.is_some() }
                        },
                    )
                })
                .collect::<Result<Vec<_>, WebCodeGenError>>()?;

            let maybe_foreign_definer = if foreign_definer_idents.is_empty() {
                None
            } else {
                Some(quote! {
                    #(#foreign_definer_idents)||*
                })
            };

            completeness_statements.push(
                if let Some((partial_builder_kind, _, partial_builder_constraint)) =
                    column.requires_partial_builder(conn)?
                {
                    let foreign_table = partial_builder_constraint.foreign_table(conn)?;
                    let foreign_builder_type = foreign_table.insertable_builder_ty()?;
                    where_constraints.push(quote! {
                        #foreign_builder_type: common_traits::builder::IsCompleteBuilder
                    });
                    if let Some(foreign_definer) = maybe_foreign_definer {
                        let statement = quote! {
                            self.#column_ident.is_complete() || #foreign_definer
                        };
                        if number_of_insertable_columns > 1 {
                            quote! { (#statement) }
                        } else {
                            statement
                        }
                    } else {
                        quote! {
                            self.#column_ident.is_complete()
                        }
                    }
                } else {
                    if let Some(foreign_definer) = maybe_foreign_definer {
                        let statement = quote! {
                            self.#column_ident.is_some() || #foreign_definer
                        };
                        if number_of_insertable_columns > 1 {
                            quote! { (#statement) }
                        } else {
                            statement
                        }
                    } else {
                        quote! {
                            self.#column_ident.is_some()
                        }
                    }
                },
            );
        }

        let completeness_statement = if completeness_statements.is_empty() {
            quote! { true }
        } else {
            quote! {
                #(#completeness_statements)&&*
            }
        };

        let maybe_generics = if table_generics.is_empty() {
            None
        } else {
            Some(quote! { < #(#table_generics),* > })
        };
        where_constraints.sort_unstable_by_key(|ts| ts.to_string());
        where_constraints.dedup_by_key(|ts| ts.to_string());
        let maybe_where_constraints = if where_constraints.is_empty() {
            None
        } else {
            Some(quote! { where #(#where_constraints),* })
        };

        let result = quote! {
            impl #maybe_generics common_traits::builder::IsCompleteBuilder for #builder_ident #maybe_generics #maybe_where_constraints
            {
                fn is_complete(&self) -> bool {
                    #completeness_statement
                }
            }
        };

        Ok(result)
    }
}
