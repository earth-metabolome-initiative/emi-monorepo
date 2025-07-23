#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::mixing_procedure_models::MixingProcedureModel
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
        use crate::codegen::diesel_codegen::tables::mixing_procedure_models::mixing_procedure_models::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure_model_id)
            .do_update()
            .set(self)
            .filter(
                measured_with
                    .ne(excluded(measured_with))
                    .or(procedure_measured_with.ne(excluded(procedure_measured_with)))
                    .or(source.ne(excluded(source)))
                    .or(mixed_with.ne(excluded(mixed_with)))
                    .or(procedure_mixed_into.ne(excluded(procedure_mixed_into)))
                    .or(kilograms.ne(excluded(kilograms))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
    for crate::codegen::structs_codegen::tables::mixing_procedure_models::MixingProcedureModel
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
        use crate::codegen::diesel_codegen::tables::mixing_procedure_models::mixing_procedure_models::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure_model_id)
            .do_update()
            .set(self)
            .filter(
                measured_with
                    .ne(excluded(measured_with))
                    .or(procedure_measured_with.ne(excluded(procedure_measured_with)))
                    .or(source.ne(excluded(source)))
                    .or(mixed_with.ne(excluded(mixed_with)))
                    .or(procedure_mixed_into.ne(excluded(procedure_mixed_into)))
                    .or(kilograms.ne(excluded(kilograms))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
