#![cfg(feature = "postgres")]
//! Submodule proving the implementations of the traits relative to Diesel with
//! `PostgreSQL` as the backend.

impl diesel::deserialize::FromSql<crate::diesel_impls::TimestampUTC, diesel::pg::Pg>
    for crate::TimestampUTC
{
    fn from_sql(value: diesel::pg::PgValue<'_>) -> diesel::deserialize::Result<Self> {
        <chrono::DateTime<chrono::Utc> as diesel::deserialize::FromSql<
            diesel::sql_types::Timestamptz,
            diesel::pg::Pg,
        >>::from_sql(value)
        .map(Self::from)
    }
}

impl diesel::serialize::ToSql<crate::diesel_impls::TimestampUTC, diesel::pg::Pg>
    for crate::TimestampUTC
{
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>,
    ) -> diesel::serialize::Result {
        <chrono::DateTime<chrono::Utc> as diesel::serialize::ToSql<
            diesel::sql_types::Timestamptz,
            diesel::pg::Pg,
        >>::to_sql(self.as_ref(), out)
    }
}
