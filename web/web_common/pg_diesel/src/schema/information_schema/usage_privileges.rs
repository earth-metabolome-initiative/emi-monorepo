//! `usage_privileges` view from `information_schema`.

diesel::table! {
    /// `information_schema.usage_privileges` — view containing one row for each
    /// USAGE privilege granted on a schema, domain, collation, character set,
    /// or translation to a currently enabled role.
    information_schema.usage_privileges (grantor, grantee, object_catalog, object_schema, object_name, object_type, privilege_type) {
        /// Name of the role that granted the privilege.
        grantor -> Nullable<Text>,
        /// Name of the role that was granted the privilege.
        grantee -> Nullable<Text>,
        /// Name of the database (catalog) containing the object.
        object_catalog -> Nullable<Text>,
        /// Name of the schema containing the object.
        object_schema -> Nullable<Text>,
        /// Name of the object.
        object_name -> Nullable<Text>,
        /// Type of object (SCHEMA, DOMAIN, COLLATION, CHARACTER SET, TRANSLATION).
        object_type -> Nullable<Text>,
        /// Type of privilege (always USAGE for this view).
        privilege_type -> Nullable<Text>,
        /// "YES" if the privilege is grantable to others, "NO" otherwise.
        is_grantable -> Nullable<Text>,
    }
}
