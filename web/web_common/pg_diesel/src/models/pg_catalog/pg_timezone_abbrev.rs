//! Model for pg_catalog.pg_timezone_abbrevs view.
use diesel::{pg::data_types::PgInterval, prelude::*};

#[derive(Queryable, QueryableByName, Selectable, Identifiable, Debug, PartialEq, Eq, Clone)]
#[diesel(table_name = crate::schema::pg_catalog::pg_timezone_abbrevs::pg_timezone_abbrevs)]
#[diesel(primary_key(abbrev))]
/// Represents a row from the `pg_timezone_abbrevs` view.
pub struct PgTimezoneAbbrev {
    /// Timezone abbreviation
    pub abbrev: Option<String>,
    /// UTC offset
    pub utc_offset: Option<PgInterval>,
    /// Is daylight saving time
    pub is_dst: Option<bool>,
}
