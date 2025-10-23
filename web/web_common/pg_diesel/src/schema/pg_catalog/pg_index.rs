//! Submodule for the `pg_catalog.pg_index` table schema.

diesel::table! {
    /// `pg_index` — system catalog containing one row per index in the database.
    /// Provides low-level metadata about index properties, columns, uniqueness,
    /// and internal system behavior.
    pg_catalog.pg_index (indexrelid) {
        /// OID of the index itself.
        indexrelid -> Oid,
        /// OID of the table the index is defined on.
        indrelid -> Oid,
        /// Total number of columns in the index.
        indnatts -> SmallInt,
        /// Number of key columns used for uniqueness checks (excluding included columns in some versions).
        indnkeyatts -> SmallInt,
        /// `true` if the index enforces uniqueness.
        indisunique -> Bool,
        /// `true` if nulls are considered distinct in a unique index.
        indnullsnotdistinct -> Bool,
        /// `true` if the index is a primary key.
        indisprimary -> Bool,
        /// `true` if the index is an exclusion constraint index.
        indisexclusion -> Bool,
        /// `true` if the index is immediate (not deferred).
        indimmediate -> Bool,
        /// `true` if the index is clustered on the table.
        indisclustered -> Bool,
        /// `true` if the index is valid (can be used by queries).
        indisvalid -> Bool,
        /// `true` if the index is protected against transaction wraparound issues.
        indcheckxmin -> Bool,
        /// `true` if the index is ready for inserts/updates.
        indisready -> Bool,
        /// `true` if the index is live and actively maintained by the system.
        indislive -> Bool,
        /// `true` if the index is the replication identity of the table.
        indisreplident -> Bool,
        /// Array of attribute numbers of table columns included in the index.
        indkey -> Array<SmallInt>,
        /// Array of OIDs for column collations for the index columns.
        indcollation -> Array<Oid>,
        /// Array of operator class OIDs for the index columns.
        indclass -> Array<Oid>,
        /// Array of index options (per-column flags such as DESC, NULLS FIRST, etc.).
        indoption -> Array<SmallInt>,
        /// Expression tree for the index (if it's an expression index), stored as a node tree string.
        indexprs -> Nullable<Text>,
        /// Partial index predicate (if it's a partial index), stored as a node tree string.
        indpred -> Nullable<Text>,
    }
}

use super::pg_attribute::pg_attribute;
diesel::allow_tables_to_appear_in_same_query!(pg_index, pg_attribute);
