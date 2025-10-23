//! Submodule for the `pg_catalog.pg_opfamily` table schema.

diesel::table! {
    /// `pg_catalog.pg_opfamily` — system catalog containing operator families.
    /// An operator family groups related operator classes for an access method.
    pg_catalog.pg_opfamily (oid) {
        /// OID of the operator family.
        oid -> Oid,
        /// OID of the index access method this operator family is for.
        opfmethod -> Oid,
        /// Name of the operator family.
        opfname -> Text,
        /// OID of the namespace containing this operator family.
        opfnamespace -> Oid,
        /// OID of the owner of the operator family.
        opfowner -> Oid,
    }
}
