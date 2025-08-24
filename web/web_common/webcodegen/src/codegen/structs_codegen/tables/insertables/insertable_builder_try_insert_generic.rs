//! Submodule providing the generation of the `try_insert` method for the
//! insertable builder struct.

use std::collections::HashSet;

use diesel::PgConnection;
use proc_macro2::TokenStream;
use quote::quote;
use crate::traits::TableLike;

use crate::{Codegen, Table, errors::WebCodeGenError};

impl Codegen<'_> {
    /// Returns the implementation of the `TryInsertGeneric` trait for the
    /// insertable builder struct.
    ///
    /// # Arguments
    ///
    /// * `table` - The table for which the implementation is generated.
    /// * `conn` - The database connection to use for querying the database.
    ///
    /// # Errors
    ///
    /// If the provided connection to the database fails.
    pub(super) fn generate_insertable_builder_try_insert_generic_implementation(
        &self,
        table: &Table,
        conn: &mut PgConnection,
    ) -> Result<TokenStream, WebCodeGenError> {
        let Some(user_table) = self.users_table else {
            return Err(crate::errors::CodeGenerationError::UserTableNotProvided.into());
        };
        let user_id_type = user_table.primary_key_type(conn)?;
        let builder_ident = table.insertable_builder_ident()?;
        let row_path = table.import_struct_path()?;
        let attributes_enum = table.insertable_enum_ident()?;
        let table_extension_network = self.table_extension_network().unwrap();
        let extension_tables = table_extension_network.extension_tables(table);
        let extension_foreign_keys = table_extension_network.extension_foreign_keys(table, conn)?;
        let primary_key_type = table.primary_key_type(conn)?;
        let mut where_constraints = vec![quote! {
            Self: web_common_traits::database::InsertableVariant<
                C,
                UserId=#user_id_type,
                Row=#row_path,
                Error=web_common_traits::database::InsertError<#attributes_enum>
            >
        }];
        let mut try_insert_generic_where_constraint: HashSet<Table> = HashSet::new();
        let mut completeness_statements = Vec::new();
        let mut table_generics = Vec::new();

        for extension_table in &extension_tables {
            let generic_ident = extension_table.struct_ident()?;
            table_generics.push(quote! {#generic_ident});
            where_constraints.push(quote! {
                #generic_ident: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = #primary_key_type>
            });
            try_insert_generic_where_constraint.insert((*extension_table).clone());
        }

        // For each extension foreign key, we need to call recursively the `is_complete`
        // method
        for foreign_key in &extension_foreign_keys {
            let foreign_key_ident = foreign_key.constraint_ident(conn)?;
            completeness_statements.push(quote! {
                self.#foreign_key_ident.is_complete()
            });
        }

        // For each of the other columns, if they are not optional, we need to check if
        // they are complete
        for column in table.insertable_columns(conn, false)? {
            // If the column is nullable, we do not need to check its completeness
            if column.is_nullable() {
                continue;
            }

            // Otherwise, we need to check if the column is complete, i.e. whether
            // it has been set in the builder
            let column_ident = column.snake_case_ident()?;

            completeness_statements.push(
                if let Some(partial_builder_constraint) = column.requires_partial_builder(conn)? {
                    let foreign_table = partial_builder_constraint
                        .foreign_table(conn)?
                        .expect("Foreign table should be present");
                    if !try_insert_generic_where_constraint.contains(&foreign_table) {
                        let foreign_builder_type = foreign_table.insertable_builder_ty()?;
                        where_constraints.push(quote! {
                            #foreign_builder_type: web_common_traits::database::TryInsertGeneric<C>
                        });
                        try_insert_generic_where_constraint.insert(foreign_table);
                    }
                    quote! {
                        self.#column_ident.is_complete()
                    }
                } else {
                    quote! {
                        self.#column_ident.is_some()
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

        let left_generics = {
            let mut left_generics = table_generics.clone();
            left_generics.push(quote! { C });
            left_generics
        };

        let maybe_generics = if table_generics.is_empty() {
            None
        } else {
            Some(quote! { < #(#table_generics),* > })
        };

        Ok(quote! {
            impl < #(#left_generics),* > web_common_traits::database::TryInsertGeneric<C> for #builder_ident #maybe_generics
                where
                    #(#where_constraints),*
            {
                type Attributes = #attributes_enum;

                fn is_complete(&self) -> bool {
                    #completeness_statement
                }

                fn mint_primary_key(
                    self,
                    user_id: #user_id_type,
                    conn: &mut C,
                ) -> Result<Self::PrimaryKey, web_common_traits::database::InsertError<Self::Attributes>>
                {
                    use diesel::Identifiable;
                    use web_common_traits::database::InsertableVariant;
                    let insertable: #row_path = self.insert(user_id, conn)?;
                    Ok(insertable.id())
                }
            }
        })
    }
}
