//! Submodule for the `pg_catalog.pg_inherits` table schema.

diesel::table! {
    /// `pg_catalog.pg_inherits` — system catalog recording table inheritance hierarchy.
    /// Each row represents an inheritance relationship between a child table and a parent table.
    pg_catalog.pg_inherits (inhrelid, inhseqno) {
        /// OID of the child table (inheriting relation).
        inhrelid -> Oid,
        /// OID of the parent table.
        inhparent -> Oid,
        /// Sequence number to distinguish multiple parents (1-based).
        inhseqno -> Integer,
        /// `true` if this inheritance relationship is being detached (partition detach in progress).
        inhdetachpending -> Bool,
    }
}
