//! Diesel code generation

use std::path::Path;

use diesel::PgConnection;
use proc_macro2::TokenStream;
use syn::Ident;

use super::Codegen;
use crate::Table;

mod allow_tables_to_appear_in_same_query;
mod joinable;
mod table;
mod types;

use time_requirements::prelude::{Task, TimeTracker};

impl Codegen<'_> {
    /// Code relative to generating all of the diesel code.
    ///
    /// # Arguments
    ///
    /// * `root` - The root path for the generated code.
    /// * `tables` - The list of tables for which to generate the diesel code.
    /// * `conn` - A mutable reference to a `PgConnection`.
    pub(crate) fn generate_diesel_code(
        &self,
        root: &Path,
        tables: &[Table],
        conn: &mut PgConnection,
    ) -> Result<TimeTracker, crate::errors::WebCodeGenError> {
        let submodule_file = root.with_extension("rs");
        let mut time_tracker = TimeTracker::new("Generating Diesel code");

        std::fs::create_dir_all(root)?;

        let mut submodule_file_content = TokenStream::new();

        if self.enable_tables_schema {
            let task = Task::new("Generating tables schema");
            self.generate_table_macro(
                root.join(crate::codegen::CODEGEN_TABLES_PATH).as_path(),
                tables,
                conn,
            )?;
            time_tracker.add_completed_task(task);

            let tables_ident =
                Ident::new(crate::codegen::CODEGEN_TABLES_PATH, proc_macro2::Span::call_site());

            submodule_file_content.extend(quote::quote! {
                pub mod #tables_ident;
            });
        }
        if self.enable_sql_types {
            let task = Task::new("Generating types schema");
            self.generate_types_macro(
                root.join(crate::codegen::CODEGEN_TYPES_PATH).as_path(),
                tables,
                conn,
            )?;
            time_tracker.add_completed_task(task);

            let types_ident =
                Ident::new(crate::codegen::CODEGEN_TYPES_PATH, proc_macro2::Span::call_site());

            submodule_file_content.extend(quote::quote! {
                pub mod #types_ident;
            });
        }
        if self.enable_joinables {
            let task = Task::new("Generating joinables schema");
            self.generate_joinable_macro(
                root.join(crate::codegen::CODEGEN_JOINABLE_PATH).as_path(),
                tables,
                conn,
            )?;
            time_tracker.add_completed_task(task);

            let joinable_ident =
                Ident::new(crate::codegen::CODEGEN_JOINABLE_PATH, proc_macro2::Span::call_site());

            submodule_file_content.extend(quote::quote! {
                mod #joinable_ident;
            });
        }
        if self.enable_allow_tables_to_appear_in_same_query {
            let task = Task::new("Generating allow tables to appear in same query schema");
            self.generate_allow_tables_to_appear_in_same_query(
                root.join("allow_tables_to_appear_in_same_query").as_path(),
                tables,
                conn,
            )?;
            time_tracker.add_completed_task(task);

            submodule_file_content.extend(quote::quote! {
                mod allow_tables_to_appear_in_same_query;
            });
        }

        std::fs::write(&submodule_file, submodule_file_content.to_string())?;

        Ok(time_tracker)
    }
}
