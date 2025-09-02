//! Submodule defining a function to return the set of `ProcedureTemplate`s
//! tables defined in the database.

use diesel::PgConnection;
use webcodegen::{Column, PgIndex, Table};

use crate::Procedure;

/// The name of the procedure templates table.
pub const PROCEDURE_TEMPLATES_TABLE_NAME: &str = "procedure_templates";

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Hash)]
/// Struct representing a procedure template table.
pub(crate) struct ProcedureTemplate {
    /// The underlying table.
    pub(super) table: Table,
}

impl AsRef<Table> for ProcedureTemplate {
    fn as_ref(&self) -> &Table {
        &self.table
    }
}

impl ProcedureTemplate {
    /// Returns whether the given table is a procedure template table.
    ///
    /// # Arguments
    ///
    /// * `table` - The table to check.
    /// * `conn` - A mutable reference to a PostgreSQL connection.
    ///
    /// # Errors
    ///
    /// * If the database query fails, returns a `diesel::result::Error`.
    /// * If the table is not recognized as a procedure template, returns a
    ///   custom error.
    pub(crate) fn must_be_procedure_template_table(
        table: &Table,
        conn: &mut PgConnection,
    ) -> Result<(), crate::errors::Error> {
        if table.table_name == PROCEDURE_TEMPLATES_TABLE_NAME {
            return Ok(());
        }

        let procedure_template =
            Table::load(conn, PROCEDURE_TEMPLATES_TABLE_NAME, "public", &table.table_catalog)?;
        
        if !table.is_extending(&procedure_template, conn)? {
            return Err(crate::errors::ProcedureTemplateError::NotAProcedureTemplateTable(
                Box::new(table.clone()),
            )
            .into());
        }

        Ok(())
    }

    /// Returns the `Procedure` associated with this procedure template.
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
        let mut compatible_procedures = Vec::new();
        for procedure in Procedure::load_all(&self.table.table_catalog, conn)? {
            if procedure.procedure_template(conn)? == *self {
                compatible_procedures.push(procedure);
            }
        }

        if compatible_procedures.is_empty() {
            return Err(crate::errors::ProcedureTemplateError::NoProcedure(Box::new(
                self.table.clone(),
            ))
            .into());
        }

        if compatible_procedures.len() > 1 {
            return Err(crate::errors::ProcedureTemplateError::MultipleProcedures(
                Box::new(self.table.clone()),
                compatible_procedures.into_iter().map(|p| p.table).collect(),
            )
            .into());
        }

        Ok(compatible_procedures.pop().unwrap())
    }

    /// Returns the set of `ProcedureTemplate`s tables defined in the database.
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
        let mut procedure_templates = Vec::new();
        for table in Table::load_all(conn, table_catalog, "public")? {
            if Self::must_be_procedure_template_table(&table, conn).is_err() {
                continue;
            }
            procedure_templates.push(Self { table });
        }
        Ok(procedure_templates)
    }

    /// Constructs a `ProcedureTemplate` from a `Table`.
    /// 
    /// # Arguments
    /// 
    /// * `table` - The table to convert.
    /// * `conn` - A mutable reference to a PostgreSQL connection.
    /// 
    /// # Errors
    /// 
    /// * If the database query fails, returns a `diesel::result::Error`.
    /// * If the table is not recognized as a procedure template, returns a
    ///   custom error.
    pub(crate) fn from_table(
        table: Table,
        conn: &mut PgConnection,
    ) -> Result<Self, crate::errors::Error> {
        Self::must_be_procedure_template_table(&table, conn)?;
        Ok(Self { table })
    }

    /// Returns the primary key column of the procedure template table.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a PostgreSQL connection.
    ///
    /// # Errors
    ///
    /// * If the database query fails, returns a `diesel::result::Error`.
    ///
    /// # Panics
    ///
    /// If the procedure template table does not have exactly one primary key
    /// column.
    pub(crate) fn primary_key_column(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Column, crate::errors::Error> {
        let primary_key_columns = self.as_ref().primary_key_columns(conn)?;
        if primary_key_columns.len() != 1 {
            unreachable!("Procedure template tables must have exactly one primary key column");
        }
        Ok(primary_key_columns.into_iter().next().unwrap())
    }

    /// Returns the same-as indices associating the current procedure template
    /// with a foreign procedure template.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a PostgreSQL connection.
    ///
    /// # Errors
    ///
    /// * If the database query fails, returns a `diesel::result::Error`.
    pub(crate) fn foreign_procedure_template_same_as_indices(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<PgIndex>, crate::errors::Error> {
        let mut foreign_same_as_indices = Vec::new();
        let primary_key_column = self.primary_key_column(conn)?;
        for same_as_index in self.as_ref().same_as_indices(conn)? {
            let columns = same_as_index.columns(conn)?;

            // The index may be a foreign procedure template same-as index if it has
            // exactly two columns.
            if columns.len() != 2 {
                continue;
            }

            // The current same-as index is considered to be linking to a foreign procedure
            // template if the first column of the index is the primary key column of the
            // current procedure template table.
            if columns[0] != primary_key_column {
                continue;
            }

            // The second column of the index must be a foreign key referencing the primary
            // key column of another procedure template table.
            for foreign_key in columns[1].foreign_keys(conn)? {
                let Some(foreign_table) = foreign_key.foreign_table(conn)? else {
                    continue;
                };
                if Self::must_be_procedure_template_table(&foreign_table, conn).is_ok() {
                    foreign_same_as_indices.push(same_as_index.clone());
                    break;
                }
            }
        }
        Ok(foreign_same_as_indices)
    }
}
