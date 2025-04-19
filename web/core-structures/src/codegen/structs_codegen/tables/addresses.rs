#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(diesel::Selectable, diesel::Queryable, diesel::Identifiable)]
#[diesel(primary_key(id))]
#[diesel(table_name = crate::codegen::diesel_codegen::tables::addresses::addresses)]
pub struct Address {
    pub id: i32,
    pub iso: String,
    pub city_code: String,
    pub street_name: String,
    pub street_number: String,
    pub postal_code: String,
    pub geolocation: postgis_diesel::types::Point,
}
impl Address {
    #[cfg(feature = "postgres")]
    pub async fn iso(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::countries::Country, diesel::result::Error>
    {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::countries::Country::table()
            .filter(
                crate::codegen::diesel_codegen::tables::countries::countries::dsl::iso
                    .eq(&self.iso),
            )
            .first::<crate::codegen::structs_codegen::tables::countries::Country>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn city_code(
        &self,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<crate::codegen::structs_codegen::tables::cities::City, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        crate::codegen::structs_codegen::tables::cities::City::table()
            .filter(
                crate::codegen::diesel_codegen::tables::cities::cities::dsl::code
                    .eq(&self.city_code),
            )
            .first::<crate::codegen::structs_codegen::tables::cities::City>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_iso(
        conn: &mut diesel_async::AsyncPgConnection,
        iso: &crate::codegen::structs_codegen::tables::countries::Country,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::addresses::addresses::dsl::iso.eq(&iso.iso),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_city_code(
        conn: &mut diesel_async::AsyncPgConnection,
        city_code: &crate::codegen::structs_codegen::tables::cities::City,
    ) -> Result<Vec<Self>, diesel::result::Error> {
        use diesel::{ExpressionMethods, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(
                crate::codegen::diesel_codegen::tables::addresses::addresses::dsl::city_code
                    .eq(&city_code.code),
            )
            .load::<Self>(conn)
            .await
    }
    #[cfg(feature = "postgres")]
    pub async fn from_iso_and_city_code_and_street_name_and_street_number(
        iso: &str,
        city_code: &str,
        street_name: &str,
        street_number: &str,
        conn: &mut diesel_async::AsyncPgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{OptionalExtension, QueryDsl, associations::HasTable};
        use diesel_async::RunQueryDsl;
        Self::table()
            .filter(diesel::BoolExpressionMethods::and(
                diesel::BoolExpressionMethods::and(
                    diesel::BoolExpressionMethods::and(
                        diesel::ExpressionMethods::eq(
                            crate::codegen::diesel_codegen::tables::addresses::addresses::iso,
                            iso,
                        ),
                        diesel::ExpressionMethods::eq(
                            crate::codegen::diesel_codegen::tables::addresses::addresses::city_code,
                            city_code,
                        ),
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
