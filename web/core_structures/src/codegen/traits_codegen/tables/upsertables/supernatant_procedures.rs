#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::supernatant_procedures::SupernatantProcedure
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
        use crate::codegen::diesel_codegen::tables::supernatant_procedures::supernatant_procedures::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure_id)
            .do_update()
            .set(self)
            .filter(
                procedure_model_id
                    .ne(excluded(procedure_model_id))
                    .or(stratified_source.ne(excluded(stratified_source)))
                    .or(supernatant_destination.ne(excluded(supernatant_destination)))
                    .or(transferred_with.ne(excluded(transferred_with)))
                    .or(pipette_tip.ne(excluded(pipette_tip))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
    for crate::codegen::structs_codegen::tables::supernatant_procedures::SupernatantProcedure
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
        use crate::codegen::diesel_codegen::tables::supernatant_procedures::supernatant_procedures::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure_id)
            .do_update()
            .set(self)
            .filter(
                procedure_model_id
                    .ne(excluded(procedure_model_id))
                    .or(stratified_source.ne(excluded(stratified_source)))
                    .or(supernatant_destination.ne(excluded(supernatant_destination)))
                    .or(transferred_with.ne(excluded(transferred_with)))
                    .or(pipette_tip.ne(excluded(pipette_tip))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
