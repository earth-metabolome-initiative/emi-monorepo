//! Submodule for the `information_schema.role_udt_grants` view schema.

diesel::table! {
    /// `information_schema.role_udt_grants` — view containing one row for each
    /// user-defined type privilege granted to or by a role in the current database.
    information_schema.role_udt_grants (grantor, grantee, udt_catalog, udt_schema, udt_name, privilege_type) {
        /// Role that granted the privilege.
        grantor -> Nullable<Text>,
        /// Role that received the privilege.
        grantee -> Nullable<Text>,
        /// Catalog (database) containing the user-defined type.
        udt_catalog -> Nullable<Text>,
        /// Schema containing the user-defined type.
        udt_schema -> Nullable<Text>,
        /// Name of the user-defined type.
        udt_name -> Nullable<Text>,
        /// Type of privilege granted.
        privilege_type -> Nullable<Text>,
        /// Whether the privilege is grantable.
        is_grantable -> Nullable<Text>,
    }
}
