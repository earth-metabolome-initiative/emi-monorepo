//! Submodule for the `information_schema.column_privileges` view schema.

diesel::table! {
    /// `information_schema.column_privileges` — view containing one row for each privilege
    /// granted on a column to a user or role. This provides access control information
    /// for column-level security.
    information_schema.column_privileges (grantor, grantee, table_catalog, table_schema, table_name, column_name, privilege_type) {
        /// Name of the user who granted the privilege.
        grantor -> Nullable<Text>,
        /// Name of the user or role to whom the privilege was granted.
        grantee -> Nullable<Text>,
        /// Catalog (database) containing the table.
        table_catalog -> Nullable<Text>,
        /// Schema containing the table.
        table_schema -> Nullable<Text>,
        /// Name of the table.
        table_name -> Nullable<Text>,
        /// Name of the column.
        column_name -> Nullable<Text>,
        /// Type of privilege granted.
        privilege_type -> Nullable<Text>,
        /// Whether the privilege is grantable.
        is_grantable -> Nullable<Text>,
    }
}
