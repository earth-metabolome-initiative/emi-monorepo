//! Submodule providing the code to generate the implementation of the
//! `Loadable` traits for all requiring tables.

use std::path::Path;

use diesel::PgConnection;
use proc_macro2::TokenStream;
use quote::quote;

use crate::{Codegen, Table};

impl Codegen<'_> {
    /// Generates the [`Loadable`](web_common_traits::database::Loadable) traits
    /// implementation for the tables
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
        let feature_flag = self.syntax.as_feature_flag();
        let connection_type = self.syntax.as_connection_type();
        for table in tables {
            // First we need to check wether the table has a PK
            if !table.has_primary_keys(conn)? {
                continue;
            }

            let primary_key_type = table.primary_key_type(conn)?;
            let primary_key_attributes = table.primary_key_attributes(conn)?;

            let table_struct = table.import_struct_path()?;
            let snake_case_ident = table.snake_case_ident()?;
            // We create a file for each table
            let table_file = root.join(format!("{}.rs", table.snake_case_name()?));

            // impl `Loadable`` for struct_ident
            std::fs::write(
                &table_file,
                self.beautify_code(&quote! {
                    #feature_flag
                    impl web_common_traits::prelude::Loadable for #table_struct{
                        type Conn = #connection_type;
                        type PrimaryKey = #primary_key_type;

                        async fn load(
                            #primary_key_attributes: &#primary_key_type,
                            conn: &mut Self::Conn
                        ) -> Result<Option<Self>, diesel::result::Error> {
                            use diesel_async::RunQueryDsl;
                            use diesel::associations::HasTable;
                            use diesel::{QueryDsl, OptionalExtension};
                            #table_struct::table()
                                .find(#primary_key_attributes)
                                .first::<Self>(conn)
                                .await
                                .optional()
                        }

                        async fn load_all(conn: &mut Self::Conn
                        ) -> Result<Vec<Self>, diesel::result::Error> {
                            use diesel_async::RunQueryDsl;
                            use diesel::associations::HasTable;
                            #table_struct::table().load::<Self>(conn).await
                        }
                    }
                })?,
            )?;

            table_deletable_main_module.extend(quote::quote! {
                mod #snake_case_ident;
            });
        }

        let table_module = root.with_extension("rs");
        std::fs::write(&table_module, self.beautify_code(&table_deletable_main_module)?)?;

        Ok(())
    }
}
