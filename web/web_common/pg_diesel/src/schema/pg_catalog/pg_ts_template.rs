//! Schema for pg_catalog.pg_ts_template table.

diesel::table! {
    use diesel::sql_types::*;

    /// Text search templates
    ///
    /// The catalog `pg_ts_template` contains entries defining text search templates.
    /// A template is the implementation skeleton for a class of text search dictionaries.
    pg_catalog.pg_ts_template (oid) {
        /// Row identifier
        oid -> Oid,
        /// Text search template name
        tmplname -> Text,
        /// The OID of the namespace that contains this template
        tmplnamespace -> Oid,
        /// OID of the template's initialization function
        tmplinit -> Oid,
        /// OID of the template's lexize function
        tmpllexize -> Oid,
    }
}
