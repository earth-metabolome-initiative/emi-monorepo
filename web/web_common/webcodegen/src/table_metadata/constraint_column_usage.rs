use diesel::{ExpressionMethods, PgConnection, QueryDsl, Queryable, QueryableByName, RunQueryDsl};

use crate::errors::WebCodeGenError;

/// Represents a row in the `constraint_column_usage` table in `PostgreSQL`
///
/// The `constraint_column_usage` table contains information about columns that
/// are used in constraints.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/infoschema-constraint-column-usage.html)
#[derive(Queryable, QueryableByName, Debug)]
#[diesel(table_name = crate::schema::constraint_column_usage)]
pub struct ConstraintColumnUsage {
    /// The name of the database containing the constraint
    pub constraint_catalog: String,
    /// The name of the schema containing the constraint
    pub constraint_schema: String,
    /// The name of the constraint
    pub constraint_name: String,
    /// The name of the database containing the table with the column
    pub table_catalog: String,
    /// The name of the schema containing the table with the column
    pub table_schema: String,
    /// The name of the table containing the column
    pub table_name: String,
    /// The name of the column used in the constraint
    pub column_name: String,
}

impl ConstraintColumnUsage {
    /// Load all the constraint column usages from the database
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Returns
    ///
    /// A `Result` containing a `Vec` of `ConstraintColumnUsage` if the
    /// operation was successful, or a `WebCodeGenError` if an error occurred
    ///
    /// # Errors
    ///
    /// If an error occurs while loading the column usages from the database
    pub fn load_all_constraint_column_usages(
        conn: &mut PgConnection,
    ) -> Result<Vec<Self>, WebCodeGenError> {
        use crate::schema::constraint_column_usage;
        constraint_column_usage::table
            .load::<ConstraintColumnUsage>(conn)
            .map_err(WebCodeGenError::from)
    }

    /// Load all the constraint column usages from the database
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    /// * `constraint_name` - The name of the constraint to load the column
    ///   usages for
    /// * `constraint_schema` - An optional schema name to filter the column
    ///   usages by
    /// * `constraint_catalog` - The name of the catalog to filter the column
    ///   usages by
    ///
    /// # Returns
    ///
    /// A `Result` containing a `Vec` of `ConstraintColumnUsage` if the
    /// operation was successful, or a `WebCodeGenError` if an error occurred
    ///
    /// # Errors
    ///
    /// If an error occurs while loading the column usages from the database
    pub fn load_constraint_column_usages(
        conn: &mut PgConnection,
        constraint_name: &str,
        constraint_schema: Option<&str>,
        constraint_catalog: &str,
    ) -> Result<Vec<Self>, WebCodeGenError> {
        use crate::schema::constraint_column_usage;
        let constraint_schema = constraint_schema.unwrap_or("public");
        constraint_column_usage::table
            .filter(constraint_column_usage::constraint_name.eq(constraint_name))
            .filter(constraint_column_usage::constraint_schema.eq(constraint_schema))
            .filter(constraint_column_usage::constraint_catalog.eq(constraint_catalog))
            .load::<ConstraintColumnUsage>(conn)
            .map_err(WebCodeGenError::from)
    }
}
