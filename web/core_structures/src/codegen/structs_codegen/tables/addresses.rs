#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    diesel::Selectable,
    diesel::Insertable,
    diesel::AsChangeset,
    diesel::Queryable,
    diesel::Identifiable,
)]
#[cfg_attr(feature = "yew", derive(yew::prelude::Properties))]
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
    ) -> Result<
        crate::codegen::structs_codegen::tables::cities::City,
        diesel::result::Error,
    >
    where
        crate::codegen::structs_codegen::tables::cities::City: diesel::Identifiable,
        <crate::codegen::structs_codegen::tables::cities::City as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::cities::City as diesel::Identifiable>::Id,
        >,
        <<crate::codegen::structs_codegen::tables::cities::City as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::cities::City as diesel::Identifiable>::Id,
        >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
        <<<crate::codegen::structs_codegen::tables::cities::City as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
            <crate::codegen::structs_codegen::tables::cities::City as diesel::Identifiable>::Id,
        >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
            'a,
            C,
            crate::codegen::structs_codegen::tables::cities::City,
        >,
    {
        use diesel::{QueryDsl, RunQueryDsl, associations::HasTable};
        RunQueryDsl::first(
            QueryDsl::find(
                crate::codegen::structs_codegen::tables::cities::City::table(),
                self.city_id,
            ),
            conn,
        )
    }
    #[cfg(feature = "postgres")]
    pub fn from_city_id(
        city_id: &i32,
        conn: &mut diesel::PgConnection,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, associations::HasTable};

        use crate::codegen::diesel_codegen::tables::addresses::addresses;
        Self::table()
            .filter(addresses::city_id.eq(city_id))
            .order_by(addresses::id.asc())
            .load::<Self>(conn)
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
    #[cfg(feature = "postgres")]
    pub fn from_city_id_and_street_name_and_street_number(
        city_id: &i32,
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
}
impl AsRef<Address> for Address {
    fn as_ref(&self) -> &Address {
        self
    }
}
