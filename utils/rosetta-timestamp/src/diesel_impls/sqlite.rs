//! Submodule proving the implementations of the traits relative to Diesel with SQLite as the backend.


impl diesel::deserialize::FromSql<crate::diesel_impls::TimestampUTC, diesel::sqlite::Sqlite>
    for crate::TimestampUTC
{
    fn from_sql(value: diesel::sqlite::SqliteValue<'_, '_, '_>,) -> diesel::deserialize::Result<Self> {
        <chrono::NaiveDateTime as diesel::deserialize::FromSql<
            diesel::sql_types::Timestamp,
            diesel::sqlite::Sqlite,
        >>::from_sql(value)
        .map(Self::from)
    }
}

impl diesel::serialize::ToSql<crate::diesel_impls::TimestampUTC, diesel::sqlite::Sqlite>
    for crate::TimestampUTC
{
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, diesel::sqlite::Sqlite>,
    ) -> diesel::serialize::Result {
        <chrono::DateTime<chrono::Utc> as diesel::serialize::ToSql<
            diesel::sql_types::TimestamptzSqlite,
            diesel::sqlite::Sqlite,
        >>::to_sql(self.as_ref(), out)
    }
}
