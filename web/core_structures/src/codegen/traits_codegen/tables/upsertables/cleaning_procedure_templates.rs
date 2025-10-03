#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
for crate::codegen::structs_codegen::tables::cleaning_procedure_templates::CleaningProcedureTemplate {
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        use diesel::BoolExpressionMethods;
        use diesel::RunQueryDsl;
        use crate::codegen::diesel_codegen::tables::cleaning_procedure_templates::cleaning_procedure_templates::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure_template)
            .do_update()
            .set(self)
            .filter(
                cleaned_with_model
                    .ne(excluded(cleaned_with_model))
                    .or(
                        procedure_template_cleaned_with_model
                            .ne(excluded(procedure_template_cleaned_with_model)),
                    )
                    .or(cleaned_model.ne(excluded(cleaned_model)))
                    .or(
                        procedure_template_cleaned_model
                            .ne(excluded(procedure_template_cleaned_model)),
                    )
                    .or(liters.ne(excluded(liters))),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
for crate::codegen::structs_codegen::tables::cleaning_procedure_templates::CleaningProcedureTemplate {
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        use diesel::BoolExpressionMethods;
        use diesel::RunQueryDsl;
        use crate::codegen::diesel_codegen::tables::cleaning_procedure_templates::cleaning_procedure_templates::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure_template)
            .do_update()
            .set(self)
            .filter(
                cleaned_with_model
                    .ne(excluded(cleaned_with_model))
                    .or(
                        procedure_template_cleaned_with_model
                            .ne(excluded(procedure_template_cleaned_with_model)),
                    )
                    .or(cleaned_model.ne(excluded(cleaned_model)))
                    .or(
                        procedure_template_cleaned_model
                            .ne(excluded(procedure_template_cleaned_model)),
                    )
                    .or(liters.ne(excluded(liters))),
            )
            .get_results(conn)
            .map(|mut result| { result.pop() })
    }
}
