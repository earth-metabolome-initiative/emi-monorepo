use diesel::{
    pg::PgConnection, BoolExpressionMethods, ExpressionMethods, JoinOnDsl, QueryDsl, Queryable,
    QueryableByName, RunQueryDsl, Selectable, SelectableHelper,
};

use super::Column;
use crate::errors::WebCodeGenError;

/// Represents a row in the `pg_indexes` view
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq)]
#[diesel(table_name = crate::schema::pg_indexes)]
/// Represents a `PostgreSQL` index in the database.
pub struct Indices {
    /// The name of the schema containing the index.
    pub schemaname: String,
    /// The name of the table associated with the index.
    pub tablename: String,
    /// The name of the index.
    pub indexname: String,
    /// The name of the tablespace containing the index.
    pub tablespace: Option<String>,
    /// The definition of the index.
    pub indexdef: String,
}

/// Represents the `pg_index` system catalog table in `PostgreSQL`.
/// This table stores information about indexes on tables.
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq)]
#[diesel(table_name = crate::schema::pg_index)]
#[allow(clippy::struct_excessive_bools)]
pub struct PgIndex {
    /// The OID of the index.
    pub indexrelid: u32,
    /// The OID of the table this index belongs to.
    pub indrelid: u32,
    /// The number of columns in the index (including expression columns).
    pub indnatts: i16,
    /// The number of key columns in the index (excluding non-key expressions).
    pub indnkeyatts: i16,
    /// `true` if the index enforces uniqueness.
    pub indisunique: bool,
    /// `true` if this is the primary key index for the table.
    pub indisprimary: bool,
    /// `true` if this index enforces an exclusion constraint.
    pub indisexclusion: bool,
    /// `true` if this index enforces immediate constraints (not deferrable).
    pub indimmediate: bool,
    /// `true` if the index is the clustering index for the table.
    pub indisclustered: bool,
    /// `true` if the index is valid for use by queries.
    pub indisvalid: bool,
    /// `true` if the index's validity depends on the transaction snapshot.
    pub indcheckxmin: bool,
    /// `true` if the index is ready for inserts and queries.
    pub indisready: bool,
    /// `true` if the index is marked as live (not pending deletion).
    pub indislive: bool,
    /// `true` if the index is the replication identity index.
    pub indisreplident: bool,
    /// The column numbers of the indexed columns (0-based for expressions).
    pub indkey: Vec<i16>,
    /// The collation OIDs for indexed columns (0 if default collation is used).
    pub indcollation: Vec<u32>,
    /// The operator class OIDs for indexed columns.
    pub indclass: Vec<u32>,
    /// Per-column index options.
    pub indoption: Vec<i16>,
}

impl PgIndex {
    /// Returns the columns that are involved in the index
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// If an error occurs while loading the columns from the database
    pub fn columns(&self, conn: &mut PgConnection) -> Result<Vec<Column>, WebCodeGenError> {
        use crate::schema::{columns, pg_attribute, pg_class, pg_index};

        Ok(pg_index::table
            .inner_join(pg_class::table.on(pg_class::oid.eq(pg_index::indrelid)))
            .inner_join(pg_attribute::table.on(pg_attribute::attrelid.eq(pg_class::oid)))
            .inner_join(
                columns::table.on(columns::table_name
                    .eq(pg_class::relname)
                    .and(columns::column_name.eq(pg_attribute::attname))),
            )
            .filter(
                pg_index::indexrelid
                    .eq(self.indexrelid)
                    .and(pg_attribute::attnum.eq_any(&self.indkey)),
            )
            .select(Column::as_select())
            .load::<Column>(conn)?)
    }

    #[must_use]
    /// Returns whether the index is unique
    pub fn is_unique(&self) -> bool {
        self.indisunique
    }
}
