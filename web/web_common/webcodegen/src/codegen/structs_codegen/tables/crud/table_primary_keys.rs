//! Submodule providing the code generation for the `TablePrimaryKey` enum.

use std::path::Path;

use diesel::PgConnection;
use proc_macro2::TokenStream;
use syn::Ident;

use crate::{
    Codegen, Table,
    codegen::{CODEGEN_DIRECTORY, CODEGEN_TABLES_PATH},
};

impl Codegen<'_> {
    pub(crate) fn table_primary_keys_enum_path() -> TokenStream {
        let codegen_ident = Ident::new(CODEGEN_DIRECTORY, proc_macro2::Span::call_site());
        let tables_module_ident = Ident::new(CODEGEN_TABLES_PATH, proc_macro2::Span::call_site());
        quote::quote! {
            crate::#codegen_ident::#tables_module_ident::table_primary_keys::TablePrimaryKey
        }
    }

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
        let table_name_enum_path = Self::table_names_enum_path();

        let mut table_idents = Vec::new();

        for table in tables {
            let struct_ident = table.struct_ident()?;
            let primary_key = table.primary_key_type(conn)?;

            table_idents.push(quote::quote! {
                #struct_ident(#primary_key)
            });
        }

        let table_name_impls: Vec<TokenStream> = tables
            .iter()
            .map(|table| {
                let struct_ident = table.struct_ident()?;

                Ok(quote::quote! {
                    TablePrimaryKey::#struct_ident(_) => {
                        #table_name_enum_path::#struct_ident
                    }
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

                impl web_common_traits::prelude::Tabular for TablePrimaryKey {
                    type TableName = #table_name_enum_path;
                    fn table_name(&self) -> Self::TableName {
                        match self {
                            #(
                                #table_name_impls
                            ),*
                        }
                    }
                }
            })?,
        )?;
        Ok(())
    }
}
