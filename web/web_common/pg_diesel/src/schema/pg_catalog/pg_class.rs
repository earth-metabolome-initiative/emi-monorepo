//! Submodule for the `pg_catalog.pg_class` table schema.

diesel::table! {
    /// `pg_class` — system catalog containing one row for each table, index, sequence, view,
    /// materialized view, composite type, and TOAST table.
    /// Stores metadata such as storage, persistence, row count estimates, and relation type.
    pg_catalog.pg_class (oid, relname, relnamespace) {
        /// OID of the relation.
        oid -> Oid,
        /// Name of the relation (table, index, etc.).
        relname -> Text,
        /// OID of the namespace (schema) containing this relation.
        relnamespace -> Oid,
        /// OID of the composite type that represents the row type for this relation,
        /// or 0 if none.
        reltype -> Oid,
        /// OID of the underlying type if this is a typed table, else 0.
        reloftype -> Oid,
        /// OID of the role that owns the relation.
        relowner -> Oid,
        /// OID of the access method used (e.g., heap, B-tree, hash).
        relam -> Oid,
        /// Identifier of the relation's physical storage file (or 0 if none, such as for views).
        relfilenode -> Oid,
        /// OID of the tablespace containing the relation, or 0 for the database's default tablespace.
        reltablespace -> Oid,
        /// Number of disk pages the relation uses (as of last `ANALYZE`).
        relpages -> Integer,
        /// Estimated number of live rows in the relation (as of last `ANALYZE`).
        reltuples -> Float,
        /// Number of all-visible pages (from visibility map).
        relallvisible -> Integer,
        /// OID of the TOAST table associated with this relation, or 0 if none.
        reltoastrelid -> Oid,
        /// `true` if the relation has (or once had) an index.
        relhasindex -> Bool,
        /// `true` if the relation is shared across all databases (e.g., `pg_database`).
        relisshared -> Bool,
        /// Relation persistence:
        /// 'p' = permanent,
        /// 'u' = unlogged,
        /// 't' = temporary.
        relpersistence -> Char,
        /// Relation kind:
        /// 'r' = ordinary table,
        /// 'i' = index,
        /// 'S' = sequence,
        /// 't' = TOAST table,
        /// 'v' = view,
        /// 'm' = materialized view,
        /// 'c' = composite type,
        /// 'f' = foreign table,
        /// 'p' = partitioned table,
        /// 'I' = partitioned index.
        relkind -> Char,
        /// Number of user columns (attributes) in the relation.
        relnatts -> SmallInt,
        /// Number of check constraints on the relation.
        relchecks -> SmallInt,
        /// `true` if the relation has rules.
        relhasrules -> Bool,
        /// `true` if the relation has triggers.
        relhastriggers -> Bool,
        /// `true` if the relation has child tables (is a parent in inheritance).
        relhassubclass -> Bool,
        /// `true` if row-level security (RLS) is enabled.
        relrowsecurity -> Bool,
        /// `true` if row-level security (RLS) is forced for all users.
        relforcerowsecurity -> Bool,
        /// `true` if the table is currently populated (for materialized views).
        relispopulated -> Bool,
        /// Replica identity setting:
        /// 'd' = default (primary key),
        /// 'n' = nothing,
        /// 'f' = all columns,
        /// 'i' = index.
        relreplident -> Char,
        /// `true` if this relation is a partition of another table.
        relispartition -> Bool,
        /// OID of the relation this one is being rewritten into (during operations like `ALTER TABLE`), or 0.
        relrewrite -> Oid,
        /// Minimum frozen transaction ID for the relation (used by VACUUM).
        relfrozenxid -> Oid,
        /// Minimum frozen multixact ID for the relation (used by VACUUM).
        relminmxid -> Oid,
        /// Access privileges (ACL) for the relation.
        relacl -> Nullable<Array<Text>>,
        /// Relation-level options for the relation.
        reloptions -> Nullable<Array<Text>>,
        /// Partition bound for partitioned tables.
        relpartbound -> Nullable<Text>,
    }
}

use super::pg_namespace::pg_namespace;
diesel::allow_tables_to_appear_in_same_query!(pg_class, pg_namespace);

use super::pg_index::pg_index;
diesel::allow_tables_to_appear_in_same_query!(pg_class, pg_index);

use super::pg_trigger::pg_trigger;
diesel::allow_tables_to_appear_in_same_query!(pg_class, pg_trigger);

use crate::schema::pg_catalog::pg_description::pg_description;
diesel::allow_tables_to_appear_in_same_query!(pg_class, pg_description);
