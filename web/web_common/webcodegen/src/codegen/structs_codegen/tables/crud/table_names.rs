//! Submodule providing the code generation for the `TableName` enum.

use std::path::Path;

use syn::Ident;

use crate::{
    Codegen, Table,
    codegen::{CODEGEN_DIRECTORY, CODEGEN_TABLES_PATH},
    traits::TableLike,
};

pub(crate) fn table_names_enum_path() -> syn::Type {
    let codegen_ident = Ident::new(CODEGEN_DIRECTORY, proc_macro2::Span::call_site());
    let tables_module_ident = Ident::new(CODEGEN_TABLES_PATH, proc_macro2::Span::call_site());
    syn::parse_quote! {
        crate::#codegen_ident::#tables_module_ident::table_names::TableName
    }
}

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
        let table_snake_case =
            tables.iter().map(Table::snake_case_name).collect::<Result<Vec<_>, _>>()?;
        let tables_enum_file = root.join("table_names.rs");
        std::fs::write(
            &tables_enum_file,
            self.beautify_code(&quote::quote! {
                #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
                #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
                pub enum TableName {
                    #(#table_idents),*
                }

                impl web_common_traits::database::TableEnum for TableName {}

                impl core::str::FromStr for TableName {
                    type Err = String;
                    fn from_str(s: &str) -> Result<Self, Self::Err> {
                        match s {
                            #(
                                #table_snake_case => Ok(TableName::#table_idents)
                            ),*,
                            _ => Err(format!("Unknown table name: {}", s)),
                        }
                    }
                }

                impl core::fmt::Display for TableName {
                    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                        match self {
                            #(
                                TableName::#table_idents => write!(f, #table_snake_case),
                            )*
                        }
                    }
                }
            }),
        )?;

        Ok(())
    }
}
