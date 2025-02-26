//! Code generating the table structs.

use std::path::Path;

use diesel::PgConnection;
use syn::Ident;

use super::Codegen;
use crate::Table;

impl Codegen<'_> {
    /// Generate implementations of the structs representing rows of the tables
    /// in the database.
    ///
    /// # Arguments
    ///
    /// * `root` - The root path for the generated code.
    /// * `tables` - The list of tables for which to generate the diesel code.
    /// * `conn` - A mutable reference to a `PgConnection`.
    pub(crate) fn generate_table_structs(
        &self,
        root: &Path,
        tables: &[Table],
        conn: &mut PgConnection,
    ) -> Result<(), crate::errors::WebCodeGenError> {
        std::fs::create_dir_all(root)?;
        // We generate each table in a separate document under the provided root, and we
        // collect all of the imported modules in a public one.
        let mut table_main_module = quote::quote! {};
        for table in tables {
            let table_identifier =
                Ident::new(&table.snake_case_name()?, proc_macro2::Span::call_site());
            let table_file = root.join(format!("{}.rs", table.snake_case_name()?));
            let table_struct = table.struct_ident()?;
            let table_content = table.to_syn(conn)?;
            let foreign_key_methods = table.foreign_key_methods(conn)?;

            std::fs::write(&table_file, self.beautify_code(&quote::quote!{
                #table_content
                impl #table_struct {
                    #foreign_key_methods
                }
            })?)?;

            table_main_module.extend(quote::quote! {
                pub mod #table_identifier;
            });
        }

        let table_module = root.with_extension("rs");
        std::fs::write(&table_module, self.beautify_code(&table_main_module)?)?;

        Ok(())
    }
}
