//! Submodule implementing code relative to diesel's [`joinable`](https://docs.rs/diesel/latest/diesel/macro.joinable.html) macro.

use std::{collections::HashMap, path::Path};

use diesel::PgConnection;
use proc_macro2::TokenStream;
use syn::Ident;

use super::Codegen;
use crate::Table;

impl Codegen<'_> {
    /// Generate implementations of the `joinable` diesel macro.
    ///
    /// # Arguments
    ///
    /// * `root` - The root path for the generated code.
    /// * `tables` - The list of tables for which to generate the diesel code.
    /// * `conn` - A mutable reference to a `PgConnection`.
    pub(crate) fn generate_joinable_macro(
        &self,
        root: &Path,
        tables: &[Table],
        conn: &mut PgConnection,
    ) -> Result<(), crate::errors::WebCodeGenError> {
        // As a first step, we create a directory for the generated code.
        std::fs::create_dir_all(root)?;
        // Then we create a Token stream for the main module that will expose the
        // individual tables.
        let mut joinable_main_module = TokenStream::new();
        // We will now iterate on the various tables
        for table in tables {
            // We retrieve the table identifier
            let table_identifier = table.snake_case_ident()?;
            // We create a file for each table
            let table_file = root.join(format!("{}.rs", table.snake_case_name()?));

            let mut table_hashmap: HashMap<&Table, Option<TokenStream>> = HashMap::new();

            for foreign_key in table.foreign_keys(conn)? {
                // For each table we retrieve the foreign key(s).
                // First we fetch the foreign table (and its primary key)
                let Some((foreign_table, _foreign_table_pk)) = foreign_key.foreign_table(conn)?
                else {
                    continue;
                };

                // There is no need to implement this macro for the case of a foreign table that curresponds
                // with the table itself.
                if &foreign_table == table {
                    continue;
                }

                // We check wether for the current table we havce already created a given token
                // stream.
                if let Some(maybe_token_stream) = table_hashmap.get_mut(&foreign_table) {
                    // Since there is already a foreign_table present in our HashMap we set the
                    // TakenStrem to None
                    *maybe_token_stream = None;
                    continue;
                }

                // Since we want to access to the Table that `foreign_table` outside of the
                // current for loop (foreign_table dying here), we iterate over
                // tge tables array defined out there (tables: &[Table],) and find an entry
                // matching &foreign_table.
                let Some(foreign_table_ref) = tables.iter().find(|t| t == &&foreign_table) else {
                    continue;
                };

                // We retrieve the foreign_table identifier
                let foreign_table_identifier =
                    Ident::new(&foreign_table.snake_case_name()?, proc_macro2::Span::call_site());
                // We retrieve the foreign_key identifer
                let foreign_key_identifier = foreign_key.snake_case_ident()?;
                // Using TokeStream we write the  joinable!(table -> foreign_table
                // (foreign_key));

                let foreign_table_path = foreign_table_ref.import_diesel_path()?;

                table_hashmap.insert(foreign_table_ref, Some(quote::quote! {
                    use #foreign_table_path;

                    diesel::joinable!(#table_identifier -> #foreign_table_identifier (#foreign_key_identifier));
                }
            )
        );
            }

            // We make sure that there is at least Some TokenStream for the table.
            // We create a TokenStream that is a Composite of all TS in the HashMap

            let overall_token_stream: TokenStream = table_hashmap.into_values().flatten().collect();

            if overall_token_stream.is_empty() {
                continue;
            }

            let table_path = table.import_diesel_path()?;

            std::fs::write(
                &table_file,
                self.beautify_code(&quote::quote! {
                use #table_path;
                #overall_token_stream})?,
            )?;

            joinable_main_module.extend(quote::quote! {
                pub mod #table_identifier;
            });
        }

        let table_module = root.with_extension("rs");
        std::fs::write(&table_module, self.beautify_code(&joinable_main_module)?)?;

        Ok(())
    }
}
