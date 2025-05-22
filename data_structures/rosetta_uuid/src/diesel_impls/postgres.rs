//! Implementation for the [Postgres Backend](diesel::pg::Pg).

impl diesel::deserialize::FromSql<crate::diesel_impls::Uuid, diesel::pg::Pg> for crate::Uuid {
    fn from_sql(value: diesel::pg::PgValue<'_>) -> diesel::deserialize::Result<Self> {
        <uuid::Uuid as diesel::deserialize::FromSql<diesel::sql_types::Uuid, diesel::pg::Pg>>::from_sql(value).map(Self::from)
    }
}

impl diesel::serialize::ToSql<crate::diesel_impls::Uuid, diesel::pg::Pg> for crate::Uuid {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>,
    ) -> diesel::serialize::Result {
        <uuid::Uuid as diesel::serialize::ToSql<diesel::sql_types::Uuid, diesel::pg::Pg>>::to_sql(
            self.as_ref(),
            out,
        )
    }
}
