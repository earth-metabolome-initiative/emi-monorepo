//! Code generating the table structs.

use std::path::Path;

use diesel::PgConnection;
use proc_macro2::TokenStream;

use super::Codegen;
use crate::{
    Table,
    codegen::{CODEGEN_INSERTABLES_PATH, CODEGEN_MOST_CONCRETE_VARIANTS_PATH},
    traits::TableLike,
};

mod crud;
mod insertables;
mod most_concrete_variant;

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
        let mut table_main_module = TokenStream::new();
        for table in tables {
            let table_identifier = table.snake_case_ident()?;
            let table_file = root.join(format!("{}.rs", table.snake_case_name()?));
            let table_struct = table.struct_ident()?;
            let table_content = table.to_syn(self.enable_yew, conn)?;
            let foreign_key_methods = if self.enable_foreign_trait {
                table.foreign_key_methods(conn)?
            } else {
                TokenStream::new()
            };
            let from_attributes = table.from_attributes(conn)?;

            std::fs::write(
                &table_file,
                self.beautify_code(&quote::quote! {
                    #table_content
                    impl #table_struct {
                        #foreign_key_methods
                        #from_attributes
                    }

                    impl AsRef<#table_struct> for #table_struct {
                        fn as_ref(&self) -> &#table_struct {
                            self
                        }
                    }
                }),
            )?;

            table_main_module.extend(quote::quote! {
                pub mod #table_identifier;
                pub use #table_identifier::#table_struct;
            });
        }

        if self.enable_insertable_trait {
            self.generate_insertable_structs(
                root.join(CODEGEN_INSERTABLES_PATH).as_path(),
                tables,
                conn,
            )?;
            table_main_module.extend(quote::quote! {
                pub mod insertables;
            });
        }

        if self.enable_most_concrete_variant_trait {
            self.generate_most_concrete_variant_structs(
                root.join(CODEGEN_MOST_CONCRETE_VARIANTS_PATH).as_path(),
                tables,
                conn,
            )?;
            table_main_module.extend(quote::quote! {
                pub mod most_concrete_variants;
            });
        }

        // If any of the CRUD traits are enabled, we generate the Table names enum.
        if self.should_generate_crud() {
            self.generate_table_names_enumeration(root, tables)?;
            table_main_module.extend(quote::quote! {
                pub mod table_names;
            });

            self.generate_table_primary_keys_enumeration(root, tables, conn)?;
            table_main_module.extend(quote::quote! {
                pub mod table_primary_keys;
            });

            self.generate_rows_enumeration(&root.join("rows"), tables, conn)?;
            table_main_module.extend(quote::quote! {
                pub mod rows;
            });

            self.generate_row_enumeration(&root.join("row"), tables, conn)?;
            table_main_module.extend(quote::quote! {
                pub mod row;
            });
        }

        let table_module = root.with_extension("rs");
        std::fs::write(&table_module, self.beautify_code(&table_main_module))?;

        Ok(())
    }
}
