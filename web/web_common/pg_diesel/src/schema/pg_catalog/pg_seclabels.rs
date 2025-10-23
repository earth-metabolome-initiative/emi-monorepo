//! Submodule for the `pg_catalog.pg_seclabels` view schema.

diesel::table! {
    /// `pg_catalog.pg_seclabels` — view showing security labels in a user-friendly format.
    /// Each row represents a security label with human-readable object identification.
    /// Uses `objoid` as a nominal primary key for Diesel compatibility.
    pg_catalog.pg_seclabels (objoid) {
        /// OID of the object.
        objoid -> Nullable<Oid>,
        /// OID of the system catalog containing the object.
        classoid -> Nullable<Oid>,
        /// Sub-object ID.
        objsubid -> Nullable<Integer>,
        /// Type of object (e.g., 'table', 'column', 'function').
        objtype -> Nullable<Text>,
        /// OID of the namespace containing the object (null for some object types).
        objnamespace -> Nullable<Oid>,
        /// Name of the object.
        objname -> Nullable<Text>,
        /// Name of the security label provider.
        provider -> Nullable<Text>,
        /// Security label text.
        label -> Nullable<Text>,
    }
}
