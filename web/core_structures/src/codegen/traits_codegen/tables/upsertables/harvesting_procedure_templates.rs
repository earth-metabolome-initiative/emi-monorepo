#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
for crate::codegen::structs_codegen::tables::harvesting_procedure_templates::HarvestingProcedureTemplate {
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        use diesel::BoolExpressionMethods;
        use diesel::RunQueryDsl;
        use crate::codegen::diesel_codegen::tables::harvesting_procedure_templates::harvesting_procedure_templates::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure_template_id)
            .do_update()
            .set(self)
            .filter(
                sample_source_model
                    .ne(excluded(sample_source_model))
                    .or(
                        procedure_template_sample_source_model
                            .ne(excluded(procedure_template_sample_source_model)),
                    )
                    .or(sample_model.ne(excluded(sample_model)))
                    .or(
                        procedure_template_sample_model
                            .ne(excluded(procedure_template_sample_model)),
                    ),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
for crate::codegen::structs_codegen::tables::harvesting_procedure_templates::HarvestingProcedureTemplate {
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        use diesel::BoolExpressionMethods;
        use diesel::RunQueryDsl;
        use crate::codegen::diesel_codegen::tables::harvesting_procedure_templates::harvesting_procedure_templates::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure_template_id)
            .do_update()
            .set(self)
            .filter(
                sample_source_model
                    .ne(excluded(sample_source_model))
                    .or(
                        procedure_template_sample_source_model
                            .ne(excluded(procedure_template_sample_source_model)),
                    )
                    .or(sample_model.ne(excluded(sample_model)))
                    .or(
                        procedure_template_sample_model
                            .ne(excluded(procedure_template_sample_model)),
                    ),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
