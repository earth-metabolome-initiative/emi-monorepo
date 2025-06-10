#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::procedures::Procedure
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use crate::codegen::diesel_codegen::tables::procedures::procedures::*;
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
                procedure_model_id
                    .ne(excluded(procedure_model_id))
                    .or(created_by.ne(excluded(created_by)))
                    .or(created_at.ne(excluded(created_at)))
                    .or(updated_by.ne(excluded(updated_by)))
                    .or(updated_at.ne(excluded(updated_at))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
    for crate::codegen::structs_codegen::tables::procedures::Procedure
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use crate::codegen::diesel_codegen::tables::procedures::procedures::*;
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
                procedure_model_id
                    .ne(excluded(procedure_model_id))
                    .or(created_by.ne(excluded(created_by)))
                    .or(created_at.ne(excluded(created_at)))
                    .or(updated_by.ne(excluded(updated_by)))
                    .or(updated_at.ne(excluded(updated_at))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
