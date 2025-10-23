//! `sql_sizing` view from `information_schema`.

diesel::table! {
    /// `information_schema.sql_sizing` — view containing information about
    /// SQL sizing limits supported by the database system. Shows maximum
    /// values for various SQL constructs and implementation limits.
    information_schema.sql_sizing (sizing_id) {
        /// Identifier of the sizing limit.
        sizing_id -> Nullable<Integer>,
        /// Descriptive name of the sizing limit.
        sizing_name -> Nullable<Text>,
        /// Maximum supported value for this sizing limit.
        supported_value -> Nullable<Integer>,
        /// Additional comments about the sizing limit.
        comments -> Nullable<Text>,
    }
}
