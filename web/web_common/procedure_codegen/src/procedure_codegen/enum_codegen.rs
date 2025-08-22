//! Submodule implementing the generation of enums for procedures builders.

use diesel::PgConnection;

use super::ProcedureCodegen;
use crate::utils::procedures;

impl ProcedureCodegen {
    /// Generates the enum codegen.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a PostgreSQL connection.
    ///
    /// # Errors
    ///
    /// * Returns an error if the code generation fails.
    pub(super) fn enum_codegen(&self, conn: &mut PgConnection) -> Result<(), crate::errors::Error> {
        for _procedure in procedures(conn)? {
            todo!("Implement enum codegen");
        }
        Ok(())
    }
}
