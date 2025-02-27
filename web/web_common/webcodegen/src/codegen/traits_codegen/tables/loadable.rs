//! Submodule providing the code to generate the implementation of the
//! `Loadable`` traits for all requiring tables.

use std::path::Path;

use diesel::PgConnection;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::{errors::WebCodeGenError, Codegen, Table};

impl Codegen<'_> {
    /// Generates the `Loadable`` traits implementation for the tables
    ///
    /// # Arguments
    ///
    /// * `root` - The root path for the generated code.
    /// * `tables` - The list of tables for which to generate the diesel code.
    /// * `conn` - A mutable reference to a `PgConnection`.
    pub(super) fn generate_loadable_impls(
        &self,
        root: &Path,
        tables: &[Table],
        conn: &mut PgConnection,
    ) -> Result<(), crate::errors::WebCodeGenError> {
        std::fs::create_dir_all(root)?;
        // We generate each table in a separate document under the provided root, and we
        // collect all of the imported modules in a public one.
        let mut table_deletable_main_module = TokenStream::new();
        for table in tables {
            // First we need to check wether the table has a PK
            if !table.has_primary_keys(conn)? {
                continue;
            }

            let primary_key_columns = table.primary_key_columns(conn)?;
            let primary_key_type = table.primary_key_type(conn)?;
            let primary_key_attributes = table.primary_key_attributes(conn)?;

            let table_struct = table.import_struct_path()?;
            let table_diesel = table.import_diesel_path()?;
            let snake_case_ident = table.snake_case_ident()?;
            // We create a file for each table
            let table_file = root.join(format!("{}.rs", table.snake_case_name()?));

            let where_clause = primary_key_columns
                .iter()
                .map(|column| {
                    let column_name: Ident = column.sanitized_snake_case_ident()?;
                    Ok(quote! {
                        diesel::ExpressionMethods::eq(#table_diesel::#column_name, #column_name)
                    })
                })
                .collect::<Result<Vec<_>, WebCodeGenError>>()?;

            // Join the where clauses with an and
            let where_clause = where_clause
                .into_iter()
                .reduce(|a, b| quote! { diesel::BoolExpressionMethods::and(#a, #b) })
                .unwrap_or_default();

            // impl `Loadable`` for struct_ident
            std::fs::write(&table_file, self.beautify_code(&quote! {
                impl web_common_traits::prelude::Loadable for #table_struct{
                    type PrimaryKey = #primary_key_type;

                    #[cfg(feature = "diesel")]
                    async fn load(
                        #primary_key_attributes: &Self::PrimaryKey,
                        conn: &mut web_common_traits::prelude::DBConn
                    ) -> Result<Option<Self>, diesel::result::Error> {
                        use diesel_async::RunQueryDsl;
                        use diesel::{QueryDsl, OptionalExtension};
                        #table_diesel::table
                            .filter(#where_clause)
                            .select(<Self as diesel::SelectableHelper<diesel::pg::Pg>>::as_select())
                            .first::<Self>(conn)
                            .await
                            .optional()
                    }

                    async fn load_all(conn: &mut web_common_traits::prelude::DBConn
                    ) -> Result<Vec<Self>, diesel::result::Error> {
                        use diesel_async::RunQueryDsl;
                        #table_diesel::table.load::<Self>(conn)
                        .await
                    }
                }
            })?)?;

            table_deletable_main_module.extend(quote::quote! {
                mod #snake_case_ident;
            });
        }

        let table_module = root.with_extension("rs");
        std::fs::write(&table_module, self.beautify_code(&table_deletable_main_module)?)?;

        Ok(())
    }
}
