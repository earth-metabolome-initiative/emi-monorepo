//! Schema for pg_catalog.pg_ts_parser table.

diesel::table! {
    use diesel::sql_types::*;

    /// Text search parsers
    ///
    /// The catalog `pg_ts_parser` contains entries defining text search parsers.
    /// A parser is responsible for splitting input text into lexemes and assigning
    /// a token type to each lexeme.
    pg_catalog.pg_ts_parser (oid) {
        /// Row identifier
        oid -> Oid,
        /// Text search parser name
        prsname -> Text,
        /// The OID of the namespace that contains this parser
        prsnamespace -> Oid,
        /// OID of the parser's startup function
        prsstart -> Oid,
        /// OID of the parser's next-token function
        prstoken -> Oid,
        /// OID of the parser's shutdown function
        prsend -> Oid,
        /// OID of the parser's headline function
        prsheadline -> Oid,
        /// OID of the parser's lextype function
        prslextype -> Oid,
    }
}
