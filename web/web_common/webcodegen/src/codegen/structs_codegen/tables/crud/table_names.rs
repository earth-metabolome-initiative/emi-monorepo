//! Submodule providing the code generation for the `TableName` enum.

use std::path::Path;

use crate::{Codegen, Table};

impl Codegen<'_> {
    /// Generate implementations of the `TableName` enum for the provided
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
    pub(crate) fn generate_table_names_enumeration(
        &self,
        root: &Path,
        tables: &[Table],
    ) -> Result<(), crate::errors::WebCodeGenError> {
        std::fs::create_dir_all(root)?;

        let table_idents = tables.iter().map(Table::struct_ident).collect::<Result<Vec<_>, _>>()?;
        let tables_enum_file = root.join("table_names.rs");
        std::fs::write(
            &tables_enum_file,
            self.beautify_code(&quote::quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
                #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
                pub enum TableName {
                    #(#table_idents),*
                }
            })?,
        )?;

        Ok(())
    }
}
