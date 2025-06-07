#![doc = include_str!("../README.md")]

use chrono::NaiveDateTime;

#[cfg(feature = "diesel")]
pub mod diesel_impls;
#[cfg(feature = "pgrx")]
mod pgrx_impls;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
    feature = "diesel",
    derive(diesel::expression::AsExpression, diesel::deserialize::FromSqlRow)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature="diesel", diesel(sql_type = crate::diesel_impls::TimestampUTC))]
/// A wrapper around the `chrono` crate's `DateTime<Utc>` type.
pub struct TimestampUTC(chrono::DateTime<chrono::Utc>);

impl Default for TimestampUTC {
    fn default() -> Self {
        Self(chrono::Utc::now())
    }
}

impl From<chrono::DateTime<chrono::Utc>> for TimestampUTC {
    fn from(value: chrono::DateTime<chrono::Utc>) -> Self {
        Self(value)
    }
}

impl From<TimestampUTC> for chrono::DateTime<chrono::Utc> {
    fn from(value: TimestampUTC) -> Self {
        value.0
    }
}

impl AsRef<chrono::DateTime<chrono::Utc>> for TimestampUTC {
    fn as_ref(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.0
    }
}

impl AsMut<chrono::DateTime<chrono::Utc>> for TimestampUTC {
    fn as_mut(&mut self) -> &mut chrono::DateTime<chrono::Utc> {
        &mut self.0
    }
}

impl From<NaiveDateTime> for TimestampUTC {
    fn from(value: NaiveDateTime) -> Self {
        use chrono::TimeZone;
        Self(chrono::Utc.from_utc_datetime(&value))
    }
}
