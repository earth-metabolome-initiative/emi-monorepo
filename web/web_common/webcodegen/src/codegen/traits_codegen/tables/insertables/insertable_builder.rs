//! Submodule providing the code to generate the implementation of the
//! [`InsertableBuilder`] trait for all required tables.

use std::path::Path;

use diesel::PgConnection;
use proc_macro2::TokenStream;

use crate::{Codegen, Table};

impl Codegen<'_> {
    /// Generates the [`InsertableBuilder`] trait implementation for the tables
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
    pub(super) fn generate_insertable_builder_impls(
        &self,
        root: &Path,
        tables: &[Table],
        conn: &mut PgConnection,
    ) -> Result<(), crate::errors::WebCodeGenError> {
        std::fs::create_dir_all(root)?;

        let mut ifvb_main_module = TokenStream::new();
        let syntax_flag = self.syntax.as_feature_flag();

        for table in tables {
            if !table.allows_insertable(conn)? {
                continue;
            }
            // We create a file for each table
            let table_file = root.join(format!("{}.rs", table.snake_case_name()?));
            let table_path = table.import_struct_path()?;
            let table_ident = table.snake_case_ident()?;
            let insertable_frontend_variant = table.insertable_variant_ty()?;
            let insertable_frontend_variant_builder = table.insertable_builder_ty()?;

            std::fs::write(&table_file, self.beautify_code(&quote::quote!{
                #syntax_flag
				impl web_common_traits::database::InsertableBuilder for #insertable_frontend_variant_builder {
					type Row = #table_path;
                    type Product = #insertable_frontend_variant;
				}
			})?)?;

            ifvb_main_module.extend(quote::quote! {
                mod #table_ident;
            });
        }

        let table_module = root.with_extension("rs");
        std::fs::write(&table_module, self.beautify_code(&ifvb_main_module)?)?;

        Ok(())
    }
}
