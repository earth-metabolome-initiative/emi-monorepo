//! Submodule implementing the generation of enums for procedures builders.

use std::path::Path;

use diesel::PgConnection;

use super::ProcedureCodegen;
use crate::Procedure;

impl<'a> ProcedureCodegen<'a> {
    /// Generates the enum codegen.
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
    pub(super) fn enum_codegen(
        &self,
        root: &Path,
        table_catalog: &str,
        conn: &mut PgConnection,
    ) -> Result<(), crate::errors::Error> {
        for _procedure in Procedure::load_all(table_catalog, conn)? {
            todo!("Implement enum codegen");
        }
        Ok(())
    }
}
