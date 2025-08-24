//! Submodule implementing the generation of the `ProcedureInitializer` trait
//! for all procedures.

use std::path::Path;

use diesel::PgConnection;

use super::ProcedureCodegen;

impl<'a> ProcedureCodegen<'a> {
    /// Generates the implementation of the `ProcedureInitializer` trait for all
    /// procedures.
    ///
    /// # Arguments
    ///
    /// * `root` - The root path where to output the generated code.
    /// * `conn` - A mutable reference to a PostgreSQL connection.
    ///
    /// # Errors
    ///
    /// * Returns an error if the code generation fails.
    pub(super) fn procedure_initializer_impls(
        &self,
        root: &Path,
        _conn: &mut PgConnection,
    ) -> Result<(), crate::errors::Error> {
        todo!("Implement procedure initializer impls");
    }
}
