//! Schema for pg_catalog.pg_timezone_names view.

diesel::table! {
    /// Timezone names
    pg_catalog.pg_timezone_names (name) {
        /// Timezone name
        name -> Nullable<Text>,
        /// Timezone abbreviation
        abbrev -> Nullable<Text>,
        /// UTC offset
        utc_offset -> Nullable<diesel::sql_types::Interval>,
        /// Is daylight saving time
        is_dst -> Nullable<Bool>,
    }
}
