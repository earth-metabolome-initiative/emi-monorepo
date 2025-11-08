//! Submodule defining the `SchemaMacro` struct to represent an SQL schema which
//! can be printed out in the context of a `quote` macro.

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use sql_traits::traits::ForeignKeyLike;
use synql_core::{
    prelude::{Builder, ColumnLike},
    structs::{Documentation, InternalCrate, InternalModule, InternalToken, Workspace},
    traits::{ColumnSynLike, TableSynLike},
};

use crate::traits::{TABLE_SCHEMA_MODULE_NAME, TableSchema};

/// Struct representing a diesel schema.
pub struct SchemaMacro<'data, 'table, T: TableSynLike> {
    /// The table represented by the schema.
    table: &'table T,
    /// The workspace where the table is defined.
    workspace: &'table Workspace<'data>,
    /// The database connection to use to query the table schema.
    database: &'table T::DB,
}

impl<'table, 'data, T: TableSynLike> From<SchemaMacro<'data, 'table, T>> for InternalModule {
    fn from(value: SchemaMacro<'data, 'table, T>) -> Self {
        let mut macros =
            vec![value.workspace.external_macro("table").expect("Failed to find the macro")];

        if value.table.has_non_self_referential_foreign_keys(value.database) {
            let allow_tables_to_appear_in_same_query = value
                .workspace
                .external_macro("allow_tables_to_appear_in_same_query")
                .expect("Failed to find the macro");

            macros.push(allow_tables_to_appear_in_same_query);
        }

        if value.table.has_non_self_referential_singleton_foreign_keys(value.database) {
            let joinable =
                value.workspace.external_macro("joinable").expect("Failed to find the macro");

            macros.push(joinable);
        }

        InternalModule::new()
            .name(TABLE_SCHEMA_MODULE_NAME)
            .expect("Invalid name")
            .public()
            .documentation(Documentation::new().documentation(format!("Diesel schema for the `{}` table.", value.table.table_name())).unwrap().build().unwrap())
            .internal_token(
                InternalToken::new()
                    .public()
                    .external_macros(macros)
                    .internal_modules(
                        value.table.non_self_referenced_tables(value.database).into_iter().map(
                            |referenced_table| {
                                referenced_table
                                    .schema_module(value.workspace)
                                    .expect(&format!(
                                        "Failed to get the schema module for the referenced table `{}` while building the schema macro for table `{}`.",
                                        referenced_table.table_name(),
                                        value.table.table_name()
                                    ))
                            },
                        ),
                    )
                    .stream(value.into_token_stream())
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap()
    }
}

impl<'table, 'data, T: TableSynLike> From<SchemaMacro<'data, 'table, T>> for InternalCrate {
    fn from(value: SchemaMacro<'data, 'table, T>) -> Self {
        InternalCrate::new()
            .name(value.table.table_schema_crate_name())
            .expect("Invalid crate name")
            .documentation(
                Documentation::new()
                    .documentation(format!(
                        "Diesel schema crate for the `{}` table.",
                        value.table.table_name()
                    ))
                    .unwrap()
                    .build()
                    .unwrap(),
            )
            .module(value.into())
            .expect("Invalid module")
            .build()
            .unwrap()
    }
}

impl<'data, 'table, T: TableSynLike> SchemaMacro<'data, 'table, T> {
    pub(crate) fn new(
        table: &'table T,
        workspace: &'table Workspace<'data>,
        database: &'table T::DB,
    ) -> Self {
        Self { table, workspace, database }
    }
}

impl<'data, 'table, T> ToTokens for SchemaMacro<'data, 'table, T>
where
    T: TableSynLike,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut columns = Vec::new();
        for column in self.table.columns(self.database) {
            let column_attribute = column.column_snake_ident();
            let column_type =
                column.diesel_type(self.workspace, self.database).unwrap_or_else(|| {
                    panic!(
                        "Could not find diesel type `{}` for column `{}.{}`",
                        column.normalized_data_type(self.database),
                        self.table.table_name(),
                        column.column_name(),
                    )
                });

            let sql_name = if column.has_snake_case_column_name() {
                None
            } else {
                let original_column_name = column.column_name();
                Some(quote! {#[sql_name = #original_column_name]})
            };

            let documentation = if let Some(doc) = column.column_doc(self.database) {
                Some(quote! {
                    #[doc = #doc]
                })
            } else {
                None
            };

            columns.push(quote! {
                #documentation
                #sql_name
                #column_attribute -> #column_type
            });
        }
        let primary_key = if self.table.has_primary_key(self.database) {
            let primary_keys = self.table.primary_key_idents(self.database);
            Some(quote! {
                (#(#primary_keys),*)
            })
        } else {
            None
        };

        let documentation = if let Some(doc) = self.table.table_doc(self.database) {
            Some(quote! {
                #[doc = #doc]
            })
        } else {
            None
        };

        let sql_name = if self.table.has_snake_case_table_name() {
            None
        } else {
            let original_table_name = self.table.table_name();
            Some(quote! {#[sql_name = #original_table_name]})
        };

        let maybe_schema = if let Some(schema_name) = self.table.table_schema() {
            Some(quote! {#schema_name.})
        } else {
            None
        };
        let table_name_ident = self.table.table_snake_ident();

        tokens.extend(quote! {
            diesel::table! {
                #documentation
                #sql_name
                #maybe_schema #table_name_ident #primary_key {
                    #(#columns),*
                }
            }
        });

        for referenced_table in self.table.non_self_referenced_tables(self.database) {
            let crate_ref = self
                .workspace
                .internal_crate(&referenced_table.table_schema_crate_name())
                .expect(&format!(
                    "Failed to find the referenced table struct: {}",
                    referenced_table.table_name()
                ));
            let crate_ident = crate_ref.ident();
            let referenced_table_name_ident = referenced_table.table_snake_ident();
            tokens.extend(quote! {
                use #crate_ident::schema::#referenced_table_name_ident;
                diesel::allow_tables_to_appear_in_same_query!(#table_name_ident, #referenced_table_name_ident);
            });
        }

        for foreign_key in self.table.non_self_referential_singleton_foreign_keys(self.database) {
            let referenced_columns =
                foreign_key.referenced_columns(self.database).collect::<Vec<_>>();
            let [referenced_column] = referenced_columns.as_slice() else {
                continue;
            };

            let referenced_table = foreign_key.referenced_table(self.database);
            let referenced_table_name_ident = referenced_table.table_snake_ident();
            let referenced_column_ident = referenced_column.column_snake_ident();

            tokens.extend(quote! {
                diesel::joinable!(#table_name_ident -> #referenced_table_name_ident (#referenced_column_ident));
            });
        }
    }
}
