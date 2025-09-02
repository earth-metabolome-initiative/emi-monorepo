//! Submodule defining a function to return the set of `Procedure`s tables
//! defined in the database.

use diesel::PgConnection;
use webcodegen::{KeyColumnUsage, Table};

use crate::{
    ProcedureTemplate, errors::ProcedureError, procedure_templates::PROCEDURE_TEMPLATES_TABLE_NAME,
};

/// The name of the procedure table.
pub const PROCEDURES_TABLE_NAME: &str = "procedures";

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
/// Struct representing a procedure table.
pub(crate) struct Procedure {
    /// The underlying table.
    pub(super) table: Table,
}

impl AsRef<Table> for Procedure {
    fn as_ref(&self) -> &Table {
        &self.table
    }
}

/// Returns the foreign key linking the given procedure table to its
/// procedure template.
///
/// # Arguments
///
/// * `procedure_table` - The procedure table.
/// * `conn` - A mutable reference to a PostgreSQL connection.
///
/// # Errors
///
/// * If the database query fails, returns a `diesel::result::Error`.
/// * If the table is not recognized as a procedure, returns a custom error.
fn procedure_template_foreign_key(
    procedure_table: &Table,
    conn: &mut PgConnection,
) -> Result<Option<KeyColumnUsage>, crate::errors::Error> {
    if procedure_table.table_name == PROCEDURES_TABLE_NAME {
        return Ok(None);
    }

    for foreign_key in procedure_table.ancestral_same_as_foreign_keys(conn)? {
        let local_columns = foreign_key.columns(conn)?;
        assert_eq!(local_columns.len(), 2);
        for foreign_key in local_columns[1].foreign_primary_keys(conn)? {
            let Some(foreign_table) = foreign_key.foreign_table(conn)? else {
                continue;
            };
            if ProcedureTemplate::from_table(foreign_table.clone(), conn).is_ok() {
                return Ok(Some(foreign_key));
            }
        }
    }

    Err(ProcedureError::NotAProcedureTable(Box::new(procedure_table.clone())).into())
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
    /// * It contains a column which is a foreign key to a procedure template
    ///   table.
    pub(crate) fn must_be_procedure_table(
        table: &Table,
        conn: &mut PgConnection,
    ) -> Result<(), crate::errors::Error> {
        if table.table_name == PROCEDURES_TABLE_NAME {
            return Ok(());
        }

        let procedure = Table::load(conn, PROCEDURES_TABLE_NAME, "public", &table.table_catalog)?;

        if !table.is_extending(&procedure, conn)? {
            return Err(ProcedureError::NotAProcedureTable(Box::new(table.clone())).into());
        }

        procedure_template_foreign_key(table, conn)?;

        Ok(())
    }

    /// Returns the foreign key linking this procedure to its procedure
    /// template.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a PostgreSQL connection.
    ///
    /// # Errors
    ///
    /// * If the database query fails, returns a `diesel::result::Error`.
    pub(crate) fn procedure_template_foreign_key(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Option<KeyColumnUsage>, crate::errors::Error> {
        procedure_template_foreign_key(&self.table, conn)
    }

    /// Returns the `ProcedureTemplate` associated with this procedure.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a PostgreSQL connection.
    ///
    /// # Errors
    ///
    /// * If the database query fails, returns a `diesel::result::Error`.
    pub(crate) fn procedure_template(
        &self,
        conn: &mut PgConnection,
    ) -> Result<ProcedureTemplate, crate::errors::Error> {
        ProcedureTemplate::from_table(
            if let Some(foreign_key) = self.procedure_template_foreign_key(conn)? {
                foreign_key.foreign_table(conn)?.expect("Foreign key must have a foreign table")
            } else {
                Table::load(
                    conn,
                    PROCEDURE_TEMPLATES_TABLE_NAME,
                    "public",
                    &self.table.table_catalog,
                )?
            },
            conn,
        )
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
        let table = Table::load(conn, table_name, "public", table_catalog)?;
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

        for table in Table::load_all(conn, table_catalog, "public")? {
            if Self::must_be_procedure_table(&table, conn).is_err() {
                continue;
            }
            procedures.push(Self { table });
        }
        Ok(procedures)
    }
}
