use diesel::pg::PgConnection;
use diesel::{Queryable, QueryableByName, RunQueryDsl};

/// Represents a row in the `constraint_table_usage` table in the PostgreSQL database.
/// 
/// The `constraint_table_usage` table contains information about table constraints
/// defined in the database, including catalogs, schemas, and names of tables and constraints.
#[derive(Queryable, QueryableByName, Debug)]
#[diesel(table_name = crate::schema::constraint_table_usage)]
pub struct ConstraintTableUsage {
    /// The catalog of the constraint.
    pub constraint_catalog: String,
    /// The schema of the constraint.
    pub constraint_schema: String,
    /// The name of the constraint.
    pub constraint_name: String,
    /// The catalog of the table.
    pub table_catalog: String,
    /// The schema of the table.
    pub table_schema: String,
    /// The name of the table.
    pub table_name: String,
}

impl ConstraintTableUsage {
    /// Loads all rows from the `constraint_table_usage` table.
    ///
    /// # Arguments
    ///
    /// * `conn` - A connection to the PostgreSQL database.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `ConstraintTableUsage` on success,
    /// or a `diesel::result::Error` on failure.
    pub fn load_all(conn: &mut PgConnection) -> Result<Vec<Self>, diesel::result::Error> {
        use crate::schema::constraint_table_usage;
        constraint_table_usage::table.load::<ConstraintTableUsage>(conn)
    }
}
