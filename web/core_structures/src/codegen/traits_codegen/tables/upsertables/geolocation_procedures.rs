#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::geolocation_procedures::GeolocationProcedure
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        use diesel::BoolExpressionMethods;
        use diesel::RunQueryDsl;
        use crate::codegen::diesel_codegen::tables::geolocation_procedures::geolocation_procedures::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure)
            .do_update()
            .set(self)
            .filter(
                procedure_template
                    .ne(excluded(procedure_template))
                    .or(geolocated_asset.ne(excluded(geolocated_asset)))
                    .or(procedure_template_geolocated_asset_model
                        .ne(excluded(procedure_template_geolocated_asset_model)))
                    .or(procedure_geolocated_asset.ne(excluded(procedure_geolocated_asset)))
                    .or(geolocated_with.ne(excluded(geolocated_with)))
                    .or(procedure_geolocated_with.ne(excluded(procedure_geolocated_with)))
                    .or(procedure_template_geolocated_with_model
                        .ne(excluded(procedure_template_geolocated_with_model)))
                    .or(location.ne(excluded(location))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
    for crate::codegen::structs_codegen::tables::geolocation_procedures::GeolocationProcedure
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        use diesel::BoolExpressionMethods;
        use diesel::RunQueryDsl;
        use crate::codegen::diesel_codegen::tables::geolocation_procedures::geolocation_procedures::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure)
            .do_update()
            .set(self)
            .filter(
                procedure_template
                    .ne(excluded(procedure_template))
                    .or(geolocated_asset.ne(excluded(geolocated_asset)))
                    .or(procedure_template_geolocated_asset_model
                        .ne(excluded(procedure_template_geolocated_asset_model)))
                    .or(procedure_geolocated_asset.ne(excluded(procedure_geolocated_asset)))
                    .or(geolocated_with.ne(excluded(geolocated_with)))
                    .or(procedure_geolocated_with.ne(excluded(procedure_geolocated_with)))
                    .or(procedure_template_geolocated_with_model
                        .ne(excluded(procedure_template_geolocated_with_model)))
                    .or(location.ne(excluded(location))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
