//! Submodule providing the code to generate the implementation of the
//! [`Insertable`](web_common_traits::database::Insertable) trait for all
//! required tables.

use std::path::Path;

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
    ///
    /// # Errors
    ///
    /// * If the database connection fails.
    /// * If the file system fails.
    pub(super) fn generate_insertable_impls(
        &self,
        root: &Path,
        tables: &[Table],
    ) -> Result<(), crate::errors::WebCodeGenError> {
        std::fs::create_dir_all(root)?;

        let mut insertable_main_module = TokenStream::new();

        for table in tables {
            // We create a file for each table
            let table_file = root.join(format!("{}.rs", table.snake_case_name()?));
            let table_path = table.import_struct_path()?;
            let table_ident = table.snake_case_ident()?;
            let insertable_builder = table.insertable_builder_ty()?;
            let insertable_variant = table.insertable_variant_ty()?;

            std::fs::write(
                &table_file,
                self.beautify_code(&quote::quote! {
                    impl web_common_traits::database::Insertable for #table_path
                    {
                        type InsertableBuilder = #insertable_builder;
                        type InsertableVariant = #insertable_variant;
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
