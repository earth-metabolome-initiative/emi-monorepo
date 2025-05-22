//! Implementation for the [`SQLite` Backend](diesel::sqlite::Sqlite).

impl diesel::deserialize::FromSql<crate::diesel_impls::Uuid, diesel::sqlite::Sqlite>
    for crate::Uuid
{
    fn from_sql(
        mut value: diesel::sqlite::SqliteValue<'_, '_, '_>,
    ) -> diesel::deserialize::Result<Self> {
        uuid::Uuid::from_slice(value.read_blob()).map_err(Into::into).map(Self::from)
    }
}

impl diesel::serialize::ToSql<crate::diesel_impls::Uuid, diesel::sqlite::Sqlite> for crate::Uuid {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, diesel::sqlite::Sqlite>,
    ) -> diesel::serialize::Result {
        out.set_value(self.as_bytes().as_slice());
        Ok(diesel::serialize::IsNull::No)
    }
}
