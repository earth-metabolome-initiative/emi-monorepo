//! Submodule implementing the writing of the crate library files.

use std::io::Write;

use quote::quote;
use sql_relations::prelude::{ColumnLike, TableLike};

use crate::{
    structs::{SynQL, Workspace},
    traits::{SynQLDatabaseLike, column::ColumnSynLike, table::TableSynLike},
};

impl<DB: SynQLDatabaseLike> SynQL<'_, DB> {
    pub(super) fn write_crate_lib(
        &self,
        table: &DB::Table,
        workspace: &Workspace,
    ) -> Result<(), crate::Error> {
        // We create the `src` directory if it does not exist
        let crate_path = table.crate_path(workspace);
        let src_path = crate_path.join("src");
        std::fs::create_dir_all(&src_path)?;
        // We create the `lib.rs` file
        let lib_rs_path = src_path.join("lib.rs");
        let mut buffer = std::fs::File::create(lib_rs_path)?;

        // We write the content of the `lib.rs` file
        // #[derive(Debug, Queryable, Selectable, Identifiable, PartialEq, TableModel)]
        // #[table_model(error = ErrorB, ancestors = parent_table)]
        // #[diesel(table_name = child_with_satellite_table)]
        // /// Model for table B.
        // pub struct ChildWithDiscretionary {
        //     #[infallible]
        //     /// Primary key.
        //     #[same_as(satellite_table::parent_id)]
        //     id: i32,
        //     #[infallible]
        //     #[discretionary(satellite_table)]
        //     /// Foreign key to discretionary table.
        //     discretionary_id: i32,
        //     #[infallible]
        //     /// Some other column in the child table.
        //     child_field: String,
        //     /// The remote `field` value from discretionary table that B references
        // via `discretionary_id`.     #[same_as(satellite_table::field)]
        //     #[table_model(default = "Some default value")]
        //     remote_field: Option<String>,
        //     /// The remote `parent_id` value from discretionary table that B
        // references via `discretionary_id`.     #[infallible]
        //     #[same_as(satellite_table::another_field)]
        //     another_remote_column: Option<String>,
        // }

        let core_derives = table.supported_core_derives(self.database, workspace);
        let table_name = table.table_name();
        let camel_case_name = table.table_singular_camel_ident();
        let table_ident = table.table_snake_ident();
        let crate_documentation = format!("Auto-generated crate for the `{table_name}` table.");
        let struct_documentation = format!("Model for table `{table_name}`.");

        let mut ancestor_decorator = None;
        let ancestors = table.ancestral_extended_tables_topological(self.database);
        if !ancestors.is_empty() {
            let ancestor_table_paths: Vec<syn::Path> = ancestors
                .iter()
                .map(|ancestor_table| {
                    let ancestor_crate = ancestor_table.crate_ident(workspace);
                    let ancestor_table_ident = ancestor_table.table_snake_ident();
                    syn::parse_quote! { #ancestor_crate::#ancestor_table_ident }
                })
                .collect();
            ancestor_decorator = Some(quote! {
                #[table_model(ancestors = (#(#ancestor_table_paths),*))]
            });
        }

        // If the crate has check constraints, it means we need to specify
        // the error type in the derive macro.
        let mut error_decorator = None;
        if table.has_check_constraints(self.database) {
            error_decorator = Some(quote! {
                #[table_model(error = validation_errors::ValidationError)]
            });
        }

        // #[diesel(primary_key(user_id, role_id))]
        let mut primary_key_decorator = None;
        let primary_key_columns = table.primary_key_columns(self.database).collect::<Vec<_>>();
        if primary_key_columns.len() > 1 && primary_key_columns[0].column_name() != "id" {
            let primary_key_idents: Vec<syn::Ident> =
                primary_key_columns.iter().map(|col| col.column_snake_ident()).collect();
            primary_key_decorator = Some(quote! {
                #[diesel(primary_key(#(#primary_key_idents),*))]
            });
        }

        let fields = table.generate_struct_fields(workspace, self.database)?;
        let unique_indices = table.unique_indices_macros(self.database);
        let foreign_keys = table.foreign_keys_macros(self.database, workspace);

        let content = quote! {
            #![doc=#crate_documentation]

            #[derive(#(#core_derives),*)]
            #[derive(diesel::Queryable, diesel::Selectable, diesel::Identifiable, diesel_builders::prelude::TableModel)]
            #[doc=#struct_documentation]
            #ancestor_decorator
            #error_decorator
            #primary_key_decorator
            #[diesel(table_name = #table_ident)]
            pub struct #camel_case_name {
                #(#fields),*
            }
            #(#unique_indices)*
            #(#foreign_keys)*
        };

        write!(buffer, "{content}")?;

        Ok(())
    }
}
