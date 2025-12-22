#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
for crate::codegen::structs_codegen::tables::capping_procedure_templates::CappingProcedureTemplate {
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        use diesel::BoolExpressionMethods;
        use diesel::RunQueryDsl;
        use crate::codegen::diesel_codegen::tables::capping_procedure_templates::capping_procedure_templates::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure_template_id)
            .do_update()
            .set(self)
            .filter(
                capped_container_model
                    .ne(excluded(capped_container_model))
                    .or(
                        procedure_template_capped_container_model
                            .ne(excluded(procedure_template_capped_container_model)),
                    )
                    .or(capped_with_model.ne(excluded(capped_with_model)))
                    .or(
                        procedure_template_capped_with_model
                            .ne(excluded(procedure_template_capped_with_model)),
                    ),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
for crate::codegen::structs_codegen::tables::capping_procedure_templates::CappingProcedureTemplate {
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        use diesel::BoolExpressionMethods;
        use diesel::RunQueryDsl;
        use crate::codegen::diesel_codegen::tables::capping_procedure_templates::capping_procedure_templates::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure_template_id)
            .do_update()
            .set(self)
            .filter(
                capped_container_model
                    .ne(excluded(capped_container_model))
                    .or(
                        procedure_template_capped_container_model
                            .ne(excluded(procedure_template_capped_container_model)),
                    )
                    .or(capped_with_model.ne(excluded(capped_with_model)))
                    .or(
                        procedure_template_capped_with_model
                            .ne(excluded(procedure_template_capped_with_model)),
                    ),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
