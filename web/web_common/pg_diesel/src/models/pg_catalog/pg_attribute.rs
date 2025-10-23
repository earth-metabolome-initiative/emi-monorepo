//! Submodule providing the struct `PgAttribute` and associated methods.
use diesel::{PgConnection, Queryable, QueryableByName, Selectable};

use crate::models::PgType;

mod cached_queries;

/// Represents a `PostgreSQL` attribute (column) in a table.
///
/// This struct maps to the `pg_attribute` system catalog table in `PostgreSQL`,
/// which stores metadata about table columns. Each instance of `PgAttribute`
/// corresponds to a single column in a table.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/catalog-pg-attribute.html).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Hash, Clone)]
#[diesel(table_name = crate::schema::pg_catalog::pg_attribute::pg_attribute)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(clippy::struct_excessive_bools)]
pub struct PgAttribute {
    /// The OID of the table this column belongs to.
    pub attrelid: u32,
    /// The name of the column.
    pub attname: String,
    /// The OID of the data type of the column.
    pub atttypid: u32,
    /// The length of the column (if fixed), otherwise -1.
    pub attlen: i16,
    /// The number of the column (its position in the table, starting at 1).
    pub attnum: i16,
    /// The offset of the column in the tuple (if cached), otherwise -1.
    pub attcacheoff: i32,
    /// Type-specific data about the column (e.g., precision for numeric types).
    pub atttypmod: i32,
    /// The number of dimensions if the column is an array type.
    pub attndims: i16,
    /// Whether the column is passed by value.
    pub attbyval: bool,
    /// The alignment requirement of the column.
    pub attalign: String,
    /// The storage strategy for the column.
    pub attstorage: String,
    /// Whether the column is marked as NOT NULL.
    pub attnotnull: bool,
    /// Whether the column has a default value.
    pub atthasdef: bool,
    /// Whether the column has a missing value.
    pub atthasmissing: bool,
    /// The identity type of the column (e.g., 'a' for always, 'd' for by
    /// default).
    pub attidentity: String,
    /// The generation expression for the column (if any).
    pub attgenerated: String,
    /// Whether the column has been dropped.
    pub attisdropped: bool,
    /// Whether the column is a local definition.
    pub attislocal: bool,
    /// The number of times the column is inherited.
    pub attinhcount: i16,
    /// The collation of the column.
    pub attcollation: u32,
    /// The statistics target for the column (if any).
    pub attstattarget: Option<i16>,
    /// The access control list for the column (if any).
    pub attacl: Option<Vec<String>>,
    /// The column options (if any).
    pub attoptions: Option<Vec<String>>,
    /// The foreign data wrapper options for the column (if any).
    pub attfdwoptions: Option<Vec<String>>,
    /// Compression method for the column
    pub attcompression: String,
}

impl PgAttribute {
    /// Returns the [`PgType`] associated to the `PgAttribute`.
    ///
    /// # Errors
    ///
    /// Returns an error if the provided database connection fails.
    pub fn pg_type(&self, conn: &mut PgConnection) -> Result<PgType, diesel::result::Error> {
        cached_queries::pg_type(self, conn)
    }
}
