//! `sql_parts` view from `information_schema`.

diesel::table! {
    /// `information_schema.sql_parts` — view containing information about
    /// major parts of the SQL standard supported by the database system.
    /// Shows which SQL standard parts are supported and verified.
    information_schema.sql_parts (feature_id) {
        /// Identifier of the SQL standard part.
        feature_id -> Nullable<Text>,
        /// Descriptive name of the SQL standard part.
        feature_name -> Nullable<Text>,
        /// "YES" if the part is supported, "NO" if not.
        is_supported -> Nullable<Text>,
        /// Name of the conformance test that verifies this part.
        is_verified_by -> Nullable<Text>,
        /// Additional comments about the part implementation.
        comments -> Nullable<Text>,
    }
}
