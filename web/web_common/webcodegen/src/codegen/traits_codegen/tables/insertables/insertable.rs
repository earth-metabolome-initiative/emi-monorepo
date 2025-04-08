//! Submodule providing the code to generate the implementation of the
//! [`Insertable`](web_common_traits::database::Insertable) trait for all
//! required tables.

use std::path::Path;

use diesel::PgConnection;
use proc_macro2::TokenStream;

use crate::{Codegen, Table};

impl Codegen<'_> {
    /// Generates the [`Insertable`](web_common_traits::database::Insertable)
    /// trait implementation for the tables
    ///
    /// # Arguments
    ///
    /// * `root` - The root path for the generated code.
    /// * `tables` - The list of tables for which to generate the diesel code.
    /// * `conn` - A mutable reference to a `PgConnection`.
    ///
    /// # Errors
    ///
    /// * If the database connection fails.
    /// * If the file system fails.
    pub(super) fn generate_insertable_impls(
        &self,
        root: &Path,
        tables: &[Table],
        conn: &mut PgConnection,
    ) -> Result<(), crate::errors::WebCodeGenError> {
        std::fs::create_dir_all(root)?;

        let mut insertable_main_module = TokenStream::new();
        let syntax_flag = self.syntax.as_feature_flag();

        for table in tables {
            if !self.is_table_insertable(table, conn)? {
                continue;
            }

            // We create a file for each table
            let table_file = root.join(format!("{}.rs", table.snake_case_name()?));
            let table_path = table.import_struct_path()?;
            let table_ident = table.snake_case_ident()?;
            let insertable_variant = table.insertable_variant_ty()?;
            let insertable_builder = table.insertable_builder_ty()?;

            std::fs::write(
                &table_file,
                self.beautify_code(&quote::quote! {
                    #syntax_flag
                    impl web_common_traits::database::Insertable for #table_path {
                        type InsertableVariant = #insertable_variant;
                        type InsertableBuilder = #insertable_builder;
                    }
                })?,
            )?;

            insertable_main_module.extend(quote::quote! {
                mod #table_ident;
            });
        }

        let table_module = root.with_extension("rs");
        std::fs::write(&table_module, self.beautify_code(&insertable_main_module)?)?;

        Ok(())
    }
}
