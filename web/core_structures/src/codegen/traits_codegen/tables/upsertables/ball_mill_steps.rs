#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::ball_mill_steps::BallMillStep
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl,
            upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::ball_mill_steps::ball_mill_steps::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(id)
            .do_update()
            .set(self)
            .filter(
                processable_id
                    .ne(excluded(processable_id))
                    .or(instrument_id.ne(excluded(instrument_id)))
                    .or(created_by.ne(excluded(created_by)))
                    .or(created_at.ne(excluded(created_at))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
    for crate::codegen::structs_codegen::tables::ball_mill_steps::BallMillStep
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl,
            upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::ball_mill_steps::ball_mill_steps::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(id)
            .do_update()
            .set(self)
            .filter(
                processable_id
                    .ne(excluded(processable_id))
                    .or(instrument_id.ne(excluded(instrument_id)))
                    .or(created_by.ne(excluded(created_by)))
                    .or(created_at.ne(excluded(created_at))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
