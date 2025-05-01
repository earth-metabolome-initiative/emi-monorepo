//! Submodule providing the code to generate the implementation of the
//! [`Deletable`] traits for all requiring tables.

use std::path::Path;

use diesel_async::AsyncPgConnection;
use proc_macro2::TokenStream;
use quote::quote;

use crate::{Codegen, Table, codegen::Syntax};

impl Codegen<'_> {
    /// Generates the [`Deletable`] traits implementation for the tables
    ///
    /// # Arguments
    ///
    /// * `root` - The root path for the generated code.
    /// * `tables` - The list of tables for which to generate the diesel code.
    /// * `conn` - A mutable reference to a `PgConnection`.
    pub(super) async fn generate_deletable_impls(
        &self,
        root: &Path,
        tables: &[Table],
        conn: &mut AsyncPgConnection,
    ) -> Result<(), crate::errors::WebCodeGenError> {
        std::fs::create_dir_all(root)?;
        // We generate each table in a separate document under the provided root, and we
        // collect all of the imported modules in a public one.
        let mut table_deletable_main_module = TokenStream::new();
        let syntax = Syntax::PostgreSQL;
        let feature_flag = syntax.as_feature_flag();
        let connection_type = syntax.as_connection_type(true);
        let Some(user_table) = self.users_table else {
            return Err(crate::errors::CodeGenerationError::UserTableNotProvided.into());
        };
        let user_table_id = user_table.primary_key_type(conn).await?;
        for table in tables {
            // First we need to check wether the table has a PK
            if !table.allows_updatable(conn).await? {
                continue;
            }

            let table_struct = table.import_struct_path()?;
            let snake_case_ident = table.snake_case_ident()?;
            // We create a file for each table
            let table_file = root.join(format!("{}.rs", table.snake_case_name()?));

            // impl Deletable for struct_ident
            std::fs::write(&table_file, self.beautify_code(&quote! {
                #feature_flag
                impl web_common_traits::prelude::Deletable for #table_struct{
                    type Conn = #connection_type;
                    type UserId = #user_table_id;

                    async fn delete(
                        &self,
                        user_id: &Self::UserId,
                        conn: &mut Self::Conn
                    ) -> Result<bool, web_common_traits::database::DeleteError> {
                        use diesel_async::RunQueryDsl;
                        use diesel::{QueryDsl, Identifiable};
                        use diesel::associations::HasTable;
                        use web_common_traits::database::Updatable;

                        if !self.can_update(user_id, conn).await? {
                            return Err(backend_request_errors::BackendRequestError::Unauthorized.into());
                        }

                        Ok(diesel::delete(Self::table().find(<&Self as Identifiable>::id(self))).execute(conn).await.map(|x| x > 0)?)
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
