#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::weighing_procedures::WeighingProcedure
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl,
            upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::weighing_procedures::weighing_procedures::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure_id)
            .do_update()
            .set(self)
            .filter(
                procedure_model_id
                    .ne(excluded(procedure_model_id))
                    .or(weighted_with.ne(excluded(weighted_with)))
                    .or(weighted_container_id.ne(excluded(weighted_container_id)))
                    .or(kilograms.ne(excluded(kilograms))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
    for crate::codegen::structs_codegen::tables::weighing_procedures::WeighingProcedure
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl,
            upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::weighing_procedures::weighing_procedures::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure_id)
            .do_update()
            .set(self)
            .filter(
                procedure_model_id
                    .ne(excluded(procedure_model_id))
                    .or(weighted_with.ne(excluded(weighted_with)))
                    .or(weighted_container_id.ne(excluded(weighted_container_id)))
                    .or(kilograms.ne(excluded(kilograms))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
