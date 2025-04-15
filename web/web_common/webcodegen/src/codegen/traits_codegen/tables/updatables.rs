//! Submodule providing the code to generate the implementation of the
//! [`Insertable`] and [`Insertable`]-adjacent traits for all requiring methods.

mod updatable;

use std::path::Path;

use diesel::PgConnection;
use quote::quote;
use syn::Ident;

use crate::{Codegen, Table, codegen::CODEGEN_UPDATABLE_PATH};

impl Codegen<'_> {
    /// Generates the [`Insertable`] and [`Insertable`]-adjacent traits
    /// implementation for the tables
    ///
    /// # Arguments
    ///
    /// * `root` - The root path for the generated code.
    /// * `tables` - The list of tables for which to generate the diesel code.
    /// * `conn` - A mutable reference to a `PgConnection`.
    ///
    /// # Errors
    ///
    /// * If the database connection fails.
    /// * If the file system fails.
    pub(super) fn generate_updatables_impls(
        &self,
        root: &Path,
        tables: &[Table],
        conn: &mut PgConnection,
    ) -> Result<(), crate::errors::WebCodeGenError> {
        std::fs::create_dir_all(root)?;

        self.generate_updatable_impls(&root.join(CODEGEN_UPDATABLE_PATH), tables, conn)?;

        let updatable_module = Ident::new(CODEGEN_UPDATABLE_PATH, proc_macro2::Span::call_site());

        let table_module = root.with_extension("rs");
        std::fs::write(
            &table_module,
            self.beautify_code(&quote! {
                mod #updatable_module;
            })?,
        )?;

        Ok(())
    }
}
