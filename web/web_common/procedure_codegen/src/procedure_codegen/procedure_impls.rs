//! Submodule implementing the generation of the `Procedure` trait for
//! all procedures.

use std::path::Path;

use diesel::PgConnection;
use quote::quote;
use webcodegen::TableLike;

use super::ProcedureCodegen;
use crate::Procedure;

impl<'a> ProcedureCodegen<'a> {
    /// Generates the implementation of the `Procedure` trait for all
    /// procedures.
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
    pub(super) fn procedure_impls(
        &self,
        root: &Path,
        table_catalog: &str,
        conn: &mut PgConnection,
    ) -> Result<(), crate::errors::Error> {
        for procedure in Procedure::load_all(table_catalog, conn)? {
            let procedure_name = procedure.snake_case_name()?;
            let procedure_type = procedure.import_struct_path()?;
            let submodule = root.join(procedure_name).with_extension("rs");
            let procedure_model = procedure.procedure_model(conn)?;
            let procedure_model_type = procedure_model.import_struct_path()?;

            std::fs::write(
                submodule,
                self.beautify_code(&quote! {
                    impl Procedure for #procedure_type {
                        type Model = #procedure_model_type;
                    }
                })?,
            )?;
        }

        Ok(())
    }
}
