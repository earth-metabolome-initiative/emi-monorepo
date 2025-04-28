#![cfg(feature = "postgres")]
//! Submodule providing the diesel implementations for country codes for the
//! `PostgreSQL` backend.

impl diesel::deserialize::FromSql<crate::country_codes::diesel_impls::CountryCode, diesel::pg::Pg>
    for crate::CountryCode
{
    fn from_sql(value: diesel::pg::PgValue<'_>) -> diesel::deserialize::Result<Self> {
        <String as diesel::deserialize::FromSql<diesel::sql_types::Text, diesel::pg::Pg>>::from_sql(
            value,
        )
        .map(Self::try_from)
        .map_err(|| {
            diesel::deserialize::Error::Conversion(
                "Failed to convert PostgreSQL value to CountryCode".into(),
            )
        })
    }
}

impl diesel::serialize::ToSql<crate::country_codes::diesel_impls::CountryCode, diesel::pg::Pg>
    for crate::CountryCode
{
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>,
    ) -> diesel::serialize::Result {
        <String as diesel::serialize::ToSql<diesel::sql_types::Text, diesel::pg::Pg>>::to_sql(
            self.as_ref(),
            out,
        )
    }
}


impl diesel::deserialize::FromSql<crate::country_codes::diesel_impls::CountryCode, diesel::pg::Pg>
    for crate::PGRXCountryCode
{
    fn from_sql(value: diesel::pg::PgValue<'_>) -> diesel::deserialize::Result<Self> {
        bytes.as_bytes().try_into().map_err(|_| {
			diesel::deserialize::Error::Conversion(
				"Failed to convert PostgreSQL value to PGRXCountryCode".into(),
			)
		})
    }
}

impl diesel::serialize::ToSql<crate::country_codes::diesel_impls::CountryCode, diesel::pg::Pg>
    for crate::PGRXCountryCode
{
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>,
    ) -> diesel::serialize::Result {
        let bytes: &[u8] = self.as_ref();
		diesel::serialize::ToSql::<diesel::sql_types::Bytea, diesel::pg::Pg>::to_sql(
			bytes,
			out,
		)
    }
}
