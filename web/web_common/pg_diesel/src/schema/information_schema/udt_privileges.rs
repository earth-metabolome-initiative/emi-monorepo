//! `udt_privileges` view from `information_schema`.

diesel::table! {
    /// `information_schema.udt_privileges` — view containing one row for each
    /// privilege granted on a user-defined type to a currently enabled role
    /// or granted by a currently enabled role.
    information_schema.udt_privileges (grantor, grantee, udt_catalog, udt_schema, udt_name, privilege_type) {
        /// Name of the role that granted the privilege.
        grantor -> Nullable<Text>,
        /// Name of the role that was granted the privilege.
        grantee -> Nullable<Text>,
        /// Name of the database (catalog) containing the user-defined type.
        udt_catalog -> Nullable<Text>,
        /// Name of the schema containing the user-defined type.
        udt_schema -> Nullable<Text>,
        /// Name of the user-defined type.
        udt_name -> Nullable<Text>,
        /// Type of privilege (USAGE, etc.).
        privilege_type -> Nullable<Text>,
        /// "YES" if the privilege is grantable to others, "NO" otherwise.
        is_grantable -> Nullable<Text>,
    }
}
