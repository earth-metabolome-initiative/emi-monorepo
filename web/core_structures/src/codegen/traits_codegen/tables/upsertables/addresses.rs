#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::addresses::Address
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use crate::codegen::diesel_codegen::tables::addresses::addresses::*;
        use diesel::BoolExpressionMethods;
        use diesel::ExpressionMethods;
        use diesel::RunQueryDsl;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(id)
            .do_update()
            .set(self)
            .filter(
                city_id
                    .ne(excluded(city_id))
                    .or(street_name.ne(excluded(street_name)))
                    .or(street_number.ne(excluded(street_number)))
                    .or(postal_code.ne(excluded(postal_code)))
                    .or(geolocation.ne(excluded(geolocation))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
    for crate::codegen::structs_codegen::tables::addresses::Address
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use crate::codegen::diesel_codegen::tables::addresses::addresses::*;
        use diesel::BoolExpressionMethods;
        use diesel::ExpressionMethods;
        use diesel::RunQueryDsl;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(id)
            .do_update()
            .set(self)
            .filter(
                city_id
                    .ne(excluded(city_id))
                    .or(street_name.ne(excluded(street_name)))
                    .or(street_number.ne(excluded(street_number)))
                    .or(postal_code.ne(excluded(postal_code)))
                    .or(geolocation.ne(excluded(geolocation))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
