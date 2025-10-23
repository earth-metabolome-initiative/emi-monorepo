//! Submodule for the `information_schema.role_usage_grants` view schema.

diesel::table! {
    /// `information_schema.role_usage_grants` — view containing one row for each
    /// usage privilege granted to or by a role in the current database.
    information_schema.role_usage_grants (grantor, grantee, object_catalog, object_schema, object_name, object_type, privilege_type) {
        /// Role that granted the privilege.
        grantor -> Nullable<Text>,
        /// Role that received the privilege.
        grantee -> Nullable<Text>,
        /// Catalog (database) containing the object.
        object_catalog -> Nullable<Text>,
        /// Schema containing the object.
        object_schema -> Nullable<Text>,
        /// Name of the object.
        object_name -> Nullable<Text>,
        /// Type of the object.
        object_type -> Nullable<Text>,
        /// Type of privilege granted.
        privilege_type -> Nullable<Text>,
        /// Whether the privilege is grantable.
        is_grantable -> Nullable<Text>,
    }
}
