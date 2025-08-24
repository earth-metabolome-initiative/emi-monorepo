//! Submodule defining a function to return the set of `Procedure`s tables
//! defined in the database.

use diesel::PgConnection;
use webcodegen::Table;

use crate::{ProcedureModel, errors::ProcedureError};

const PROCEDURES_SCHEMA: &str = "procedures";

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
/// Struct representing a procedure table.
pub(crate) struct Procedure {
    /// The underlying table.
    table: Table,
}

impl AsRef<Table> for Procedure {
    fn as_ref(&self) -> &Table {
        &self.table
    }
}

impl Procedure {
    /// Returns whether the given table is a procedure table.
    ///
    /// # Arguments
    ///
    /// * `table` - The table to check.
    /// * `conn` - A mutable reference to a PostgreSQL connection.
    ///
    /// # Errors
    ///
    /// * If the database query fails, returns a `diesel::result::Error`.
    /// * If the table is not recognized as a procedure, returns a custom error.
    ///
    /// # Implementation details
    ///
    /// * A procedure table is defined as a table in the `procedures` schema.
    /// * It contains a column which is a foreign key to a procedure model
    ///   table.
    pub(super) fn must_be_procedure_table(
        table: &Table,
        conn: &mut PgConnection,
    ) -> Result<(), crate::errors::Error> {
        if table.table_schema != PROCEDURES_SCHEMA {
            return Err(ProcedureError::NotAProcedureTable(Box::new(table.clone())).into());
        }

        let foreign_keys = table.foreign_keys(conn)?;
        let mut number_of_found_foreign_keys = 0;
        for foreign_key in foreign_keys {
            let Some(foreign_table) = foreign_key.foreign_table(conn)? else {
                continue;
            };
            if ProcedureModel::must_be_procedure_model_table(&foreign_table, conn).is_ok() {
                number_of_found_foreign_keys += 1;
            }
        }

        if number_of_found_foreign_keys != 1 {
            return Err(ProcedureError::NotAProcedureTable(Box::new(table.clone())).into());
        }

        Ok(())
    }

    /// Returns the `ProcedureModel` associated with this procedure.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a PostgreSQL connection.
    ///
    /// # Errors
    ///
    /// * If the database query fails, returns a `diesel::result::Error`.
    pub(crate) fn procedure_model(
        &self,
        conn: &mut PgConnection,
    ) -> Result<ProcedureModel, crate::errors::Error> {
        let foreign_keys = self.table.foreign_keys(conn)?;
        for foreign_key in foreign_keys {
            let Some(foreign_table) = foreign_key.foreign_table(conn)? else {
                continue;
            };
            if ProcedureModel::must_be_procedure_model_table(&foreign_table, conn).is_ok() {
                return ProcedureModel::load_by_name(
                    &self.table.table_catalog,
                    &foreign_table.table_name,
                    conn,
                );
            }
        }
        Err(ProcedureError::NotAProcedureTable(Box::new(self.table.clone())).into())
    }

    /// Loads a `Procedure` by name.
    ///
    /// # Arguments
    ///
    /// * `table_catalog` - The name of the database catalog (database name).
    /// * `table_name` - The name of the procedure table.
    /// * `conn` - A mutable reference to a PostgreSQL connection.
    ///
    /// # Errors
    ///
    /// * If the database query fails, returns a `diesel::result::Error`.
    /// * If the table associated with the procedure is not recognized as a
    ///   procedure, returns a custom error.
    pub(crate) fn load_by_name(
        table_catalog: &str,
        table_name: &str,
        conn: &mut PgConnection,
    ) -> Result<Self, crate::errors::Error> {
        let table = Table::load(conn, table_name, Some(PROCEDURES_SCHEMA), table_catalog)?;
        Self::must_be_procedure_table(&table, conn)?;
        Ok(Self { table })
    }

    /// Returns the set of `Procedure`s tables defined in the database.
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
        let mut procedures = Vec::new();
        for table in Table::load_all(conn, table_catalog)? {
            if table.table_schema != PROCEDURES_SCHEMA {
                continue;
            }
            Self::must_be_procedure_table(&table, conn)?;
            procedures.push(Self { table });
        }
        Ok(procedures)
    }
}
