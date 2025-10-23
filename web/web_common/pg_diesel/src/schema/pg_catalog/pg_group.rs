//! Submodule for the `pg_catalog.pg_group` view schema.

diesel::table! {
    /// `pg_catalog.pg_group` — view providing access to information about database roles
    /// that are marked as groups (for backwards compatibility with PostgreSQL versions
    /// before 8.1). This is a deprecated view; use pg_roles or pg_authid instead.
    pg_catalog.pg_group (groname) {
        /// Name of the group role.
        groname -> Nullable<Text>,
        /// OID of the group role.
        grosysid -> Nullable<Oid>,
        /// Array of OIDs of roles that are members of this group.
        grolist -> Nullable<Array<Oid>>,
    }
}
