#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Queryable,
    diesel::Identifiable,
    diesel::Associations,
)]
#[cfg_attr(feature = "yew", derive(yew::prelude::Properties))]
#[diesel(
    belongs_to(
        crate::codegen::structs_codegen::tables::cities::City,
        foreign_key = city_id
    )
)]
#[diesel(primary_key(id))]
#[diesel(table_name = crate::codegen::diesel_codegen::tables::addresses::addresses)]
pub struct Address {
    pub id: i32,
    pub city_id: i32,
    pub street_name: String,
    pub street_number: String,
    pub postal_code: String,
    pub geolocation: postgis_diesel::types::Point,
}
impl web_common_traits::prelude::TableName for Address {
    const TABLE_NAME: &'static str = "addresses";
}
impl<'a> From<&'a Address>
    for web_common_traits::database::IdOrBuilder<
        i32,
        crate::codegen::structs_codegen::tables::insertables::InsertableAddressBuilder,
    >
{
    fn from(value: &'a Address) -> Self {
        web_common_traits::database::IdOrBuilder::Id(value.id)
    }
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
    pub fn city<C: diesel::connection::LoadConnection>(
        &self,
        conn: &mut C,
    ) -> Result<crate::codegen::structs_codegen::tables::cities::City, diesel::result::Error>
    where
        crate::codegen::structs_codegen::tables::cities::City: web_common_traits::database::Read<C>,
    {
        use web_common_traits::database::Read;
        crate::codegen::structs_codegen::tables::cities::City::read(self.city_id, conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_city_id_and_street_name_and_street_number(
        city_id: i32,
        street_name: &str,
        street_number: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable,
        };

        use crate::codegen::diesel_codegen::tables::addresses::addresses;
        Self::table()
            .filter(
                addresses::city_id
                    .eq(city_id)
                    .and(addresses::street_name.eq(street_name))
                    .and(addresses::street_number.eq(street_number)),
            )
            .order_by(addresses::id.asc())
            .first::<Self>(conn)
    }
    #[cfg(feature = "postgres")]
    pub fn from_street_name(
        street_name: &str,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::addresses::addresses;
        Self::table()
            .filter(addresses::street_name.eq(street_name))
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
}
impl AsRef<Address> for Address {
    fn as_ref(&self) -> &Address {
        self
    }
}
