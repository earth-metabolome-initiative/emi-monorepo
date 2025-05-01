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
    pub(crate) async fn generate_types_macro(
        &self,
        root: &Path,
        tables: &[Table],
        conn: &mut AsyncPgConnection,
    ) -> Result<(), crate::errors::WebCodeGenError> {
        std::fs::create_dir_all(root)?;

        let types = Self::required_types(tables, conn).await?;

        // We generate each table in a separate document under the provided root, and we
        // collect all of the imported modules in a public one.
        let mut types_main_module = TokenStream::new();

        for r#type in types {
            let type_name = r#type.snake_case_name()?;
            let type_ident = r#type.snake_case_identifier()?;
            let type_file = root.join(format!("{type_name}.rs"));
            std::fs::write(&type_file, self.beautify_code(&r#type.to_diesel_macro())?)?;

            types_main_module.extend(quote::quote! {
                pub mod #type_ident;
            });
        }

        let table_module = root.with_extension("rs");
        std::fs::write(&table_module, self.beautify_code(&types_main_module)?)?;

        Ok(())
    }
}
