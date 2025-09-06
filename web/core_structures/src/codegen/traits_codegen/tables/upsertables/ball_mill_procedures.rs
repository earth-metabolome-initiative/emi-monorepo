#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::ball_mill_procedures::BallMillProcedure
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl,
            upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::ball_mill_procedures::ball_mill_procedures::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure)
            .do_update()
            .set(self)
            .filter(
                procedure_template
                    .ne(excluded(procedure_template))
                    .or(bead_model.ne(excluded(bead_model)))
                    .or(procedure_template_bead_model.ne(excluded(procedure_template_bead_model)))
                    .or(procedure_bead.ne(excluded(procedure_bead)))
                    .or(milled_with_model.ne(excluded(milled_with_model)))
                    .or(procedure_template_milled_with_model
                        .ne(excluded(procedure_template_milled_with_model)))
                    .or(procedure_milled_with.ne(excluded(procedure_milled_with)))
                    .or(milled_with.ne(excluded(milled_with)))
                    .or(milled_container.ne(excluded(milled_container)))
                    .or(milled_container_model.ne(excluded(milled_container_model)))
                    .or(procedure_template_milled_container_model
                        .ne(excluded(procedure_template_milled_container_model)))
                    .or(procedure_milled_container.ne(excluded(procedure_milled_container))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
    for crate::codegen::structs_codegen::tables::ball_mill_procedures::BallMillProcedure
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl,
            upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::ball_mill_procedures::ball_mill_procedures::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure)
            .do_update()
            .set(self)
            .filter(
                procedure_template
                    .ne(excluded(procedure_template))
                    .or(bead_model.ne(excluded(bead_model)))
                    .or(procedure_template_bead_model.ne(excluded(procedure_template_bead_model)))
                    .or(procedure_bead.ne(excluded(procedure_bead)))
                    .or(milled_with_model.ne(excluded(milled_with_model)))
                    .or(procedure_template_milled_with_model
                        .ne(excluded(procedure_template_milled_with_model)))
                    .or(procedure_milled_with.ne(excluded(procedure_milled_with)))
                    .or(milled_with.ne(excluded(milled_with)))
                    .or(milled_container.ne(excluded(milled_container)))
                    .or(milled_container_model.ne(excluded(milled_container_model)))
                    .or(procedure_template_milled_container_model
                        .ne(excluded(procedure_template_milled_container_model)))
                    .or(procedure_milled_container.ne(excluded(procedure_milled_container))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
