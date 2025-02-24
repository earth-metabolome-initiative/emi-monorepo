//! Submodule implementing code relative to diesel's [`table`](https://docs.rs/diesel/latest/diesel/macro.table.html) macro.

use std::path::Path;

use diesel::PgConnection;
use itertools::Itertools;
use proc_macro2::TokenStream;

use crate::{errors::WebCodeGenError, Column, PgType, Table};

use super::Codegen;

impl<'a> Codegen<'a> {
    /// Generate implementations of the `table` diesel macro.
    ///
    /// # Arguments
    ///
    /// * `root` - The root path for the generated code.
    /// * `tables` - The list of tables for which to generate the diesel code.
    /// * `conn` - A mutable reference to a `PgConnection`.
    ///
    pub(crate) fn generate_types_macro(
        &self,
        root: &Path,
        tables: &[Table],
        conn: &mut PgConnection,
    ) -> Result<(), crate::errors::WebCodeGenError> {
        std::fs::create_dir_all(&root)?;

        let mut types = tables
            .iter()
            .map(|table| {
                let custom_types = table
                    .columns(conn)?
                    .into_iter()
                    .filter(Column::has_custom_type)
                    .map(|column| PgType::from_name(column.data_type_str(conn)?, conn))
                    .filter_ok(|pg_type| pg_type.is_enum() || pg_type.is_composite())
                    .collect::<Result<Vec<PgType>, WebCodeGenError>>()?;
                let mut additional_custom_types = custom_types.clone();
                for custom_type in custom_types {
                    additional_custom_types.extend(custom_type.internal_custom_types(conn)?);
                }
                Ok(additional_custom_types)
            })
            .collect::<Result<Vec<Vec<PgType>>, WebCodeGenError>>()?
            .into_iter()
            .flatten()
            .collect::<Vec<PgType>>();

        types.sort_unstable();
        types.dedup();

        // We generate each table in a separate document under the provided root, and we
        // collect all of the imported modules in a public one.
        let mut types_main_module = TokenStream::new();

        for r#type in types {
            let type_name = r#type.snake_case_name()?;
            let type_ident = r#type.snake_case_identifier()?;
            let type_file = root.join(format!("{}.rs", type_name));
            std::fs::write(&type_file, self.beautify_code(r#type.to_syn(conn)?)?)?;

            types_main_module.extend(quote::quote! {
                pub mod #type_ident;
            });
        }

        let table_module = root.with_extension("rs");
        std::fs::write(&table_module, self.beautify_code(types_main_module)?)?;

        Ok(())
    }
}
