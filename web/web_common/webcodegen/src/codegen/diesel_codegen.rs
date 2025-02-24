//! Diesel code generation

use std::path::Path;

use diesel::PgConnection;
use proc_macro2::TokenStream;

use crate::Table;

use super::Codegen;

mod allow_tables_to_appear_in_same_query;
mod joinable;
mod table;
mod types;

impl<'a> Codegen<'a> {
    /// Code relative to generating all of the diesel code.
    ///
    /// # Arguments
    ///
    /// * `root` - The root path for the generated code.
    /// * `tables` - The list of tables for which to generate the diesel code.
    /// * `conn` - A mutable reference to a `PgConnection`.
    ///
    pub(crate) fn generate_diesel_code(
        &self,
        root: &Path,
        tables: &[Table],
        conn: &mut PgConnection,
    ) -> Result<(), crate::errors::WebCodeGenError> {
        let submodule_file = root.with_extension("rs");

        std::fs::create_dir_all(&root)?;

        let mut submodule_file_content = TokenStream::new();

        if self.enable_tables {
            self.generate_table_macro(root.join("table").as_path(), tables, conn)?;
            submodule_file_content.extend(quote::quote! {
                pub mod table;
            });
        }
        if self.enable_sql_types {
            self.generate_types_macro(root.join("types").as_path(), tables, conn)?;
            submodule_file_content.extend(quote::quote! {
                pub mod types;
            });
        }
        if self.enable_joinables {
            self.generate_joinable_macro(root.join("joinable").as_path(), tables, conn)?;
            submodule_file_content.extend(quote::quote! {
                mod joinable;
            });
        }
        if self.enable_allow_tables_to_appear_in_same_query {
            self.generate_allow_tables_to_appear_in_same_query(
                root.join("allow_tables_to_appear_in_same_query").as_path(),
                tables,
                conn,
            )?;
            submodule_file_content.extend(quote::quote! {
                mod allow_tables_to_appear_in_same_query;
            });
        }

        std::fs::write(&submodule_file, submodule_file_content.to_string())?;

        Ok(())
    }
}
