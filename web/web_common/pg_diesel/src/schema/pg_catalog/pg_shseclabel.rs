//! Submodule for the `pg_catalog.pg_shseclabel` table schema.

diesel::table! {
    /// `pg_catalog.pg_shseclabel` — shared security labels on database objects.
    /// Each row represents a security label assigned to a shared database object.
    /// Composite primary key: (objoid, classoid, provider).
    pg_catalog.pg_shseclabel (objoid, classoid, provider) {
        /// OID of the object this security label pertains to.
        objoid -> Oid,
        /// OID of the system catalog this object appears in.
        classoid -> Oid,
        /// Label provider associated with this label.
        provider -> Text,
        /// Security label applied to this object.
        label -> Text,
    }
}
