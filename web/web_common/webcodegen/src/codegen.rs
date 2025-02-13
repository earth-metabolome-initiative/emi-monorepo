//! Submodule providing the 'Codegen' struct for code generation.

use std::{collections::HashSet, path::Path};

use diesel::PgConnection;
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{File, Ident};

use crate::{errors::WebCodeGenError, Column, PgType, Table};
use prettyplease::unparse;

#[derive(Debug)]
/// Error type for code generation.
pub enum CodeGenerationError {
    /// The output path was not provided.
    PathNotProvided,
}

#[derive(Debug, Default)]
/// Struct for code generation.
pub struct Codegen<'a> {
    /// List of tables to ignore when generating code.
    tables_deny_list: Vec<&'a Table>,
    /// The output path for the generated code.
    output_path: Option<&'a Path>,
}

impl<'a> Codegen<'a> {
    #[must_use]
    /// Adds a new table to the deny list.
    pub fn add_table_to_deny_list(mut self, table: &'a Table) -> Self {
        self.tables_deny_list.push(table);
        self
    }

    #[must_use]
    /// Sets the output path for the generated code.
    pub fn set_output_path(mut self, output_path: &'a Path) -> Self {
        self.output_path = Some(output_path);
        self
    }

    /// Writes all the tables syn version to a file.
    /// 
    /// # Arguments
    /// 
    /// * `conn` - A mutable reference to a `PgConnection`.
    /// * `table_catalog` - The name of the table catalog.
    /// * `table_schema` - The name of the table schema.
    /// 
    /// # Errors
    /// 
    /// * Returns an error if the output path is not provided.
    /// * Returns an error if the tables cannot be loaded.
    /// * Returns an error if the tables cannot be converted to schema.
    /// * Returns an error if the tables cannot be converted to syn.
    /// * Returns an error if the user defined types cannot be loaded.
    /// * Returns an error if the user defined types cannot be converted to syn.
    /// * Returns an error if the generated code cannot be written to the output file.
    /// 
    pub fn generate(
        &self,
        conn: &mut PgConnection,
        table_catalog: &str,
        table_schema: Option<&str>,
    ) -> Result<(), WebCodeGenError> {
        if self.output_path.is_none() {
            return Err(CodeGenerationError::PathNotProvided.into());
        }

        let tables = Table::load_all(conn, table_catalog, table_schema)?
            .into_iter()
            .filter(|table| !(table.is_temporary() || table.is_view()))
            .filter(|table| !self.tables_deny_list.contains(&table))
            .collect::<Vec<Table>>();

        let schema = tables
            .iter()
            .map(|table| table.to_schema(conn))
            .collect::<Result<Vec<TokenStream>, WebCodeGenError>>()?;

        let table_structs = tables
            .iter()
            .map(|table| table.to_syn(conn))
            .collect::<Result<Vec<TokenStream>, WebCodeGenError>>()?;

        let all_table_idents = tables
            .iter()
            .map(Table::snake_case_ident)
            .collect::<Result<Vec<Ident>, WebCodeGenError>>()?;

        let table_idents_below_64_columns = tables
            .iter()
            .filter(|table| {
                table
                    .columns(conn)
                    .map(|columns| columns.len() <= 64)
                    .unwrap_or(false)
            })
            .map(Table::snake_case_ident)
            .collect::<Result<Vec<Ident>, WebCodeGenError>>()?;

        let table_idents_below_32_columns = tables
            .iter()
            .filter(|table| {
                table
                    .columns(conn)
                    .map(|columns| columns.len() <= 32)
                    .unwrap_or(false)
            })
            .map(Table::snake_case_ident)
            .collect::<Result<Vec<Ident>, WebCodeGenError>>()?;

        let table_idents_below_16_columns = tables
            .iter()
            .filter(|table| {
                table
                    .columns(conn)
                    .map(|columns| columns.len() <= 16)
                    .unwrap_or(false)
            })
            .map(Table::snake_case_ident)
            .collect::<Result<Vec<Ident>, WebCodeGenError>>()?;

        let user_defined_types = tables
            .iter()
            .map(|table| {
                let custom_types = table
                    .columns(conn)?
                    .into_iter()
                    .filter(Column::has_custom_type)
                    .map(|column| PgType::from_name(column.data_type_str(conn)?, conn))
                    .filter_ok(|pg_type| pg_type.is_enum() || pg_type.is_composite())
                    .collect::<Result<HashSet<PgType>, WebCodeGenError>>()?;
                let mut additional_custom_types = custom_types.clone();
                for custom_type in custom_types {
                    additional_custom_types.extend(custom_type.internal_custom_types(conn)?);
                }
                Ok(additional_custom_types)
            })
            .collect::<Result<Vec<HashSet<PgType>>, WebCodeGenError>>()?
            .into_iter()
            .flatten()
            .collect::<HashSet<PgType>>();

        let user_defined_types_syn = user_defined_types
            .into_iter()
            .map(|pg_type| pg_type.to_syn(conn))
            .collect::<Result<Vec<TokenStream>, WebCodeGenError>>()?;

        // We define for each group of tables by column size the corresponding diesel macro
        // for allow_tables_to_appear_in_same_query, with negative flags to avoid multiple such
        // macros active at the same time.
        let above_64_columns = if all_table_idents.len() == table_idents_below_64_columns.len() {
            TokenStream::new()
        } else {
            quote! {
                #[cfg(feature = "128-column-tables")]
                diesel::allow_tables_to_appear_in_same_query!( #( #all_table_idents ),* );
            }
        };

        let above_32_columns = if table_idents_below_64_columns.len()
            == table_idents_below_32_columns.len()
        {
            TokenStream::new()
        } else {
            quote! {
                #[cfg(all(feature = "64-column-tables", not(feature = "128-column-tables")))]
                diesel::allow_tables_to_appear_in_same_query!( #( #table_idents_below_64_columns ),* );
            }
        };

        let above_16_columns = if table_idents_below_32_columns.len()
            == table_idents_below_16_columns.len()
        {
            TokenStream::new()
        } else {
            quote! {
                #[cfg(all(feature = "32-column-tables", not(feature = "64-column-tables")))]
                diesel::allow_tables_to_appear_in_same_query!( #( #table_idents_below_32_columns ),* );
            }
        };

        let below_16_columns = if table_idents_below_16_columns.is_empty() {
            TokenStream::new()
        } else if table_idents_below_16_columns.len() == table_idents_below_32_columns.len() {
            quote! {
                #[cfg(feature = "diesel")]
                diesel::allow_tables_to_appear_in_same_query!( #( #table_idents_below_16_columns ),* );
            }
        } else {
            quote! {
                #[cfg(all(feature = "diesel", not(feature = "32-column-tables")))]
                diesel::allow_tables_to_appear_in_same_query!( #( #table_idents_below_16_columns ),* );
            }
        };

        // Create a new TokenStream
        let output = quote! {
            #[cfg(feature = "diesel")]
            use diesel::{ExpressionMethods, QueryDsl};
            #[cfg(feature = "diesel")]
            use diesel_async::RunQueryDsl;

            #(#user_defined_types_syn)*

            #( #schema )*

            #above_64_columns
            #above_32_columns
            #above_16_columns
            #below_16_columns

            #( #table_structs )*
        };

        // Convert the generated TokenStream to a string
        let code_string = output.to_string();

        // Parse the generated code string into a syn::Item
        let syntax_tree: File = syn::parse_str(&code_string).unwrap();

        // Use prettyplease to format the syntax tree
        let formatted_code = unparse(&syntax_tree);

        // Write the formatted code to the output file
        std::fs::write(self.output_path.unwrap(), formatted_code).unwrap();

        Ok(())
    }
}
