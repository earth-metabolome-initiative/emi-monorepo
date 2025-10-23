use diesel::{Queryable, QueryableByName};

/// Represents a row in the `constraint_table_usage` table in the `PostgreSQL`
/// database.
///
/// The `constraint_table_usage` table contains information about table
/// constraints defined in the database, including catalogs, schemas, and names
/// of tables and constraints.
#[derive(Queryable, QueryableByName, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::information_schema::constraint_table_usage::constraint_table_usage)]
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
