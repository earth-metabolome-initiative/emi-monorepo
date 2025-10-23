//! Submodule for the `pg_catalog.pg_available_extension_versions` view schema.

diesel::table! {
    /// `pg_catalog.pg_available_extension_versions` — view of available extension versions.
    /// Shows all extension versions that are available for installation.
    pg_catalog.pg_available_extension_versions (name, version) {
        /// Name of the extension (part of composite primary key).
        name -> Nullable<Text>,
        /// Version string of the extension (part of composite primary key).
        version -> Nullable<Text>,
        /// Whether this version is currently installed.
        installed -> Nullable<Bool>,
        /// Whether installation requires superuser privileges.
        superuser -> Nullable<Bool>,
        /// Whether the extension is trusted (can be installed by non-superusers).
        trusted -> Nullable<Bool>,
        /// Whether the extension can be relocated to another schema.
        relocatable -> Nullable<Bool>,
        /// Default schema for the extension.
        schema -> Nullable<Text>,
        /// Array of extensions that this extension requires.
        requires -> Nullable<Array<Text>>,
        /// Comment describing the extension.
        comment -> Nullable<Text>,
    }
}
