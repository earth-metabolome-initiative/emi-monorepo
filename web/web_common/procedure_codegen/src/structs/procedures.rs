//! Submodule defining a function to return the set of `Procedure`s tables
//! defined in the database.

use diesel::PgConnection;
use webcodegen::{KeyColumnUsage, Table};

use crate::{
    PROCEDURE_TEMPLATES_SCHEMA, ProcedureTemplate, errors::ProcedureError,
    procedure_templates::PROCEDURE_TEMPLATES_TABLE_NAME,
};

/// The schema in which procedure tables are defined.
pub const PROCEDURES_SCHEMA: &str = "procedures";
/// The name of the procedure table.
pub const PROCEDURES_TABLE_NAME: &str = "procedures";

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
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
    if procedure_table.table_schema == PROCEDURES_SCHEMA
        && procedure_table.table_name == PROCEDURES_TABLE_NAME
    {
        return Ok(None);
    }

    let primary_key_columns = procedure_table.primary_key_columns(conn)?;

    if primary_key_columns.len() != 1 {
        return Err(ProcedureError::NotAProcedureTable(Box::new(procedure_table.clone())).into());
    }

    let primary_key_column = &primary_key_columns[0];

    for foreign_key in procedure_table.foreign_keys(conn)? {
        let Some(foreign_table) = foreign_key.foreign_table(conn)? else {
            continue;
        };
        // If the foreign table is not a procedure template table, continue searching.
        if ProcedureTemplate::must_be_procedure_template_table(&foreign_table).is_err() {
            continue;
        }

        // If the foreign key has more than one column, continue searching.
        let local_columns = foreign_key.columns(conn)?;
        if local_columns.len() != 1 {
            continue;
        }

        let local_column = &local_columns[0];

        // For the local column, there must exist a same-as relationship with the
        // primary key column which links to the procedure table: (procedure,
        // procedure_template).
        for same_as_constraint in local_column.same_as_constraints(conn)? {
            let Some(foreign_table) = same_as_constraint.foreign_table(conn)? else {
                continue;
            };
            if foreign_table.table_schema != PROCEDURES_SCHEMA
                || foreign_table.table_name != PROCEDURES_TABLE_NAME
            {
                continue;
            }
            let columns = same_as_constraint.columns(conn)?;
            if columns.len() != 2 {
                continue;
            }
            if &columns[0] == primary_key_column && &columns[1] == local_column {
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
        if table.table_schema != PROCEDURES_SCHEMA {
            return Err(ProcedureError::NotAProcedureTable(Box::new(table.clone())).into());
        }

        if table.table_name == PROCEDURES_TABLE_NAME {
            return Ok(());
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
        ProcedureTemplate::try_from(
            if let Some(foreign_key) = self.procedure_template_foreign_key(conn)? {
                foreign_key.foreign_table(conn)?.expect("Foreign key must have a foreign table")
            } else {
                Table::load(
                    conn,
                    PROCEDURE_TEMPLATES_TABLE_NAME,
                    PROCEDURE_TEMPLATES_SCHEMA,
                    &self.table.table_catalog,
                )?
            },
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
        let table = Table::load(conn, table_name, PROCEDURES_SCHEMA, table_catalog)?;
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
        for table in Table::load_all(conn, table_catalog, PROCEDURES_SCHEMA)? {
            Self::must_be_procedure_table(&table, conn)?;
            procedures.push(Self { table });
        }
        Ok(procedures)
    }
}
