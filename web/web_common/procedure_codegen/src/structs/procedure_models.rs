//! Submodule defining a function to return the set of `ProcedureModel`s tables
//! defined in the database.

use diesel::PgConnection;
use webcodegen::Table;

use crate::Procedure;

const PROCEDURE_MODELS_SCHEMA: &str = "procedure_models";

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
/// Struct representing a procedure model table.
pub(crate) struct ProcedureModel {
    /// The underlying table.
    table: Table,
}

impl AsRef<Table> for ProcedureModel {
    fn as_ref(&self) -> &Table {
        &self.table
    }
}

impl ProcedureModel {
    /// Returns whether the given table is a procedure model table.
    ///
    /// # Arguments
    ///
    /// * `table` - The table to check.
    /// * `conn` - A mutable reference to a PostgreSQL connection.
    ///
    /// # Errors
    ///
    /// * If the database query fails, returns a `diesel::result::Error`.
    /// * If the table is not recognized as a procedure model, returns a custom
    ///   error.
    pub(super) fn must_be_procedure_model_table(
        table: &Table,
        conn: &mut PgConnection,
    ) -> Result<(), crate::errors::Error> {
        if table.table_schema != PROCEDURE_MODELS_SCHEMA {
            return Err(crate::errors::ProcedureModelError::NotAProcedureModelTable(Box::new(
                table.clone(),
            ))
            .into());
        }

        Ok(())
    }

    /// Returns the `Procedure` associated with this procedure model.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a PostgreSQL connection.
    ///
    /// # Errors
    ///
    /// * If the database query fails, returns a `diesel::result::Error`.
    pub(crate) fn procedure(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Procedure, crate::errors::Error> {
        for procedure in Procedure::load_all(&self.table.table_catalog, conn)? {
            if procedure.procedure_model(conn)? == *self {
                return Ok(procedure);
            }
        }

        unreachable!("A procedure model must be associated with exactly one procedure");
    }

    /// Loads a `ProcedureModel` by name.
    ///
    /// # Arguments
    ///
    /// * `table_catalog` - The name of the database catalog (database name).
    /// * `table_name` - The name of the procedure model table.
    /// * `conn` - A mutable reference to a PostgreSQL connection.
    ///
    /// # Errors
    ///
    /// * If the database query fails, returns a `diesel::result::Error`.
    /// * If the table associated with the procedure model is not recognized as
    ///   a procedure model, returns a custom error.
    pub(crate) fn load_by_name(
        table_catalog: &str,
        table_name: &str,
        conn: &mut PgConnection,
    ) -> Result<Self, crate::errors::Error> {
        let table = Table::load(conn, table_name, Some(PROCEDURE_MODELS_SCHEMA), table_catalog)?;
        Self::must_be_procedure_model_table(&table, conn)?;
        Ok(Self { table })
    }

    /// Returns the set of `ProcedureModel`s tables defined in the database.
    ///
    /// # Arguments
    ///
    /// * `table_catalog` - The name of the database catalog (database name).
    /// * `conn` - A mutable reference to a PostgreSQL connection.
    ///
    /// # Errors
    ///
    /// * If the database query fails, returns a `diesel::result::Error`.
    pub(crate) fn load_all(
        table_catalog: &str,
        conn: &mut PgConnection,
    ) -> Result<Vec<Self>, crate::errors::Error> {
        let mut procedure_models = Vec::new();
        for table in Table::load_all(conn, table_catalog)? {
            if table.table_schema != PROCEDURE_MODELS_SCHEMA {
                continue;
            }

            Self::must_be_procedure_model_table(&table, conn)?;
            procedure_models.push(Self { table });
        }
        Ok(procedure_models)
    }
}
