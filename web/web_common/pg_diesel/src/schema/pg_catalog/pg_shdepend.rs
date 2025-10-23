//! Submodule for the `pg_catalog.pg_shdepend` table schema.

diesel::table! {
    /// `pg_catalog.pg_shdepend` — system catalog containing shared dependencies.
    /// Records dependencies between database objects and shared objects like roles.
    /// Has no primary key as it's a dependency tracking table.
    pg_catalog.pg_shdepend (dbid) {
        /// OID of the database (0 for shared objects).
        dbid -> Oid,
        /// OID of the system catalog containing the dependent object.
        classid -> Oid,
        /// OID of the dependent object.
        objid -> Oid,
        /// Sub-object ID (e.g., column number).
        objsubid -> Integer,
        /// OID of the system catalog containing the referenced object.
        refclassid -> Oid,
        /// OID of the referenced shared object.
        refobjid -> Oid,
        /// Dependency type code.
        deptype -> Text,
    }
}
