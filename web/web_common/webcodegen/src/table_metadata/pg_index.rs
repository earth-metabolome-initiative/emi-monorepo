use cached::proc_macro::cached;
use diesel::{
    BoolExpressionMethods, ExpressionMethods, JoinOnDsl, PgConnection, QueryDsl, Queryable,
    QueryableByName, RunQueryDsl, Selectable, SelectableHelper,
};

use super::Column;
use crate::{KeyColumnUsage, Table, errors::WebCodeGenError};

#[cached(
    result = true,
    key = "String",
    convert = r#"{ format!("{}:{}", index.indexrelid, index.indrelid) }"#
)]
fn columns(index: &PgIndex, conn: &mut PgConnection) -> Result<Vec<Column>, diesel::result::Error> {
    use crate::schema::{columns, pg_attribute, pg_class, pg_index};

    pg_index::table
        .inner_join(pg_class::table.on(pg_class::oid.eq(pg_index::indrelid)))
        .inner_join(pg_attribute::table.on(pg_attribute::attrelid.eq(pg_class::oid)))
        .inner_join(
            columns::table.on(columns::table_name
                .eq(pg_class::relname)
                .and(columns::column_name.eq(pg_attribute::attname))),
        )
        .filter(
            pg_index::indexrelid
                .eq(index.indexrelid)
                .and(pg_attribute::attnum.eq_any(&index.indkey)),
        )
        .select(Column::as_select())
        .load::<Column>(conn)
}

#[cached(
    result = true,
    key = "String",
    convert = r#"{ format!("{}:{}", index.indexrelid, index.indrelid) }"#
)]
fn is_same_as_index(
    index: &PgIndex,
    conn: &mut PgConnection,
) -> Result<Option<KeyColumnUsage>, WebCodeGenError> {
    // If the index is not unique, it cannot be a same-as index
    if !index.is_unique() {
        return Ok(None);
    }

    // Next, we retrieve the columns associated with the index.
    let columns = index.columns(conn)?;

    // If the index has only one column, it cannot be a same-as index
    if columns.len() <= 1 {
        return Ok(None);
    }

    // We retrieve the table that this index belongs to.
    let table = index.table(conn)?;

    // We expect that all of the columns in the primary key of the table are also in
    // the index.
    let primary_key_columns = table.primary_key_columns(conn)?;

    // If any of the primary key columns are not in the index, it cannot be a
    // same-as index
    if !primary_key_columns.iter().all(|pk_col| columns.contains(pk_col)) {
        return Ok(None);
    }

    // We check that there are other columns in the index that are not part of the
    // primary key.
    if primary_key_columns.len() == columns.len() {
        // The UNIQUE index is composed only of the primary key columns,
        // so it cannot be a same-as index.
        return Ok(None);
    }

    // There needs to be a foreign key constraint which includes all of the
    // remaining columns in the index which refers to some other table's
    // primary key.
    let mut non_primary_key_columns: Vec<Column> =
        columns.clone().into_iter().filter(|col| !primary_key_columns.contains(col)).collect();

    // We make sure that the columns are sorted, so to be easy to compare them
    // later.
    non_primary_key_columns.sort_unstable();

    // We search for a foreign key constraint that includes all of these columns,
    // and that refers to a primary key of another table. If we find any
    // foreign key which satisfies this condition, then we can conclude that
    // the index is a same-as index.

    let mut foreign_keys = table
        .foreign_keys(conn)?
        .into_iter()
        .filter(|fk| {
            // If the foreign key does not refer to a foreign's table primary key, it cannot
            // be a same-as index.
            if !fk.is_foreign_primary_key(conn).unwrap_or(false) {
                return false;
            }
            // If the foreign key does not include all of the non-primary key columns, it
            // cannot be a same-as index.
            let mut fk_columns = fk.columns(conn).unwrap_or_default();
            fk_columns.sort_unstable();
            non_primary_key_columns == fk_columns
        })
        .collect::<Vec<_>>();

    assert_eq!(
        foreign_keys.len(),
        1,
        "There should be exactly one foreign key that matches the criteria in table `{}.{}.{}` for index with columns [{}]",
        table.table_catalog,
        table.table_schema,
        table.table_name,
        columns
            .iter()
            .map(|c| format!("{}.{}", c.table_name, c.column_name))
            .collect::<Vec<_>>()
            .join(", ")
    );

    Ok(foreign_keys.pop())
}

/// Represents the `pg_index` system catalog table in `PostgreSQL`.
/// This table stores information about indexes on tables.
#[derive(
    Clone, Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Hash, Ord, PartialOrd,
)]
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
    /// Returns the table that this index belongs to.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// If an error occurs while loading the table from the database
    pub fn table(&self, conn: &mut PgConnection) -> Result<Table, diesel::result::Error> {
        use crate::schema::{pg_class, tables};

        pg_class::table
            .inner_join(tables::table.on(tables::table_name.eq(pg_class::relname)))
            .filter(pg_class::oid.eq(self.indrelid))
            .select(Table::as_select())
            .first(conn)
    }

    /// Returns the columns that are involved in the index
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// If an error occurs while loading the columns from the database
    pub fn columns(&self, conn: &mut PgConnection) -> Result<Vec<Column>, diesel::result::Error> {
        columns(self, conn)
    }

    /// Returns whether the unique index is defining a same-as relationship
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    ///
    /// # Implementation details
    ///
    /// An index is considered a "same-as" index if it is unique, is a composite
    /// index (i.e., has more than one column), and it is composed of the
    /// set of columns of the primary key of the table and the set of columns of
    /// the primary key of a foreign table, and all of those columns appear
    /// as foreign key columns in the table where the index is defined.
    pub fn is_same_as(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Option<KeyColumnUsage>, WebCodeGenError> {
        is_same_as_index(self, conn)
    }

    #[must_use]
    /// Returns whether the index is unique
    pub fn is_unique(&self) -> bool {
        self.indisunique
    }

    #[must_use]
    /// Returns whether the index is a primary key
    pub fn is_primary_key(&self) -> bool {
        self.indisprimary
    }
}
