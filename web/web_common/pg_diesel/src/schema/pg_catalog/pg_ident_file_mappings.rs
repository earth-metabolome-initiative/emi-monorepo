//! Submodule for the `pg_catalog.pg_ident_file_mappings` view schema.

diesel::table! {
    /// `pg_catalog.pg_ident_file_mappings` — view showing the contents of the user name mapping
    /// file (pg_ident.conf). Each row represents one mapping from system user to database user.
    pg_catalog.pg_ident_file_mappings (map_number) {
        /// Mapping number (sequence in which mappings are processed).
        map_number -> Nullable<Integer>,
        /// Name of the file containing this mapping.
        file_name -> Nullable<Text>,
        /// Line number of this mapping within the file.
        line_number -> Nullable<Integer>,
        /// Name of the map.
        map_name -> Nullable<Text>,
        /// System user name (can be a regular expression).
        sys_name -> Nullable<Text>,
        /// PostgreSQL user name that the system user maps to.
        pg_username -> Nullable<Text>,
        /// Error message if the mapping is invalid.
        error -> Nullable<Text>,
    }
}
