#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::sampling_procedures::SamplingProcedure
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl,
            upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::sampling_procedures::sampling_procedures::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure_id)
            .do_update()
            .set(self)
            .filter(
                processable_id
                    .ne(excluded(processable_id))
                    .or(trackable_id.ne(excluded(trackable_id))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
    for crate::codegen::structs_codegen::tables::sampling_procedures::SamplingProcedure
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl,
            upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::sampling_procedures::sampling_procedures::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure_id)
            .do_update()
            .set(self)
            .filter(
                processable_id
                    .ne(excluded(processable_id))
                    .or(trackable_id.ne(excluded(trackable_id))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
