//! Diesel code generation

use std::path::Path;

use diesel::PgConnection;
use proc_macro2::TokenStream;
use syn::Ident;

use super::Codegen;
use crate::Table;

mod tables;
mod types;

impl Codegen<'_> {
    /// Code relative to generating all of the diesel code.
    ///
    /// # Arguments
    ///
    /// * `root` - The root path for the generated code.
    /// * `tables` - The list of tables for which to generate the diesel code.
    /// * `conn` - A mutable reference to a `PgConnection`.
    pub(crate) fn generate_structs_code(
        &self,
        root: &Path,
        tables: &[Table],
        conn: &mut PgConnection,
    ) -> Result<(), crate::errors::WebCodeGenError> {
        let submodule_file = root.with_extension("rs");

        std::fs::create_dir_all(root)?;

        let mut submodule_file_content = TokenStream::new();

        if self.enable_type_structs {
            self.generate_types_structs(
                root.join(crate::codegen::CODEGEN_TYPES_PATH).as_path(),
                tables,
                conn,
            )?;

            let types_ident =
                Ident::new(crate::codegen::CODEGEN_TYPES_PATH, proc_macro2::Span::call_site());

            submodule_file_content.extend(quote::quote! {
                pub mod #types_ident;
            });
        }

        if self.enable_table_structs {
            self.generate_table_structs(
                root.join(crate::codegen::CODEGEN_TABLES_PATH).as_path(),
                tables,
                conn,
            )?;

            let tables_ident =
                Ident::new(crate::codegen::CODEGEN_TABLES_PATH, proc_macro2::Span::call_site());

            let table_structs =
                tables.iter().map(Table::struct_ident).collect::<Result<Vec<_>, _>>()?;

            submodule_file_content.extend(quote::quote! {
                pub mod #tables_ident;
                pub use #tables_ident::{#(#table_structs),*};
            });
        }

        std::fs::write(&submodule_file, self.beautify_code(&submodule_file_content)?)?;

        Ok(())
    }
}
