//! Submodule implementing the generation of the `Procedure` trait for
//! all procedures.

use diesel::PgConnection;

use super::ProcedureCodegen;
use crate::utils::procedures;

impl ProcedureCodegen {
    /// Generates the implementation of the `Procedure` trait for all
    /// procedures.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a PostgreSQL connection.
    ///
    /// # Errors
    ///
    /// * Returns an error if the code generation fails.
    pub(super) fn procedure_impls(
        &self,
        conn: &mut PgConnection,
    ) -> Result<(), crate::errors::Error> {
        for _procedure in procedures(conn)? {
            todo!("Implement procedure impls");
        }
        Ok(())
    }
}
