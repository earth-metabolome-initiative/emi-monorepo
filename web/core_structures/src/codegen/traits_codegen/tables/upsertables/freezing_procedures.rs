#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::freezing_procedures::FreezingProcedure
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl,
            upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::freezing_procedures::freezing_procedures::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure)
            .do_update()
            .set(self)
            .filter(
                procedure_template
                    .ne(excluded(procedure_template_id))
                    .or(frozen_container.ne(excluded(frozen_container)))
                    .or(frozen_container_model.ne(excluded(frozen_container_model)))
                    .or(procedure_template_frozen_container_model
                        .ne(excluded(procedure_template_frozen_container_model)))
                    .or(procedure_frozen_container.ne(excluded(procedure_frozen_container)))
                    .or(frozen_with.ne(excluded(frozen_with)))
                    .or(frozen_with_model.ne(excluded(frozen_with_model)))
                    .or(procedure_template_frozen_with_model
                        .ne(excluded(procedure_template_frozen_with_model)))
                    .or(procedure_frozen_with.ne(excluded(procedure_frozen_with))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
    for crate::codegen::structs_codegen::tables::freezing_procedures::FreezingProcedure
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl,
            upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::freezing_procedures::freezing_procedures::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure)
            .do_update()
            .set(self)
            .filter(
                procedure_template
                    .ne(excluded(procedure_template_id))
                    .or(frozen_container.ne(excluded(frozen_container)))
                    .or(frozen_container_model.ne(excluded(frozen_container_model)))
                    .or(procedure_template_frozen_container_model
                        .ne(excluded(procedure_template_frozen_container_model)))
                    .or(procedure_frozen_container.ne(excluded(procedure_frozen_container)))
                    .or(frozen_with.ne(excluded(frozen_with)))
                    .or(frozen_with_model.ne(excluded(frozen_with_model)))
                    .or(procedure_template_frozen_with_model
                        .ne(excluded(procedure_template_frozen_with_model)))
                    .or(procedure_frozen_with.ne(excluded(procedure_frozen_with))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
