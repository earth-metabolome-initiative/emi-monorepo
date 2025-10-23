//! Submodule providing the `PgPublicationTable` struct representing a row of
//! the `pg_publication_tables` view in `PostgreSQL`.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a row from the `pg_publication_tables` view.
///
/// The `pg_publication_tables` view provides a user-friendly representation of
/// which tables are included in which publications, along with column lists and
/// row filters if applicable.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/view-pg-publication-tables.html).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_publication_tables::pg_publication_tables)]
pub struct PgPublicationTable {
    /// Publication name.
    pub pubname: Option<String>,
    /// Schema name.
    pub schemaname: Option<String>,
    /// Table name.
    pub tablename: Option<String>,
    /// Column names included.
    pub attnames: Option<Vec<String>>,
    /// Row filter expression.
    pub rowfilter: Option<String>,
}
