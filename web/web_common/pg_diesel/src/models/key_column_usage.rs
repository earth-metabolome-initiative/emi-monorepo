use std::fmt::Display;

use diesel::{PgConnection, Queryable, QueryableByName, Selectable};

use crate::models::{Column, PgIndex, ReferentialConstraint, Table};

mod cached_queries;

/// Represents a row in the `key_column_usage` table, which contains information
/// about columns that are constrained by a unique or primary key constraint.
///
/// For more details, see [`PostgreSQL`](https://www.postgresql.org/docs/current/infoschema-key-column-usage.html)
#[derive(
    Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Ord, PartialOrd, Clone, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::key_column_usage)]
pub struct KeyColumnUsage {
    /// The name of the database that contains the constraint.
    pub constraint_catalog: String,
    /// The name of the schema that contains the constraint.
    pub constraint_schema: String,
    /// The name of the constraint.
    pub constraint_name: String,
    /// The name of the database that contains the table.
    pub table_catalog: String,
    /// The name of the schema that contains the table.
    pub table_schema: String,
    /// The name of the table that contains the column.
    pub table_name: String,
    /// The name of the column that is constrained.
    pub column_name: String,
    /// The position of the column within the constraint.
    pub ordinal_position: i32,
    /// The position of the column within the unique constraint, if applicable.
    pub position_in_unique_constraint: Option<i32>,
}

impl Display for KeyColumnUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}.{}({})",
            self.table_schema, self.table_name, self.column_name, self.constraint_name
        )
    }
}

impl KeyColumnUsage {
    /// Returns the SQL definition of the key column usage as a foreign key
    /// constraint.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn to_sql(&self, conn: &mut PgConnection) -> Result<String, diesel::result::Error> {
        let local_columns = self.local_columns(conn)?;
        let foreign_table = self.foreign_table(conn)?;
        let foreign_columns = self.foreign_columns(conn)?;

        let local_column_names: Vec<String> =
            local_columns.iter().map(|col| col.column_name.clone()).collect();
        let foreign_column_names: Vec<String> =
            foreign_columns.iter().map(|col| col.column_name.clone()).collect();

        let on_delete_clause =
            if self.has_on_delete_cascade(conn)? { " ON DELETE CASCADE" } else { "" };

        Ok(format!(
            "FOREIGN KEY ({}) REFERENCES {}.{}({}){}",
            local_column_names.join(", "),
            foreign_table.table_schema,
            foreign_table.table_name,
            foreign_column_names.join(", "),
            on_delete_clause
        ))
    }

    /// Returns whether the key is on delete cascade
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn has_on_delete_cascade(
        &self,
        conn: &mut PgConnection,
    ) -> Result<bool, diesel::result::Error> {
        let referential_constraint = self.referential_constraint(conn)?;
        Ok(referential_constraint.delete_rule == "CASCADE")
    }

    /// Returns the referential constraint associated with this key column usage
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while loading the referential constraint from the
    ///   database
    pub(crate) fn referential_constraint(
        &self,
        conn: &mut PgConnection,
    ) -> Result<ReferentialConstraint, diesel::result::Error> {
        cached_queries::referential_constraint(self, conn)
    }

    /// Returns the table associated with this key column usage
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while loading the table from the database
    pub fn local_table(&self, conn: &mut PgConnection) -> Result<Table, diesel::result::Error> {
        cached_queries::local_table(self, conn)
    }

    /// Returns whether the key column usage is self-referential, i.e. the
    /// foreign table is the same as the local table
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    pub fn is_self_referential(
        &self,
        conn: &mut PgConnection,
    ) -> Result<bool, diesel::result::Error> {
        Ok(self.local_table(conn)? == self.foreign_table(conn)?)
    }

    /// Returns the foreign table associated with this key column usage
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while loading the foreign table from the database
    pub fn foreign_table(&self, conn: &mut PgConnection) -> Result<Table, diesel::result::Error> {
        cached_queries::foreign_table(self, conn)
    }

    /// Returns all the local columns involved in the constraint
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while loading the key column usages from the
    ///   database
    pub fn local_columns(&self, conn: &mut PgConnection) -> Result<Vec<Column>, diesel::result::Error> {
        cached_queries::local_columns(self, conn)
    }

    /// Returns the column mapping between the local and foreign columns
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while loading the column mappings from the database
    pub fn column_mappings(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<(Column, Column)>, diesel::result::Error> {
        Ok(self
            .local_columns(conn)?
            .iter()
            .cloned()
            .zip(self.foreign_columns(conn)?.iter().cloned())
            .collect())
    }

    /// Returns whether it is a composite key column usage
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while loading the key column usages from the
    ///   database
    pub fn is_composite(&self, conn: &mut PgConnection) -> Result<bool, diesel::result::Error> {
        self.local_columns(conn).map(|columns| columns.len() > 1)
    }

    /// Returns whether any column involved in the constraint is nullable
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while loading the key column usages from the
    ///   database
    pub fn is_nullable(&self, conn: &mut PgConnection) -> Result<bool, diesel::result::Error> {
        self.local_columns(conn).map(|columns| columns.iter().any(Column::is_nullable))
    }

    /// Returns the columns in the foreign table that are referenced by this key
    /// column usage
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while loading the foreign key column usages from
    ///   the database
    pub fn foreign_columns(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<Column>, diesel::result::Error> {
        cached_queries::foreign_columns(self, conn)
    }

    /// Returns whether the key column usage refers to a foreign primary key
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn is_foreign_primary_key(
        &self,
        conn: &mut PgConnection,
    ) -> Result<bool, diesel::result::Error> {
        let foreign_table = self.foreign_table(conn)?;

        // Check if the foreign table has a primary key
        let primary_keys = foreign_table.primary_key_columns(conn)?;
        let foreign_columns = self.foreign_columns(conn)?;
        Ok(primary_keys == foreign_columns)
    }

    /// Returns whether the key column usage refers to a local primary key
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn is_local_primary_key(
        &self,
        conn: &mut PgConnection,
    ) -> Result<bool, diesel::result::Error> {
        let table = self.local_table(conn)?;

        // Check if the table has a primary key
        let primary_keys = table.primary_key_columns(conn)?;
        let columns = self.local_columns(conn)?;
        Ok(primary_keys == columns)
    }

    /// Returns whether the key column usage includes the local primary key
    /// columns
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn includes_local_primary_key(
        &self,
        conn: &mut PgConnection,
    ) -> Result<bool, diesel::result::Error> {
        let local_table = self.local_table(conn)?;
        let primary_keys = local_table.primary_key_columns(conn)?;
        let columns = self.local_columns(conn)?;
        Ok(primary_keys.iter().all(|pk| columns.contains(pk)))
    }

    /// Returns whether the key column usage includes the foreign primary key
    /// columns
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn includes_foreign_primary_key(
        &self,
        conn: &mut PgConnection,
    ) -> Result<bool, diesel::result::Error> {
        let foreign_table = self.foreign_table(conn)?;
        let primary_keys = foreign_table.primary_key_columns(conn)?;
        let foreign_columns = self.foreign_columns(conn)?;
        Ok(primary_keys.iter().all(|pk| foreign_columns.contains(pk)))
    }

    /// Returns whether the key column usage refers to a foreign unique key
    /// constraint
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn is_foreign_unique_key(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Option<PgIndex>, diesel::result::Error> {
        let foreign_table = self.foreign_table(conn)?;
        let foreign_columns = self.foreign_columns(conn)?;

        // Check if the foreign table has a unique key constraint
        for unique_constraint in foreign_table.unique_indices(conn)? {
            // Check if the foreign columns match the unique constraint columns
            let unique_columns = unique_constraint.columns(conn)?;
            if unique_columns.len() == foreign_columns.len()
                && unique_columns.iter().all(|c| foreign_columns.contains(c))
            {
                return Ok(Some(unique_constraint));
            }
        }
        Ok(None)
    }
}
