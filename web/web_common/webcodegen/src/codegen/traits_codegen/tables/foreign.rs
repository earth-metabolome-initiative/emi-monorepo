//! Submodule providing the code to generate the implementation of the Foreign
//! traits for all requiring methods.

use std::path::Path;

use diesel::PgConnection;
use proc_macro2::TokenStream;
use syn::Ident;

use crate::{Codegen, Table, codegen::CODEGEN_FOREIGN_KEYS_PATH};

mod foreign_keys;

impl Codegen<'_> {
    /// Generates the Foreign traits implementation for the tables
    ///
    /// # Arguments
    ///
    /// * `root` - The root path for the generated code.
    /// * `tables` - The list of tables for which to generate the diesel code.
    /// * `conn` - A mutable reference to a `PgConnection`.
    pub(super) fn generate_foreign_impls(
        &self,
        root: &Path,
        tables: &[Table],
        conn: &mut PgConnection,
    ) -> Result<(), crate::errors::WebCodeGenError> {
        std::fs::create_dir_all(root)?;
        // We generate each table in a separate document under the provided root, and we
        // collect all of the imported modules in a public one.
        let mut table_foreign_main_module = TokenStream::new();
        if self.should_generate_crud() {
            // We dispatch a call to generate the `ForeignKeys` trait
            self.generate_foreign_keys_impls(&root.join(CODEGEN_FOREIGN_KEYS_PATH), tables, conn)?;
            let foreign_keys_module =
                Ident::new(CODEGEN_FOREIGN_KEYS_PATH, proc_macro2::Span::call_site());
            table_foreign_main_module.extend(quote::quote! {
                mod #foreign_keys_module;
            });
        }

        let table_module = root.with_extension("rs");
        std::fs::write(&table_module, self.beautify_code(&table_foreign_main_module)?)?;

        Ok(())
    }
}
