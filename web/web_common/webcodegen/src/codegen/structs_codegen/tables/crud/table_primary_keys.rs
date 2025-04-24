//! Submodule providing the code generation for the `TablePrimaryKey` enum.

use std::path::Path;

use diesel::PgConnection;

use crate::{Codegen, Table};

impl Codegen<'_> {
    /// Generate implementations of the `TablePrimaryKey` enum for the provided
    /// tables.
    ///
    /// # Arguments
    ///
    /// * `root` - The root path for the generated code.
    /// * `tables` - The tables for which to generate the diesel code.
    ///
    /// # Errors
    ///
    /// * If the database connection fails.
    /// * If the file system fails.
    pub(crate) fn generate_table_primary_keys_enumeration(
        &self,
        root: &Path,
        tables: &[Table],
        conn: &mut PgConnection,
    ) -> Result<(), crate::errors::WebCodeGenError> {
        std::fs::create_dir_all(root)?;
        let table_idents = tables
            .iter()
            .map(|table| {
                let struct_ident = table.struct_ident()?;
                let primary_key = table.primary_key_type(conn)?;

                Ok(quote::quote! {
                    #struct_ident(#primary_key)
                })
            })
            .collect::<Result<Vec<_>, crate::errors::WebCodeGenError>>()?;
        let tables_enum_file = root.join("table_primary_keys.rs");
        std::fs::write(
            &tables_enum_file,
            self.beautify_code(&quote::quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
                #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
                pub enum TablePrimaryKey {
                    #(#table_idents),*
                }
            })?,
        )?;
        Ok(())
    }
}
