use diesel::{
    ExpressionMethods, QueryDsl, Queryable, QueryableByName, Selectable,
};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::errors::WebCodeGenError;

/// Represents a row in the `key_column_usage` table, which contains information
/// about columns that are constrained by a unique or primary key constraint.
///
/// For more details, see [`PostgreSQL`](https://www.postgresql.org/docs/current/infoschema-key-column-usage.html)
#[derive(Queryable, QueryableByName, Selectable, Debug)]
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

impl KeyColumnUsage {
    /// Load all the key column usages from the database
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Returns
    ///
    /// A `Result` containing a `Vec` of `KeyColumnUsage` if the operation was
    /// successful, or a `WebCodeGenError` if an error occurred
    ///
    /// # Errors
    /// If an error occurs while loading the key column usages from the database
    pub async fn load_all_key_column_usages(
        conn: &mut AsyncPgConnection,
    ) -> Result<Vec<Self>, WebCodeGenError> {
        use crate::schema::key_column_usage;
        key_column_usage::table.load::<KeyColumnUsage>(conn).await.map_err(WebCodeGenError::from)
    }

    /// Load all the key column usages from the database
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    /// * `table_name` - The name of the table to load the key column usages for
    /// * `table_schema` - An optional schema name to filter the key column
    ///   usages by
    /// * `table_catalog` - The name of the catalog to filter the key column
    ///   usages by
    ///
    /// # Returns
    ///
    /// A `Result` containing a `Vec` of `KeyColumnUsage` if the operation was
    /// successful, or a `WebCodeGenError` if an error occurred
    ///
    /// # Errors
    ///
    /// If an error occurs while loading the key column usages from the database
    pub async fn load_key_column_usages(
        conn: &mut AsyncPgConnection,
        table_name: &str,
        table_schema: Option<&str>,
        table_catalog: &str,
    ) -> Result<Vec<Self>, WebCodeGenError> {
        use crate::schema::key_column_usage;
        let table_schema = table_schema.unwrap_or("public");
        key_column_usage::table
            .filter(key_column_usage::table_name.eq(table_name))
            .filter(key_column_usage::table_schema.eq(table_schema))
            .filter(key_column_usage::table_catalog.eq(table_catalog))
            .load::<KeyColumnUsage>(conn).await
            .map_err(WebCodeGenError::from)
    }
}
