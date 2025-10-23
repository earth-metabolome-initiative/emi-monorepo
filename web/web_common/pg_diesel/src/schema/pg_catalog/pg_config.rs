//! Submodule for the `pg_catalog.pg_config` view schema.

diesel::table! {
    /// `pg_catalog.pg_config` — view exposing PostgreSQL's compile-time configuration information.
    /// Each row represents one configuration option, showing name-value pairs from the server's
    /// build settings (such as installation paths, library directories, version info, etc.).
    pg_catalog.pg_config (name) {
        /// Name of the configuration parameter.
        name -> Nullable<Text>,
        /// Current value of the configuration parameter.
        setting -> Nullable<Text>,
    }
}
