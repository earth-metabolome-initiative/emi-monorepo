//! Submodule implementing code relative to diesel's [`joinable`](https://docs.rs/diesel/latest/diesel/macro.joinable.html) macro.

use std::path::Path;

use diesel::PgConnection;
use syn::Ident;

use proc_macro2::TokenStream;

use crate::Table;

use super::Codegen;

impl<'a> Codegen<'a> {
    /// Generate implementations of the `joinable` diesel macro.
    ///
    /// # Arguments
    ///
    /// * `root` - The root path for the generated code.
    /// * `tables` - The list of tables for which to generate the diesel code.
    /// * `conn` - A mutable reference to a `PgConnection`.
    ///
    pub(crate) fn generate_joinable_macro(
        &self,
        root: &Path,
        tables: &[Table],
        conn: &mut PgConnection,
    ) -> Result<(), crate::errors::WebCodeGenError> {
        // https://github.com/earth-metabolome-initiative/emi-monorepo/issues/36
        // todo!("See issue #36")
        // As a first step, we create a directory for the generated code.
        std::fs::create_dir_all(&root)?;
        // Then we create a Token stream for the main module that will expose the individual tables.
        let mut joinable_main_module = TokenStream::new();
        // We will now iterate on the various tables
        for table in tables {
            // We retrieve the table identifier
            let table_identifier =
                Ident::new(&table.snake_case_name()?, proc_macro2::Span::call_site());
            // We create a file for each table
            let table_file = root.join(format!("{}.rs", table.snake_case_name()?));
            // We generate the content of the file
            let mut table_content = TokenStream::new();
        
            for foreign_key in table.foreign_keys(conn)? {
                // For each table we retrieve the foreign key(s).
                // First we fetch the foreign table (and its primary key)
                let Some((foreign_table, foreign_table_pk)) = foreign_key.foreign_table(conn)?
                else {
                    continue;
                };
                // We retrieve the foreign_table identifier
                let foreign_table_identifier =
                    Ident::new(&foreign_table.snake_case_name()?, proc_macro2::Span::call_site());
                // We retrieve the foreign_table_pk identifer
                let foreign_table_pk_identifier = Ident::new(
                    &foreign_table_pk.snake_case_name()?,
                    proc_macro2::Span::call_site(),
                );
                // Using TokeStream we write the  joinable!(table -> foreign_table (foreign_key));
                table_content.extend(quote::quote! {
                    joinable!(#table_identifier -> #foreign_table_identifier (#foreign_table_pk_identifier));
                });

                std::fs::write(&table_file, self.beautify_code(table_content)?)?;
            }
            joinable_main_module.extend(quote::quote! {
                pub mod #table_identifier;
            });
        }

        let table_module = root.with_extension("rs");
        std::fs::write(&table_module, self.beautify_code(joinable_main_module)?)?;

        Ok(())
    }
}
