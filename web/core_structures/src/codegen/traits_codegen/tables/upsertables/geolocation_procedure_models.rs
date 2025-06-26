#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
for crate::codegen::structs_codegen::tables::geolocation_procedure_models::GeolocationProcedureModel {
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        use diesel::BoolExpressionMethods;
        use diesel::RunQueryDsl;
        use crate::codegen::diesel_codegen::tables::geolocation_procedure_models::geolocation_procedure_models::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure_model_id)
            .do_update()
            .set(self)
            .filter(
                geolocated_with
                    .ne(excluded(geolocated_with))
                    .or(
                        procedure_geolocated_with.ne(excluded(procedure_geolocated_with)),
                    )
                    .or(trackable_id.ne(excluded(trackable_id))),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
for crate::codegen::structs_codegen::tables::geolocation_procedure_models::GeolocationProcedureModel {
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        use diesel::BoolExpressionMethods;
        use diesel::RunQueryDsl;
        use crate::codegen::diesel_codegen::tables::geolocation_procedure_models::geolocation_procedure_models::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure_model_id)
            .do_update()
            .set(self)
            .filter(
                geolocated_with
                    .ne(excluded(geolocated_with))
                    .or(
                        procedure_geolocated_with.ne(excluded(procedure_geolocated_with)),
                    )
                    .or(trackable_id.ne(excluded(trackable_id))),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
