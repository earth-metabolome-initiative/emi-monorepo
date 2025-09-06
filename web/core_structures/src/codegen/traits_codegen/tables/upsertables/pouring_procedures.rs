#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::pouring_procedures::PouringProcedure
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl,
            upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::pouring_procedures::pouring_procedures::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure)
            .do_update()
            .set(self)
            .filter(
                procedure_template
                    .ne(excluded(procedure_template))
                    .or(poured_from.ne(excluded(poured_from)))
                    .or(procedure_template_poured_from_model
                        .ne(excluded(procedure_template_poured_from_model)))
                    .or(procedure_poured_from.ne(excluded(procedure_poured_from)))
                    .or(measured_with.ne(excluded(measured_with)))
                    .or(procedure_template_measured_with_model
                        .ne(excluded(procedure_template_measured_with_model)))
                    .or(procedure_measured_with.ne(excluded(procedure_measured_with)))
                    .or(poured_into.ne(excluded(poured_into)))
                    .or(procedure_template_poured_into_model
                        .ne(excluded(procedure_template_poured_into_model)))
                    .or(procedure_poured_into.ne(excluded(procedure_poured_into))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
    for crate::codegen::structs_codegen::tables::pouring_procedures::PouringProcedure
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl,
            upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::pouring_procedures::pouring_procedures::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure)
            .do_update()
            .set(self)
            .filter(
                procedure_template
                    .ne(excluded(procedure_template))
                    .or(poured_from.ne(excluded(poured_from)))
                    .or(procedure_template_poured_from_model
                        .ne(excluded(procedure_template_poured_from_model)))
                    .or(procedure_poured_from.ne(excluded(procedure_poured_from)))
                    .or(measured_with.ne(excluded(measured_with)))
                    .or(procedure_template_measured_with_model
                        .ne(excluded(procedure_template_measured_with_model)))
                    .or(procedure_measured_with.ne(excluded(procedure_measured_with)))
                    .or(poured_into.ne(excluded(poured_into)))
                    .or(procedure_template_poured_into_model
                        .ne(excluded(procedure_template_poured_into_model)))
                    .or(procedure_poured_into.ne(excluded(procedure_poured_into))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
