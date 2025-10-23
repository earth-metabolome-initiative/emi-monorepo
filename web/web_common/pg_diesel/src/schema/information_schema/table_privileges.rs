//! `table_privileges` view from `information_schema`.

diesel::table! {
    /// `information_schema.table_privileges` — view containing one row for each
    /// privilege granted on a table or view to a currently enabled role or
    /// granted by a currently enabled role.
    information_schema.table_privileges (grantor, grantee, table_catalog, table_schema, table_name, privilege_type) {
        /// Name of the role that granted the privilege.
        grantor -> Nullable<Text>,
        /// Name of the role that was granted the privilege.
        grantee -> Nullable<Text>,
        /// Name of the database (catalog) containing the table.
        table_catalog -> Nullable<Text>,
        /// Name of the schema containing the table.
        table_schema -> Nullable<Text>,
        /// Name of the table.
        table_name -> Nullable<Text>,
        /// Type of privilege (SELECT, INSERT, UPDATE, DELETE, etc.).
        privilege_type -> Nullable<Text>,
        /// "YES" if the privilege is grantable to others, "NO" otherwise.
        is_grantable -> Nullable<Text>,
        /// "YES" if the privilege applies to hierarchy, "NO" otherwise.
        with_hierarchy -> Nullable<Text>,
    }
}
