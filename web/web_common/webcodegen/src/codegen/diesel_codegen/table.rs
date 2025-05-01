//! Submodule implementing code relative to diesel's [`table`](https://docs.rs/diesel/latest/diesel/macro.table.html) macro.

use std::path::Path;

use diesel_async::AsyncPgConnection;
use proc_macro2::TokenStream;

use super::Codegen;
use crate::Table;

impl Codegen<'_> {
    /// Generate implementations of the `table` diesel macro.
    ///
    /// # Arguments
    ///
    /// * `root` - The root path for the generated code.
    /// * `tables` - The list of tables for which to generate the diesel code.
    /// * `conn` - A mutable reference to a `PgConnection`.
    pub(crate) async fn generate_table_macro(
        &self,
        root: &Path,
        tables: &[Table],
        conn: &mut AsyncPgConnection,
    ) -> Result<(), crate::errors::WebCodeGenError> {
        std::fs::create_dir_all(root)?;
        // We generate each table in a separate document under the provided root, and we
        // collect all of the imported modules in a public one.
        let mut table_main_module = TokenStream::new();
        for table in tables {
            let table_identifier = table.snake_case_ident()?;
            let table_file = root.join(format!("{}.rs", table.snake_case_name()?));
            let table_content = table.to_schema(conn).await?;
            std::fs::write(&table_file, self.beautify_code(&table_content)?)?;

            table_main_module.extend(quote::quote! {
                pub mod #table_identifier;
            });
        }

        let table_module = root.with_extension("rs");
        std::fs::write(&table_module, self.beautify_code(&table_main_module)?)?;

        Ok(())
    }
}
