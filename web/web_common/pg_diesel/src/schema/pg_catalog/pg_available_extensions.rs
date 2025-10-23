//! Submodule for the `pg_catalog.pg_available_extensions` view schema.

diesel::table! {
    /// `pg_catalog.pg_available_extensions` — view of available extensions.
    /// Shows extensions that are available for installation.
    pg_catalog.pg_available_extensions (name) {
        /// Name of the extension (primary key).
        name -> Nullable<Text>,
        /// Default version of the extension.
        default_version -> Nullable<Text>,
        /// Currently installed version (if any).
        installed_version -> Nullable<Text>,
        /// Comment describing the extension.
        comment -> Nullable<Text>,
    }
}
