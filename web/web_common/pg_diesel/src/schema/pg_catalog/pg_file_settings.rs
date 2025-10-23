//! Submodule for the `pg_catalog.pg_file_settings` view schema.

diesel::table! {
    /// `pg_catalog.pg_file_settings` — view showing the contents of server configuration files.
    /// Each row represents one setting from postgresql.conf, auto.conf, or included files.
    pg_catalog.pg_file_settings (sourcefile, sourceline, seqno) {
        /// Configuration file where this setting is defined.
        sourcefile -> Nullable<Text>,
        /// Line number within the source file.
        sourceline -> Nullable<Integer>,
        /// Sequence number showing order of processing.
        seqno -> Nullable<Integer>,
        /// Name of the configuration parameter.
        name -> Nullable<Text>,
        /// Value of the configuration parameter as it appears in the file.
        setting -> Nullable<Text>,
        /// `true` if the setting was successfully applied.
        applied -> Nullable<Bool>,
        /// Error message if the setting failed to apply.
        error -> Nullable<Text>,
    }
}
