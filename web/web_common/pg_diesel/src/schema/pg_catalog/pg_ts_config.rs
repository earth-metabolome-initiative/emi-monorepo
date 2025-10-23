//! Schema for pg_catalog.pg_ts_config table.

diesel::table! {
    use diesel::sql_types::*;

    /// Text search configurations
    ///
    /// The catalog `pg_ts_config` contains entries representing text search configurations.
    /// A configuration specifies a particular text search parser and a list of dictionaries
    /// to use for each of the parser's output token types.
    pg_catalog.pg_ts_config (oid) {
        /// Row identifier
        oid -> Oid,
        /// Text search configuration name
        cfgname -> Text,
        /// The OID of the namespace that contains this configuration
        cfgnamespace -> Oid,
        /// Owner of the configuration
        cfgowner -> Oid,
        /// The OID of the text search parser for this configuration
        cfgparser -> Oid,
    }
}
