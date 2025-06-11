#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::ball_mill_procedure_models::BallMillProcedureModel
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
        use crate::codegen::diesel_codegen::tables::ball_mill_procedure_models::ball_mill_procedure_models::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(id)
            .do_update()
            .set(self)
            .filter(
                seconds
                    .ne(excluded(seconds))
                    .or(hertz.ne(excluded(hertz)))
                    .or(milled_with.ne(excluded(milled_with)))
                    .or(container_id.ne(excluded(container_id))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
    for crate::codegen::structs_codegen::tables::ball_mill_procedure_models::BallMillProcedureModel
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
        use crate::codegen::diesel_codegen::tables::ball_mill_procedure_models::ball_mill_procedure_models::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(id)
            .do_update()
            .set(self)
            .filter(
                seconds
                    .ne(excluded(seconds))
                    .or(hertz.ne(excluded(hertz)))
                    .or(milled_with.ne(excluded(milled_with)))
                    .or(container_id.ne(excluded(container_id))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
