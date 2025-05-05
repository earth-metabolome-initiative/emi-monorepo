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
impl diesel::Identifiable for Address {
    type Id = i32;
    fn id(self) -> Self::Id {
        self.id
    }
}
impl Address {
    #[cfg(feature = "postgres")]
    pub async fn city(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::cities::City, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::cities::City::table()
            .filter(
                crate::codegen::diesel_codegen::tables::cities::cities::dsl::id.eq(&self.city_id),
            )
            .first::<crate::codegen::structs_codegen::tables::cities::City>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_city_id(
        conn: &mut diesel_async::AsyncPgConnection,
        city_id: &crate::codegen::structs_codegen::tables::cities::City,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::addresses::addresses::dsl::city_id
                    .eq(city_id.id),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_city_id_and_street_name_and_street_number(
        city_id: &i32,
        street_name: &str,
        street_number: &str,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{OptionalExtension, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(diesel::BoolExpressionMethods::and(
                diesel::BoolExpressionMethods::and(
                    diesel::ExpressionMethods::eq(
                        crate::codegen::diesel_codegen::tables::addresses::addresses::city_id,
                        city_id,
                    ),
                    diesel::ExpressionMethods::eq(
                        crate::codegen::diesel_codegen::tables::addresses::addresses::street_name,
                        street_name,
                    ),
                ),
                diesel::ExpressionMethods::eq(
                    crate::codegen::diesel_codegen::tables::addresses::addresses::street_number,
                    street_number,
                ),
            ))
            .first::<Self>(conn)
            .await
            .optional()
    }
}
