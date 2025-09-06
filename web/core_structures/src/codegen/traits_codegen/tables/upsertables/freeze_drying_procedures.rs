#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::freeze_drying_procedures::FreezeDryingProcedure
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        use diesel::BoolExpressionMethods;
        use diesel::RunQueryDsl;
        use crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure)
            .do_update()
            .set(self)
            .filter(
                procedure_template
                    .ne(excluded(procedure_template))
                    .or(freeze_dried_container.ne(excluded(freeze_dried_container)))
                    .or(freeze_dried_container_model.ne(excluded(freeze_dried_container_model)))
                    .or(procedure_template_freeze_dried_container_model
                        .ne(excluded(procedure_template_freeze_dried_container_model)))
                    .or(procedure_freeze_dried_container
                        .ne(excluded(procedure_freeze_dried_container)))
                    .or(freeze_dried_with.ne(excluded(freeze_dried_with)))
                    .or(freeze_dried_with_model.ne(excluded(freeze_dried_with_model)))
                    .or(procedure_template_freeze_dried_with_model
                        .ne(excluded(procedure_template_freeze_dried_with_model)))
                    .or(procedure_freeze_dried_with.ne(excluded(procedure_freeze_dried_with))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
    for crate::codegen::structs_codegen::tables::freeze_drying_procedures::FreezeDryingProcedure
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        use diesel::BoolExpressionMethods;
        use diesel::RunQueryDsl;
        use crate::codegen::diesel_codegen::tables::freeze_drying_procedures::freeze_drying_procedures::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure)
            .do_update()
            .set(self)
            .filter(
                procedure_template
                    .ne(excluded(procedure_template))
                    .or(freeze_dried_container.ne(excluded(freeze_dried_container)))
                    .or(freeze_dried_container_model.ne(excluded(freeze_dried_container_model)))
                    .or(procedure_template_freeze_dried_container_model
                        .ne(excluded(procedure_template_freeze_dried_container_model)))
                    .or(procedure_freeze_dried_container
                        .ne(excluded(procedure_freeze_dried_container)))
                    .or(freeze_dried_with.ne(excluded(freeze_dried_with)))
                    .or(freeze_dried_with_model.ne(excluded(freeze_dried_with_model)))
                    .or(procedure_template_freeze_dried_with_model
                        .ne(excluded(procedure_template_freeze_dried_with_model)))
                    .or(procedure_freeze_dried_with.ne(excluded(procedure_freeze_dried_with))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
