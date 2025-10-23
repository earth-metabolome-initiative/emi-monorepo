//! Submodule for the `pg_catalog.pg_depend` table schema.

diesel::table! {
    /// `pg_catalog.pg_depend` — system catalog containing one row per dependency between database objects.
    /// Tracks how objects depend on each other, e.g., table columns depending on types, or views depending on tables.
    pg_catalog.pg_depend (classid, objid, objsubid) {
        /// OID of the system catalog containing the dependent object (e.g., `pg_class`, `pg_proc`).
        classid -> Oid,
        /// OID of the dependent object.
        objid -> Oid,
        /// Sub-object ID within the object (e.g., column number for table columns), 0 if not applicable.
        objsubid -> Integer,
        /// OID of the system catalog containing the referenced object.
        refclassid -> Oid,
        /// OID of the referenced object.
        refobjid -> Oid,
        /// Sub-object ID within the referenced object, 0 if not applicable.
        refobjsubid -> Integer,
        /// Dependency type:
        /// 'n' = normal, 'a' = automatic, 'i' = internal, 'e' = extension, 'p' = pinned.
        deptype -> Char,
    }
}

use super::pg_extension::pg_extension;
diesel::allow_tables_to_appear_in_same_query!(pg_depend, pg_extension);

use super::pg_type::pg_type;
diesel::allow_tables_to_appear_in_same_query!(pg_depend, pg_type);

use super::pg_proc::pg_proc;
diesel::allow_tables_to_appear_in_same_query!(pg_depend, pg_proc);
