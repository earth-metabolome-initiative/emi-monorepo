#![cfg(feature = "sqlite")]
//! Submodule providing the diesel trait implementations for
//! [`FAIcon`](crate::FAIcon) for the SQLite database.

impl diesel::deserialize::FromSql<crate::diesel_impls::FAIcon, diesel::sqlite::Sqlite>
    for crate::FAIcon
{
    fn from_sql(
        value: diesel::sqlite::SqliteValue<'_, '_, '_>,
    ) -> diesel::deserialize::Result<Self> {
        Ok(Self::try_from(<String as diesel::deserialize::FromSql<
            diesel::sql_types::Text,
            diesel::sqlite::Sqlite,
        >>::from_sql(value)?)?)
    }
}

impl diesel::serialize::ToSql<crate::diesel_impls::FAIcon, diesel::sqlite::Sqlite>
    for crate::FAIcon
{
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, diesel::sqlite::Sqlite>,
    ) -> diesel::serialize::Result {
        <str as diesel::serialize::ToSql<diesel::sql_types::Text, diesel::sqlite::Sqlite>>::to_sql(
            self.as_ref(),
            out,
        )
    }
}
