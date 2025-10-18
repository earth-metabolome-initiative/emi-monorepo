use std::fmt::Display;

use diesel::{OptionalExtension, PgConnection, Queryable, QueryableByName, Selectable};

use crate::{
    model_metadata::TableMetadata,
    models::{CheckConstraint, Column, PgIndex, PgTrigger, column},
};

mod cached_queries;
pub(crate) use cached_queries::*;

#[derive(
    Queryable, QueryableByName, PartialEq, Eq, PartialOrd, Ord, Selectable, Debug, Clone, Hash,
)]
#[diesel(table_name = crate::schema::tables)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Struct defining the `information_schema.tables` table.
pub struct Table {
    /// The catalog name of the table.
    pub table_catalog: String,
    /// The schema name of the table.
    pub table_schema: String,
    /// The name of the table.
    pub table_name: String,
    /// The type of the table, e.g. `BASE TABLE` or `VIEW`.
    pub table_type: String,
    /// The name of the table that the current table is a temporary table for.
    pub self_referencing_column_name: Option<String>,
    /// The name of the column that is a foreign key to the current table.
    pub reference_generation: Option<String>,
    /// The user-defined type catalog.
    pub user_defined_type_catalog: Option<String>,
    /// The user-defined type schema.
    pub user_defined_type_schema: Option<String>,
    /// The user-defined type name.
    pub user_defined_type_name: Option<String>,
    /// The user-defined type name that the current table is a temporary table
    /// for.
    pub is_insertable_into: String,
    /// Whether the table is typed.
    pub is_typed: String,
    /// Whether the table is updatable.
    pub commit_action: Option<String>,
}

impl Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "`{}.{}`", self.table_schema, self.table_name)
    }
}

impl Table {
    /// Initializes and returns the metadata for the table.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * If the metadata cannot be loaded from the database.
    pub fn metadata(
        &self,
        conn: &mut PgConnection,
    ) -> Result<TableMetadata, diesel::result::Error> {
        let mut sql_metadata = sql_traits::structs::TableMetadata::default();
        for column in cached_queries::columns(self, conn)? {
            let column = column::Column::from(column);
            sql_metadata.add_column(std::rc::Rc::new(column));
        }
        for check_constraint in cached_queries::check_constraints(self, conn)? {
            sql_metadata.add_check_constraint(std::rc::Rc::new(check_constraint));
        }
        for foreign_key in cached_queries::foreign_keys(self, conn)? {
            sql_metadata.add_foreign_key(std::rc::Rc::new(foreign_key));
        }
        for index in cached_queries::unique_indices(self, conn)? {
            sql_metadata.add_unique_index(std::rc::Rc::new(index));
        }
        let mut primary_key_columns = Vec::new();
        for pk_column in cached_queries::primary_key_columns(self, conn)? {
            primary_key_columns.extend(
                sql_metadata
                    .column_rcs()
                    .filter(|col: &&std::rc::Rc<Column>| col.as_ref() == &pk_column)
                    .cloned(),
            );
        }
        sql_metadata.set_primary_key(primary_key_columns);

        let metadata = TableMetadata::new(
            sql_metadata,
            cached_queries::pg_description(self, conn).optional()?,
        );

        Ok(metadata)
    }

    #[must_use]
    /// Returns whether the table is temporary.
    pub fn is_temporary(&self) -> bool {
        self.table_type == "LOCAL TEMPORARY" || self.table_type == "GLOBAL TEMPORARY"
    }

    /// Returns the indices for the table.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A vector of indices.
    ///
    /// # Errors
    ///
    /// * If the indices cannot be loaded from the database.
    pub fn indices(&self, conn: &mut PgConnection) -> Result<Vec<PgIndex>, diesel::result::Error> {
        indices(self, conn)
    }

    /// Returns the UNIQUE constraint indices for the table.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A vector of indices.
    ///
    /// # Errors
    ///
    /// * If the indices cannot be loaded from the database.
    pub fn unique_indices(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<PgIndex>, diesel::result::Error> {
        unique_indices(self, conn)
    }

    /// Returns all tables in the database.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    /// * `table_catalog` - The table catalog.
    /// * `table_schema` - The table schema.
    ///
    /// # Returns
    ///
    /// A vector of all tables in the database.
    ///
    /// # Errors
    ///
    /// * If the tables cannot be loaded from the database.
    pub fn load_all(
        conn: &mut PgConnection,
        table_catalog: &str,
        table_schema: &str,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        load_all_tables(table_catalog, table_schema, conn)
    }

    /// Returns the table by name.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    /// * `table_name` - The name of the table.
    /// * `table_schema` - The schema of the table.
    /// * `table_catalog` - The catalog of the table.
    ///
    /// # Returns
    ///
    /// The table.
    ///
    /// # Errors
    ///
    /// * If the table cannot be loaded from the database.
    pub fn load(
        table_name: &str,
        table_schema: &str,
        table_catalog: &str,
        conn: &mut PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        load_table(conn, table_name, table_schema, table_catalog)
    }

    /// Returns the column by name.
    ///
    /// # Arguments
    ///
    /// * `column_name` - The name of the column.
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// The column.
    ///
    /// # Errors
    ///
    /// * If the column cannot be loaded from the database.
    pub fn column_by_name(
        &self,
        column_name: &str,
        conn: &mut PgConnection,
    ) -> Result<Column, diesel::result::Error> {
        column_by_name(self, column_name, conn)
    }

    /// Returns the check constraints for the table.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A vector of check constraints.
    ///
    /// # Errors
    ///
    /// * If the check constraints cannot be loaded from the database.
    pub fn check_constraints(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<CheckConstraint>, diesel::result::Error> {
        check_constraints(self, conn)
    }

    /// Returns the list of Triggers associates to the current table.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A vector of triggers.
    ///
    /// # Errors
    ///
    /// * If the triggers cannot be loaded from the database.
    pub fn triggers(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<PgTrigger>, diesel::result::Error> {
        triggers(self, conn)
    }
}

impl AsRef<Table> for Table {
    fn as_ref(&self) -> &Table {
        self
    }
}
