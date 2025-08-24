//! Submodule providing the code to generate the implementation of the
//! [`Tabular`](web_common_traits::prelude::Tabular) traits for all requiring
//! tables.

use std::path::Path;

use diesel::PgConnection;
use proc_macro2::TokenStream;
use quote::quote;
use crate::traits::TableLike;

use crate::codegen::{Codegen, Table};

impl Codegen<'_> {
    /// Generates the [`Tabular`](web_common_traits::prelude::Tabular) traits
    /// implementation for the tables
    ///
    /// # Arguments
    ///
    /// * `root` - The root path for the generated code.
    /// * `tables` - The list of tables for which to generate the diesel code.
    /// * `conn` - A mutable reference to a `PgConnection`.
    pub(super) fn generate_tabular_impls(
        &self,
        root: &Path,
        tables: &[Table],
        conn: &mut PgConnection,
    ) -> Result<(), crate::errors::WebCodeGenError> {
        std::fs::create_dir_all(root)?;
        // We generate each table in a separate document under the provided root, and we
        // collect all of the imported modules in a public one.
        let mut table_tabular_main_module = TokenStream::new();
        let table_name_enum_path = Self::table_names_enum_path();
        let table_primary_keys_enum_path = Self::table_primary_keys_enum_path();
        for table in tables {
            let table_struct = table.import_struct_path()?;
            let struct_ident = table.struct_ident()?;
            // We create a file for each table
            let table_file = root.join(format!("{}.rs", table.snake_case_name()?));
            let snake_case_ident = table.snake_case_ident()?;

            let primary_key: Vec<TokenStream> = table
                .primary_key_columns(conn)?
                .into_iter()
                .map(|column| {
                    let column_snake_ident = column.snake_case_ident()?;
                    Ok(quote! {
                        self.#column_snake_ident
                    })
                })
                .collect::<Result<Vec<_>, crate::errors::WebCodeGenError>>()?;

            let primary_key: TokenStream = if primary_key.len() == 1 {
                primary_key[0].clone()
            } else {
                quote! {
                    (#(#primary_key),*)
                }
            };

            // impl Tabular for struct_ident
            std::fs::write(
                &table_file,
                self.beautify_code(&quote! {
                    impl web_common_traits::prelude::Tabular for #table_struct{
                        type TableName = #table_name_enum_path;

                        fn table_name(&self) -> Self::TableName {
                            #table_name_enum_path::#struct_ident
                        }
                    }

                    impl web_common_traits::prelude::StaticTabular for #table_struct{
                        fn static_table_name() -> Self::TableName {
                            #table_name_enum_path::#struct_ident
                        }
                    }

                    impl web_common_traits::prelude::Row for #table_struct{
                        type PrimaryKey = #table_primary_keys_enum_path;

                        fn primary_key(&self) -> Self::PrimaryKey {
                            #table_primary_keys_enum_path::#struct_ident(#primary_key)
                        }
                    }
                })?,
            )?;

            table_tabular_main_module.extend(quote::quote! {
                mod #snake_case_ident;
            });
        }

        let table_module = root.with_extension("rs");
        std::fs::write(&table_module, self.beautify_code(&table_tabular_main_module)?)?;

        Ok(())
    }
}
