//! Submodule implementing the generation of the `ProcedureTemplate` trait for
//! all procedure templates.

use std::path::Path;

use diesel::PgConnection;
use quote::quote;
use webcodegen::TableLike;

use super::ProcedureCodegen;
use crate::ProcedureTemplate;

impl<'a> ProcedureCodegen<'a> {
    /// Generates the implementation of the `ProcedureTemplate` trait for all
    /// procedure templates.
    ///
    /// # Arguments
    ///
    /// * `root` - The root path where to output the generated code.
    /// * `table_catalog` - The name of the database catalog (database name).
    /// * `conn` - A mutable reference to a PostgreSQL connection.
    ///
    /// # Errors
    ///
    /// * Returns an error if the code generation fails.
    pub(super) fn procedure_template_impls(
        &self,
        root: &Path,
        table_catalog: &str,
        conn: &mut PgConnection,
    ) -> Result<(), crate::errors::Error> {
        let mut submodules = Vec::new();
        for procedure_template in ProcedureTemplate::load_all(table_catalog, conn)? {
            let procedure_template_name = procedure_template.snake_case_name()?;
            let procedure_template_ident = procedure_template.snake_case_ident()?;
            let procedure_template_type = procedure_template.import_struct_path()?;
            let submodule: std::path::PathBuf =
                root.join(procedure_template_name).with_extension("rs");
            let procedure = procedure_template.procedure(conn)?;
            let procedure_type = procedure.import_struct_path()?;
            submodules.push(quote! {
                mod #procedure_template_ident;
            });
            std::fs::write(
                submodule,
                self.beautify_code(&quote! {
                    impl web_common_traits::prelude::ProcedureTemplate for #procedure_template_type {
                        type Procedure = #procedure_type;
                    }
                })?,
            )?;
        }

        let submodule_path = root.with_extension("rs");
        std::fs::write(
            submodule_path,
            self.beautify_code(&quote! {
                #(#submodules)*
            })?,
        )?;

        Ok(())
    }
}
