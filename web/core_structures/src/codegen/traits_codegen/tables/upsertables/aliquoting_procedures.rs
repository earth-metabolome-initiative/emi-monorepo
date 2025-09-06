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
            .on_conflict(procedure)
            .do_update()
            .set(self)
            .filter(
                procedure_template
                    .ne(excluded(procedure_template))
                    .or(aliquoted_with.ne(excluded(aliquoted_with)))
                    .or(aliquoted_with_model.ne(excluded(aliquoted_with_model)))
                    .or(procedure_template_aliquoted_with_model
                        .ne(excluded(procedure_template_aliquoted_with_model)))
                    .or(procedure_aliquoted_with.ne(excluded(procedure_aliquoted_with)))
                    .or(pipette_tip_model.ne(excluded(pipette_tip_model)))
                    .or(procedure_template_pipette_tip_model
                        .ne(excluded(procedure_template_pipette_tip_model)))
                    .or(procedure_pipette_tip.ne(excluded(procedure_pipette_tip)))
                    .or(aliquoted_from.ne(excluded(aliquoted_from)))
                    .or(procedure_template_aliquoted_from_model
                        .ne(excluded(procedure_template_aliquoted_from_model)))
                    .or(procedure_aliquoted_from.ne(excluded(procedure_aliquoted_from)))
                    .or(aliquoted_into.ne(excluded(aliquoted_into)))
                    .or(procedure_template_aliquoted_into_model
                        .ne(excluded(procedure_template_aliquoted_into_model)))
                    .or(procedure_aliquoted_into.ne(excluded(procedure_aliquoted_into))),
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
            .on_conflict(procedure)
            .do_update()
            .set(self)
            .filter(
                procedure_template
                    .ne(excluded(procedure_template))
                    .or(aliquoted_with.ne(excluded(aliquoted_with)))
                    .or(aliquoted_with_model.ne(excluded(aliquoted_with_model)))
                    .or(procedure_template_aliquoted_with_model
                        .ne(excluded(procedure_template_aliquoted_with_model)))
                    .or(procedure_aliquoted_with.ne(excluded(procedure_aliquoted_with)))
                    .or(pipette_tip_model.ne(excluded(pipette_tip_model)))
                    .or(procedure_template_pipette_tip_model
                        .ne(excluded(procedure_template_pipette_tip_model)))
                    .or(procedure_pipette_tip.ne(excluded(procedure_pipette_tip)))
                    .or(aliquoted_from.ne(excluded(aliquoted_from)))
                    .or(procedure_template_aliquoted_from_model
                        .ne(excluded(procedure_template_aliquoted_from_model)))
                    .or(procedure_aliquoted_from.ne(excluded(procedure_aliquoted_from)))
                    .or(aliquoted_into.ne(excluded(aliquoted_into)))
                    .or(procedure_template_aliquoted_into_model
                        .ne(excluded(procedure_template_aliquoted_into_model)))
                    .or(procedure_aliquoted_into.ne(excluded(procedure_aliquoted_into))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
