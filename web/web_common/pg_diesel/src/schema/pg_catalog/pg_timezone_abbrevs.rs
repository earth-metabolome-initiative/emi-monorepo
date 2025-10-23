//! Schema for pg_catalog.pg_timezone_abbrevs view.

diesel::table! {
    /// Timezone abbreviations
    pg_catalog.pg_timezone_abbrevs (abbrev) {
        /// Timezone abbreviation
        abbrev -> Nullable<Text>,
        /// UTC offset
        utc_offset -> Nullable<diesel::sql_types::Interval>,
        /// Is daylight saving time
        is_dst -> Nullable<Bool>,
    }
}
