//! Submodule providing the code to generate the implementation of the
//! [`Insertable`] and [`Insertable`]-adjacent traits for all requiring methods.

mod insertable;
mod insertable_builder;
mod insertable_variant;

use std::path::Path;

use diesel::PgConnection;
use quote::quote;
use syn::Ident;

use crate::{
    codegen::{
        CODEGEN_INSERTABLE_BUILDER_PATH, CODEGEN_INSERTABLE_PATH, CODEGEN_INSERTABLE_VARIANT_PATH,
    },
    errors::WebCodeGenError,
    Codegen, Table,
};

impl Codegen<'_> {
    /// Returns wether the table should be insertable or not
    ///
    ///  # Arguments
    ///
    ///  * `table` - The passed table
    ///  * `conn` - A mutable reference to a `PgConnection`.
    ///
    ///  # Errors
    ///
    /// * If the database connection fails.
    pub(crate) fn is_table_insertable(
        &self,
        table: &Table,
        conn: &mut PgConnection,
    ) -> Result<bool, WebCodeGenError> {
        Ok(true)
    }
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
    pub(super) fn generate_insertables_impls(
        &self,
        root: &Path,
        tables: &[Table],
        conn: &mut PgConnection,
    ) -> Result<(), crate::errors::WebCodeGenError> {
        std::fs::create_dir_all(root)?;

        self.generate_insertable_impls(&root.join(CODEGEN_INSERTABLE_PATH), tables, conn)?;
        self.generate_insertable_variant_impls(
            &root.join(CODEGEN_INSERTABLE_VARIANT_PATH),
            tables,
            conn,
        )?;
        self.generate_insertable_builder_impls(
            &root.join(CODEGEN_INSERTABLE_BUILDER_PATH),
            tables,
            conn,
        )?;

        let insertable_module = Ident::new(CODEGEN_INSERTABLE_PATH, proc_macro2::Span::call_site());
        let insertable_variant_module =
            Ident::new(CODEGEN_INSERTABLE_VARIANT_PATH, proc_macro2::Span::call_site());
        let insertable_variant_builder_module =
            Ident::new(CODEGEN_INSERTABLE_BUILDER_PATH, proc_macro2::Span::call_site());

        let table_module = root.with_extension("rs");
        std::fs::write(
            &table_module,
            self.beautify_code(&quote! {
                mod #insertable_module;
                mod #insertable_variant_module;
                mod #insertable_variant_builder_module;
            })?,
        )?;

        Ok(())
    }
}
