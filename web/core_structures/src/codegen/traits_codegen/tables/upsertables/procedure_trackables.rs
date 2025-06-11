#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl,
            upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::procedure_trackables::procedure_trackables::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict((procedure_id, trackable_id))
            .do_update()
            .set(self)
            .filter(
                procedure_model_id
                    .ne(excluded(procedure_model_id))
                    .or(procedure_model_trackable_id.ne(excluded(procedure_model_trackable_id)))
                    .or(ancestor_trackable_id.ne(excluded(ancestor_trackable_id)))
                    .or(parent_trackable_id.ne(excluded(parent_trackable_id)))
                    .or(created_by.ne(excluded(created_by)))
                    .or(created_at.ne(excluded(created_at))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
    for crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl,
            upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::procedure_trackables::procedure_trackables::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict((procedure_id, trackable_id))
            .do_update()
            .set(self)
            .filter(
                procedure_model_id
                    .ne(excluded(procedure_model_id))
                    .or(procedure_model_trackable_id.ne(excluded(procedure_model_trackable_id)))
                    .or(ancestor_trackable_id.ne(excluded(ancestor_trackable_id)))
                    .or(parent_trackable_id.ne(excluded(parent_trackable_id)))
                    .or(created_by.ne(excluded(created_by)))
                    .or(created_at.ne(excluded(created_at))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
