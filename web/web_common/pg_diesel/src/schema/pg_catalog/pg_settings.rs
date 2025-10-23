//! Submodule for the `pg_catalog.pg_settings` view schema.

diesel::table! {
    /// `pg_catalog.pg_settings` — view exposing PostgreSQL run-time configuration parameters (GUCs).
    /// Each row represents one server configuration parameter and metadata about it:
    /// name, current value, type, source, allowable range, descriptions, and file/location info.
    pg_catalog.pg_settings (name) {
        /// Name of the configuration parameter.
        name -> Text,
        /// Current value of the parameter, formatted as text (units stripped; booleans as "on"/"off").
        setting -> Text,
        /// Measurement unit for the parameter (e.g., "kB", "ms", "s"); `NULL` if unitless.
        unit -> Nullable<Text>,
        /// Logical category used for grouping/documentation (e.g., "Memory", "Logging").
        category -> Text,
        /// Short description summarizing what the parameter controls.
        short_desc -> Text,
        /// Additional explanatory/descriptive text about the parameter; may be `NULL`.
        extra_desc -> Nullable<Text>,
        /// Change context describing when/how the parameter can be set or takes effect
        /// (examples: "internal", "postmaster", "sighup", "backend", "superuser", "user").
        context -> Text,
        /// Parameter data type: "bool", "integer", "real", "enum", or "string".
        vartype -> Text,
        /// Source of the current value (e.g., "default", "configuration file", "command line",
        /// "environment", "session", "database", "user", "override").
        source -> Text,
        /// Minimum allowed value for the parameter (as text); `NULL` if not applicable.
        min_val -> Nullable<Text>,
        /// Maximum allowed value for the parameter (as text); `NULL` if not applicable.
        max_val -> Nullable<Text>,
        /// For enum-typed parameters: array of allowed textual values; `NULL` for non-enums.
        enumvals -> Nullable<Array<Text>>,
        /// Built-in (compiled-in) default value for the parameter; may be `NULL`.
        boot_val -> Nullable<Text>,
        /// The value the parameter would have after a `RESET` (session/system default);
        /// may be `NULL`.
        reset_val -> Nullable<Text>,
        /// File path from which the current setting was read (typically `postgresql.conf`);
        /// `NULL` if not applicable.
        sourcefile -> Nullable<Text>,
        /// Line number within `sourcefile` where this setting appears; `NULL` if unknown.
        sourceline -> Nullable<Integer>,
        /// `true` if the parameter change is pending and requires a server restart to take effect;
        /// `NULL` if not applicable.
        pending_restart -> Nullable<Bool>,
    }
}
