#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
for crate::codegen::structs_codegen::tables::aliquoting_procedure_models::AliquotingProcedureModel {
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        use diesel::BoolExpressionMethods;
        use diesel::RunQueryDsl;
        use crate::codegen::diesel_codegen::tables::aliquoting_procedure_models::aliquoting_procedure_models::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure_model_id)
            .do_update()
            .set(self)
            .filter(
                liters
                    .ne(excluded(liters))
                    .or(aliquoted_from.ne(excluded(aliquoted_from)))
                    .or(procedure_aliquoted_from.ne(excluded(procedure_aliquoted_from)))
                    .or(aliquoted_into.ne(excluded(aliquoted_into)))
                    .or(procedure_aliquoted_into.ne(excluded(procedure_aliquoted_into)))
                    .or(aliquoted_with.ne(excluded(aliquoted_with)))
                    .or(procedure_aliquoted_with.ne(excluded(procedure_aliquoted_with)))
                    .or(pipette_tip.ne(excluded(pipette_tip)))
                    .or(procedure_pipette_tip.ne(excluded(procedure_pipette_tip))),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
for crate::codegen::structs_codegen::tables::aliquoting_procedure_models::AliquotingProcedureModel {
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        use diesel::BoolExpressionMethods;
        use diesel::RunQueryDsl;
        use crate::codegen::diesel_codegen::tables::aliquoting_procedure_models::aliquoting_procedure_models::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure_model_id)
            .do_update()
            .set(self)
            .filter(
                liters
                    .ne(excluded(liters))
                    .or(aliquoted_from.ne(excluded(aliquoted_from)))
                    .or(procedure_aliquoted_from.ne(excluded(procedure_aliquoted_from)))
                    .or(aliquoted_into.ne(excluded(aliquoted_into)))
                    .or(procedure_aliquoted_into.ne(excluded(procedure_aliquoted_into)))
                    .or(aliquoted_with.ne(excluded(aliquoted_with)))
                    .or(procedure_aliquoted_with.ne(excluded(procedure_aliquoted_with)))
                    .or(pipette_tip.ne(excluded(pipette_tip)))
                    .or(procedure_pipette_tip.ne(excluded(procedure_pipette_tip))),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
