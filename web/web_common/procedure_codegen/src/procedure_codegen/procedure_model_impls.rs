//! Submodule implementing the generation of the `ProcedureModel` trait for
//! all procedure models.

use std::path::Path;

use diesel::PgConnection;
use quote::quote;
use webcodegen::TableLike;

use super::ProcedureCodegen;
use crate::ProcedureModel;

impl<'a> ProcedureCodegen<'a> {
    /// Generates the implementation of the `ProcedureModel` trait for all
    /// procedure models.
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
    pub(super) fn procedure_model_impls(
        &self,
        root: &Path,
        table_catalog: &str,
        conn: &mut PgConnection,
    ) -> Result<(), crate::errors::Error> {
        for procedure_model in ProcedureModel::load_all(table_catalog, conn)? {
            let procedure_model_name = procedure_model.snake_case_name()?;
            let procedure_model_type = procedure_model.import_struct_path()?;
            let submodule: std::path::PathBuf =
                root.join(procedure_model_name).with_extension("rs");
            let procedure = procedure_model.procedure(conn)?;
            let procedure_type = procedure.import_struct_path()?;

            std::fs::write(
                submodule,
                self.beautify_code(&quote! {
                    impl ProcedureModel for #procedure_model_type {
                        type Procedure = #procedure_type;
                    }
                })?,
            )?;
        }
        Ok(())
    }
}
