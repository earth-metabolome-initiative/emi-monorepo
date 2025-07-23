#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::aliquoting_procedures::AliquotingProcedure
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl,
            upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::aliquoting_procedures::aliquoting_procedures::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure_id)
            .do_update()
            .set(self)
            .filter(
                procedure_model_id
                    .ne(excluded(procedure_model_id))
                    .or(aliquoted_with.ne(excluded(aliquoted_with)))
                    .or(pipette_tip.ne(excluded(pipette_tip)))
                    .or(aliquoted_container_id.ne(excluded(aliquoted_container_id))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
    for crate::codegen::structs_codegen::tables::aliquoting_procedures::AliquotingProcedure
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl,
            upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::aliquoting_procedures::aliquoting_procedures::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure_id)
            .do_update()
            .set(self)
            .filter(
                procedure_model_id
                    .ne(excluded(procedure_model_id))
                    .or(aliquoted_with.ne(excluded(aliquoted_with)))
                    .or(pipette_tip.ne(excluded(pipette_tip)))
                    .or(aliquoted_container_id.ne(excluded(aliquoted_container_id))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
