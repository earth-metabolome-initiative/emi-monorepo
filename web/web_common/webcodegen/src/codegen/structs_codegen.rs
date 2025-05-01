//! Diesel code generation

use std::path::Path;

use diesel_async::AsyncPgConnection;
use proc_macro2::TokenStream;
use syn::Ident;

use super::Codegen;
use crate::Table;

mod tables;
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
    pub(crate) async fn generate_structs_code(
        &self,
        root: &Path,
        tables: &[Table],
        conn: &mut AsyncPgConnection,
    ) -> Result<TimeTracker, crate::errors::WebCodeGenError> {
        let submodule_file = root.with_extension("rs");
        let mut time_tracker = TimeTracker::new("Generate Structs");

        std::fs::create_dir_all(root)?;

        let mut submodule_file_content = TokenStream::new();

        if self.enable_type_structs {
            let task = Task::new("Generate Types Structs");
            self.generate_types_structs(
                root.join(crate::codegen::CODEGEN_TYPES_PATH).as_path(),
                tables,
                conn,
            ).await?;

            let types_ident =
                Ident::new(crate::codegen::CODEGEN_TYPES_PATH, proc_macro2::Span::call_site());

            submodule_file_content.extend(quote::quote! {
                pub mod #types_ident;
            });
            time_tracker.add_completed_task(task);
        }

        if self.enable_table_structs {
            let task = Task::new("Generate Table Structs");
            self.generate_table_structs(
                root.join(crate::codegen::CODEGEN_TABLES_PATH).as_path(),
                tables,
                conn,
            ).await?;

            let tables_ident =
                Ident::new(crate::codegen::CODEGEN_TABLES_PATH, proc_macro2::Span::call_site());

            let table_structs =
                tables.iter().map(Table::struct_ident).collect::<Result<Vec<_>, _>>()?;

            submodule_file_content.extend(quote::quote! {
                pub mod #tables_ident;
                #[allow(unused_imports)]
                pub use #tables_ident::{#(#table_structs),*};
            });
            time_tracker.add_completed_task(task);
        }

        std::fs::write(&submodule_file, self.beautify_code(&submodule_file_content)?)?;

        Ok(time_tracker)
    }
}
