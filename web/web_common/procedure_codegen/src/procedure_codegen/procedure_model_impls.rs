//! Submodule implementing the generation of the `ProcedureModel` trait for
//! all procedure models.

use diesel::PgConnection;

use super::ProcedureCodegen;
use crate::utils::procedure_models;

impl ProcedureCodegen {
    /// Generates the implementation of the `ProcedureModel` trait for all
    /// procedure models.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a PostgreSQL connection.
    ///
    /// # Errors
    ///
    /// * Returns an error if the code generation fails.
    pub(super) fn procedure_model_impls(
        &self,
        conn: &mut PgConnection,
    ) -> Result<(), crate::errors::Error> {
        for _procedure_model in procedure_models(conn)? {
            todo!("Implement procedure impls");
        }
        Ok(())
    }
}
