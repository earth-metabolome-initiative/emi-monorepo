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
                    .or(foreign_procedure_template.ne(excluded(foreign_procedure_template)))
                    .or(foreign_procedure.ne(excluded(foreign_procedure)))
                    .or(bead_model.ne(excluded(bead_model)))
                    .or(milled_with_model.ne(excluded(milled_with_model)))
                    .or(milled_with.ne(excluded(milled_with)))
                    .or(milled_container.ne(excluded(milled_container))),
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
                    .or(foreign_procedure_template.ne(excluded(foreign_procedure_template)))
                    .or(foreign_procedure.ne(excluded(foreign_procedure)))
                    .or(bead_model.ne(excluded(bead_model)))
                    .or(milled_with_model.ne(excluded(milled_with_model)))
                    .or(milled_with.ne(excluded(milled_with)))
                    .or(milled_container.ne(excluded(milled_container))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
