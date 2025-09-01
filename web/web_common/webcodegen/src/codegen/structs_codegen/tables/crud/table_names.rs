//! Submodule providing the code generation for the `TableName` enum.

use std::path::Path;

use proc_macro2::TokenStream;
use syn::Ident;

use crate::{
    Codegen, Table,
    codegen::{CODEGEN_DIRECTORY, CODEGEN_TABLES_PATH},
    traits::TableLike,
};

impl Codegen<'_> {
    pub(crate) fn table_names_enum_path() -> TokenStream {
        let codegen_ident = Ident::new(CODEGEN_DIRECTORY, proc_macro2::Span::call_site());
        let tables_module_ident = Ident::new(CODEGEN_TABLES_PATH, proc_macro2::Span::call_site());
        quote::quote! {
            crate::#codegen_ident::#tables_module_ident::table_names::TableName
        }
    }

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

                impl core::fmt::Display for TableName {
                    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        match self {
                            #(
                                TableName::#table_idents => write!(f, stringify!(#table_idents)),
                            )*
                        }
                    }
                }
            })?,
        )?;

        Ok(())
    }
}
