//! Submodule providing the code to generate the implementation of the attribute
//! traits for all requiring methods.

use std::path::Path;

use diesel_async::AsyncPgConnection;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::{Codegen, Table};

const ATTRIBUTE_TRAITS: &[(&str, &str)] = &[("Described", "description")];

impl Codegen<'_> {
    /// Generates the attribute traits implementation for the tables
    ///
    /// # Arguments
    ///
    /// * `root` - The root path for the generated code.
    /// * `tables` - The list of tables for which to generate the diesel code.
    /// * `conn` - A mutable reference to a `PgConnection`.
    pub(super) async fn generate_attribute_impls(
        &self,
        root: &Path,
        tables: &[Table],
        conn: &mut AsyncPgConnection,
    ) -> Result<(), crate::errors::WebCodeGenError> {
        std::fs::create_dir_all(root)?;
        // We generate each table in a separate document under the provided root, and we
        // collect all of the imported modules in a public one.
        let mut table_deletable_main_module = TokenStream::new();
        for table in tables {
            let columns = table.columns(conn).await?;
            let struct_ident = table.import_struct_path()?;
            let table_file = root.join(format!("{}.rs", table.snake_case_ident()?));
            let table_snake_case_ident = table.snake_case_ident()?;
            let mut attribute_traits = TokenStream::new();
            for (trait_name, attribute_name) in ATTRIBUTE_TRAITS {
                let Some(column) = columns
                    .iter()
                    .find(|column| &column.snake_case_name().unwrap_or_default() == attribute_name)
                else {
                    continue;
                };
                let reference_type = column.rust_ref_data_type(conn).await?;
                let trait_ident = Ident::new(trait_name, proc_macro2::Span::call_site());
                let method_name = Ident::new(attribute_name, proc_macro2::Span::call_site());
                attribute_traits.extend(quote! {
                    impl web_common_traits::prelude::#trait_ident for #struct_ident {
                        fn #method_name(&self) -> #reference_type {
                            &self.#method_name
                        }
                    }
                });
            }

            if attribute_traits.is_empty() {
                continue;
            }

            std::fs::write(&table_file, self.beautify_code(&attribute_traits)?)?;

            table_deletable_main_module.extend(quote::quote! {
                mod #table_snake_case_ident;
            });
        }

        let table_module = root.with_extension("rs");
        std::fs::write(&table_module, self.beautify_code(&table_deletable_main_module)?)?;

        Ok(())
    }
}
