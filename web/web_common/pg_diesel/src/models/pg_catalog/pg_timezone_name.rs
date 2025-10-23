//! Model for pg_catalog.pg_timezone_names view.
use diesel::{pg::data_types::PgInterval, prelude::*};

#[derive(Queryable, QueryableByName, Selectable, Identifiable, Debug, PartialEq, Eq, Clone)]
#[diesel(table_name = crate::schema::pg_catalog::pg_timezone_names::pg_timezone_names)]
#[diesel(primary_key(name))]
/// Represents a row from the `pg_timezone_names` view.
pub struct PgTimezoneName {
    /// Timezone name
    pub name: Option<String>,
    /// Abbreviation of the timezone
    pub abbrev: Option<String>,
    /// UTC offset
    pub utc_offset: Option<PgInterval>,
    /// Is daylight saving time
    pub is_dst: Option<bool>,
}
