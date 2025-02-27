//! Web Common Traits code generation.

use std::path::Path;

use diesel::PgConnection;
use proc_macro2::TokenStream;

use super::Codegen;
use crate::Table;

mod tables;
mod types;
use syn::Ident;

impl Codegen<'_> {
    /// Code relative to generating all of the web common traits code.
    ///
    /// # Arguments
    ///
    /// * `root` - The root path for the generated code.
    /// * `tables` - The list of tables for which to generate the web common
    ///   traits code.
    /// * `conn` - A mutable reference to a `PgConnection`.
    pub(crate) fn generate_web_common_traits_implementations(
        &self,
        root: &Path,
        tables: &[Table],
        conn: &mut PgConnection,
    ) -> Result<(), crate::errors::WebCodeGenError> {
        let submodule_file = root.with_extension("rs");

        std::fs::create_dir_all(root)?;

        let mut submodule_file_content = TokenStream::new();

        if self.enable_type_impls {
            self.generate_types_traits(
                root.join(crate::codegen::CODEGEN_TYPES_PATH).as_path(),
                tables,
                conn,
            )?;
            let types_ident =
                Ident::new(crate::codegen::CODEGEN_TYPES_PATH, proc_macro2::Span::call_site());

            submodule_file_content.extend(quote::quote! {
                mod #types_ident;
            });
        }

        if self.should_generate_table_traits() {
            self.generate_table_traits(
                root.join(crate::codegen::CODEGEN_TABLES_PATH).as_path(),
                tables,
                conn,
            )?;

            let tables_ident =
                Ident::new(crate::codegen::CODEGEN_TABLES_PATH, proc_macro2::Span::call_site());

            submodule_file_content.extend(quote::quote! {
                mod #tables_ident;
            });
        }

        std::fs::write(&submodule_file, submodule_file_content.to_string())?;

        Ok(())
    }
}
