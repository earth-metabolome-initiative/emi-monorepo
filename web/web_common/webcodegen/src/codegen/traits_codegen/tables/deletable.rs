//! Submodule providing the code to generate the implementation of the Deletable traits for all requiring methods.


use std::path::Path;

use diesel::PgConnection;
use syn::Ident;
use quote::quote;


use crate::errors::WebCodeGenError;
use crate::Codegen;
use crate::Table;

impl Codegen<'_> {
    /// Generates the Deletable traits implementation for the tables
    ///
    /// # Arguments
    ///
    /// * `root` - The root path for the generated code.
    /// * `tables` - The list of tables for which to generate the diesel code.
    /// * `conn` - A mutable reference to a `PgConnection`.
    pub(super) fn generate_deletable_impls(
        &self,
        root: &Path,
        tables: &[Table],
        conn: &mut PgConnection,
    ) -> Result<(), crate::errors::WebCodeGenError> {
        std::fs::create_dir_all(root)?;
        // We generate each table in a separate document under the provided root, and we
        // collect all of the imported modules in a public one.
        let mut table_deletable_main_module = quote::quote! {};
        for table in tables {
            
            //First we need to check wether the table has a PK
            if !table.has_primary_keys(conn)? {
                continue;
            }

            let primary_key_columns = table.primary_key_columns(conn)?;
            let sanitized_table_ident = table.snake_case_ident()?;
            let table_struct = table.import_path()?;
            // We create a file for each table
            let table_file = root.join(format!("{}.rs", table.snake_case_name()?));

            let where_clause = primary_key_columns
                .iter()
                .map(|column| {
                    let column_name: Ident = column.sanitized_snake_case_ident()?;
                    Ok(quote! {
                        #sanitized_table_ident::#column_name.eq(&self.#column_name)
                    })
                })
                .collect::<Result<Vec<_>, WebCodeGenError>>()?;
    
            // Join the where clauses with an and
            let where_clause = where_clause
                .into_iter()
                .reduce(|a, b| quote! { diesel::BoolExpressionMethods::and(#a, #b) })
                .unwrap_or_default();
    
            let columns_feature_flag_name = table.diesel_feature_flag_name(conn)?;

            let deletable_impl = quote! {
                #[cfg(feature = #columns_feature_flag_name)]
                impl web_common_traits::prelude::Deletable for #table_struct{
                    async fn delete(&self, conn: &mut web_common_traits::prelude::DBConn) -> Result<usize, diesel::result::Error> {
                        diesel::delete(#sanitized_table_ident::table.filter(#where_clause)).execute(conn).await
                    }
                }
            };

            // impl Deletable for struct_ident
            std::fs::write(&table_file, self.beautify_code(&deletable_impl)?)?;

            table_deletable_main_module.extend(quote::quote! {
                mod #sanitized_table_ident;
            });
        }

        let table_module = root.with_extension("rs");
        std::fs::write(&table_module, self.beautify_code(&table_deletable_main_module)?)?;

        Ok(())
    }
}
