#![cfg(feature = "postgres")]
//! Submodule providing the diesel trait implementations for
//! [`FAIcon`](crate::FAIcon) for the PostgreSQL database.

impl diesel::deserialize::FromSql<crate::diesel_impls::FAIcon, diesel::pg::Pg> for crate::FAIcon {
    fn from_sql(value: diesel::pg::PgValue<'_>) -> diesel::deserialize::Result<Self> {
        Ok(Self::try_from(<String as diesel::deserialize::FromSql<
            diesel::sql_types::Text,
            diesel::pg::Pg,
        >>::from_sql(value)?)?)
    }
}

impl diesel::serialize::ToSql<crate::diesel_impls::FAIcon, diesel::pg::Pg> for crate::FAIcon {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>,
    ) -> diesel::serialize::Result {
        <str as diesel::serialize::ToSql<diesel::sql_types::Text, diesel::pg::Pg>>::to_sql(
            self.as_ref(),
            out,
        )
    }
}
