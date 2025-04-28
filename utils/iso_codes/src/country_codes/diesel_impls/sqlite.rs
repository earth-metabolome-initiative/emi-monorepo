#![cfg(feature = "sqlite")]
//! Submodule providing the diesel implementations for country codes for the
//! SQLite backend.

impl
    diesel::deserialize::FromSql<
        crate::country_codes::diesel_impls::CountryCode,
        diesel::sqlite::Sqlite,
    > for crate::CountryCode
{
    fn from_sql(
        value: diesel::sqlite::SqliteValue<'_, '_, '_>,
    ) -> diesel::deserialize::Result<Self> {
        <String as diesel::deserialize::FromSql<
            diesel::sql_types::Text,
            diesel::sqlite::Sqlite,
        >>::from_sql(value)
        .map(Self::try_from)
		.map_err(|| diesel::deserialize::Error::Conversion(
			"Failed to convert SQLite value to CountryCode".into(),
		))
    }
}

impl
    diesel::serialize::ToSql<
        crate::country_codes::diesel_impls::CountryCode,
        diesel::sqlite::Sqlite,
    > for crate::CountryCode
{
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, diesel::sqlite::Sqlite>,
    ) -> diesel::serialize::Result {
        <String as diesel::serialize::ToSql<
            diesel::sql_types::Text,
            diesel::sqlite::Sqlite,
        >>::to_sql(self.as_ref(), out)
    }
}
