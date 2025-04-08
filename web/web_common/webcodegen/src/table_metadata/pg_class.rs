//! Submodule providing the struct `PgClass` and associated methods.

use diesel::{Queryable, QueryableByName, Selectable};

/// Represents a `PostgreSQL` class (table, index, sequence, etc.).
///
/// This struct maps to the `pg_class` system catalog table in `PostgreSQL`,
/// which stores metadata about tables, indexes, sequences, and other similar
/// objects. Each instance of `PGClass` corresponds to a single object in the
/// database.
///
/// For more information, see the [PostgreSQL documentation](https://www.postgresql.org/docs/current/catalog-pg-class.html).
#[derive(Queryable, QueryableByName, Selectable, Debug, PartialEq)]
#[diesel(table_name = crate::schema::pg_class)]
#[allow(clippy::struct_excessive_bools)]
pub struct PGClass {
    /// The OID of the object.
    pub oid: u32,
    /// The name of the object.
    pub relname: String,
    /// The OID of the namespace that contains this object.
    pub relnamespace: u32,
    /// The OID of the data type of the object.
    pub reltype: u32,
    /// The OID of the data type of the table's row type, if any.
    pub reloftype: u32,
    /// The OID of the owner of the object.
    pub relowner: u32,
    /// The access method (index method) used for this object.
    pub relam: u32,
    /// The file node number of the object.
    pub relfilenode: u32,
    /// The tablespace of the object.
    pub reltablespace: u32,
    /// The number of pages in the object.
    pub relpages: i32,
    /// The number of tuples in the object.
    pub reltuples: f32,
    /// The number of all-visible pages in the object.
    pub relallvisible: i32,
    /// The OID of the TOAST table associated with this object, if any.
    pub reltoastrelid: u32,
    /// Whether the object has indexes.
    pub relhasindex: bool,
    /// Whether the object is shared across all databases.
    pub relisshared: bool,
    /// The persistence type of the object (e.g., permanent, unlogged, or
    /// temporary).
    pub relpersistence: String,
    /// The kind of object (e.g., table, index, sequence).
    pub relkind: String,
    /// The number of attributes (columns) in the object.
    pub relnatts: i16,
    /// The number of check constraints on the object.
    pub relchecks: i16,
    /// Whether the object has rules.
    pub relhasrules: bool,
    /// Whether the object has triggers.
    pub relhastriggers: bool,
    /// Whether the object has subclasses (inheritance).
    pub relhassubclass: bool,
    /// Whether the object has row-level security enabled.
    pub relrowsecurity: bool,
    /// Whether the object has row-level security forced.
    pub relforcerowsecurity: bool,
    /// Whether the object is populated.
    pub relispopulated: bool,
    /// The replication identity of the object.
    pub relreplident: String,
    /// Whether the object is a partition.
    pub relispartition: bool,
    /// The OID of the rewrite rule associated with this object, if any.
    pub relrewrite: u32,
}
