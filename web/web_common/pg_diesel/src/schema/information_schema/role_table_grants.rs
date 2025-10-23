//! Submodule for the `information_schema.role_table_grants` view schema.

diesel::table! {
    /// `information_schema.role_table_grants` — view containing one row for each
    /// table privilege granted to or by a role in the current database.
    information_schema.role_table_grants (grantor, grantee, table_catalog, table_schema, table_name, privilege_type) {
        /// Role that granted the privilege.
        grantor -> Nullable<Text>,
        /// Role that received the privilege.
        grantee -> Nullable<Text>,
        /// Catalog (database) containing the table.
        table_catalog -> Nullable<Text>,
        /// Schema containing the table.
        table_schema -> Nullable<Text>,
        /// Name of the table.
        table_name -> Nullable<Text>,
        /// Type of privilege granted.
        privilege_type -> Nullable<Text>,
        /// Whether the privilege is grantable.
        is_grantable -> Nullable<Text>,
        /// Whether the privilege applies to the hierarchy.
        with_hierarchy -> Nullable<Text>,
    }
}
