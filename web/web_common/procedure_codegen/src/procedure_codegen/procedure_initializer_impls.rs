//! Submodule implementing the generation of the `ProcedureInitializer` trait
//! for all procedures.

use diesel::PgConnection;

use super::ProcedureCodegen;

impl ProcedureCodegen {
    /// Generates the implementation of the `ProcedureInitializer` trait for all
    /// procedures.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a PostgreSQL connection.
    ///
    /// # Errors
    ///
    /// * Returns an error if the code generation fails.
    pub(super) fn procedure_initializer_impls(
        &self,
        _conn: &mut PgConnection,
    ) -> Result<(), crate::errors::Error> {
        todo!("Implement procedure initializer impls");
    }
}
