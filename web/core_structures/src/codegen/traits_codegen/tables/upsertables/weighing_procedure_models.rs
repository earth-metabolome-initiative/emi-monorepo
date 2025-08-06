#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::weighing_procedure_models::WeighingProcedureModel
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
        use crate::codegen::diesel_codegen::tables::weighing_procedure_models::weighing_procedure_models::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure_model_id)
            .do_update()
            .set(self)
            .filter(
                weighed_with
                    .ne(excluded(weighed_with))
                    .or(procedure_weighed_with.ne(excluded(procedure_weighed_with)))
                    .or(sample_container_id.ne(excluded(sample_container_id)))
                    .or(procedure_sample_container.ne(excluded(procedure_sample_container))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
    for crate::codegen::structs_codegen::tables::weighing_procedure_models::WeighingProcedureModel
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
        use crate::codegen::diesel_codegen::tables::weighing_procedure_models::weighing_procedure_models::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure_model_id)
            .do_update()
            .set(self)
            .filter(
                weighed_with
                    .ne(excluded(weighed_with))
                    .or(procedure_weighed_with.ne(excluded(procedure_weighed_with)))
                    .or(sample_container_id.ne(excluded(sample_container_id)))
                    .or(procedure_sample_container.ne(excluded(procedure_sample_container))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
