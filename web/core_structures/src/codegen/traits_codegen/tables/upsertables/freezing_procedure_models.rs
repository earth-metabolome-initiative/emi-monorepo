#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::freezing_procedure_models::FreezingProcedureModel
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
        use crate::codegen::diesel_codegen::tables::freezing_procedure_models::freezing_procedure_models::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure_model_id)
            .do_update()
            .set(self)
            .filter(
                kelvin
                    .ne(excluded(kelvin))
                    .or(kelvin_tolerance_percentage.ne(excluded(kelvin_tolerance_percentage)))
                    .or(seconds.ne(excluded(seconds)))
                    .or(frozen_with.ne(excluded(frozen_with)))
                    .or(procedure_frozen_with.ne(excluded(procedure_frozen_with)))
                    .or(frozen_container_id.ne(excluded(frozen_container_id)))
                    .or(procedure_frozen_container_id.ne(excluded(procedure_frozen_container_id))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
    for crate::codegen::structs_codegen::tables::freezing_procedure_models::FreezingProcedureModel
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
        use crate::codegen::diesel_codegen::tables::freezing_procedure_models::freezing_procedure_models::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure_model_id)
            .do_update()
            .set(self)
            .filter(
                kelvin
                    .ne(excluded(kelvin))
                    .or(kelvin_tolerance_percentage.ne(excluded(kelvin_tolerance_percentage)))
                    .or(seconds.ne(excluded(seconds)))
                    .or(frozen_with.ne(excluded(frozen_with)))
                    .or(procedure_frozen_with.ne(excluded(procedure_frozen_with)))
                    .or(frozen_container_id.ne(excluded(frozen_container_id)))
                    .or(procedure_frozen_container_id.ne(excluded(procedure_frozen_container_id))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
