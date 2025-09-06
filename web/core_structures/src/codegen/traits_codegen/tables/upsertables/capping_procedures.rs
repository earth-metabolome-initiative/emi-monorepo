#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::capping_procedures::CappingProcedure
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl,
            upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::capping_procedures::capping_procedures::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure)
            .do_update()
            .set(self)
            .filter(
                procedure_template
                    .ne(excluded(procedure_template))
                    .or(capped_container.ne(excluded(capped_container)))
                    .or(capped_container_model.ne(excluded(capped_container_model)))
                    .or(procedure_template_capped_container_model
                        .ne(excluded(procedure_template_capped_container_model)))
                    .or(procedure_capped_container.ne(excluded(procedure_capped_container)))
                    .or(capped_with_model.ne(excluded(capped_with_model)))
                    .or(procedure_template_capped_with_model
                        .ne(excluded(procedure_template_capped_with_model)))
                    .or(procedure_capped_with.ne(excluded(procedure_capped_with))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
    for crate::codegen::structs_codegen::tables::capping_procedures::CappingProcedure
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl,
            upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::capping_procedures::capping_procedures::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure)
            .do_update()
            .set(self)
            .filter(
                procedure_template
                    .ne(excluded(procedure_template))
                    .or(capped_container.ne(excluded(capped_container)))
                    .or(capped_container_model.ne(excluded(capped_container_model)))
                    .or(procedure_template_capped_container_model
                        .ne(excluded(procedure_template_capped_container_model)))
                    .or(procedure_capped_container.ne(excluded(procedure_capped_container)))
                    .or(capped_with_model.ne(excluded(capped_with_model)))
                    .or(procedure_template_capped_with_model
                        .ne(excluded(procedure_template_capped_with_model)))
                    .or(procedure_capped_with.ne(excluded(procedure_capped_with))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
