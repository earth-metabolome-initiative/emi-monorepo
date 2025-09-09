#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Queryable,
    diesel::Identifiable,
)]
#[diesel(primary_key(id))]
#[diesel(table_name = crate::codegen::diesel_codegen::tables::addresses::addresses)]
pub struct Address {
    pub id: i32,
    pub country: String,
    pub city: String,
    pub street: String,
    pub street_number: String,
    pub postal_code: String,
    pub geolocation: postgis_diesel::types::GeometryContainer<postgis_diesel::types::Point>,
    pub city_code: String,
}
impl web_common_traits::prelude::TableName for Address {
    const TABLE_NAME: &'static str = "Addresses";
}
impl
    web_common_traits::prelude::ExtensionTable<
        crate::codegen::structs_codegen::tables::addresses::Address,
    > for Address
where
    for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
{
}
impl diesel::Identifiable for Address {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl Address {
    #[cfg(feature = "postgres")]
    pub fn from_country(
        country: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::addresses::addresses;
        Self::table()
            .filter(addresses::country.eq(country))
            .order_by(addresses::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_city(
        city: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::addresses::addresses;
        Self::table()
            .filter(addresses::city.eq(city))
            .order_by(addresses::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_street(
        street: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::addresses::addresses;
        Self::table()
            .filter(addresses::street.eq(street))
            .order_by(addresses::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_street_number(
        street_number: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::addresses::addresses;
        Self::table()
            .filter(addresses::street_number.eq(street_number))
            .order_by(addresses::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_postal_code(
        postal_code: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::addresses::addresses;
        Self::table()
            .filter(addresses::postal_code.eq(postal_code))
            .order_by(addresses::id.asc())
            .load::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_city_code(
        city_code: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::addresses::addresses;
        Self::table()
            .filter(addresses::city_code.eq(city_code))
            .order_by(addresses::id.asc())
            .load::<Self>(conn)
    }
}
impl AsRef<Address> for Address {
    fn as_ref(&self) -> &Address {
        self
    }
}
