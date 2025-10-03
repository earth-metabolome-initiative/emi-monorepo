#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::cleaning_procedures::CleaningProcedure
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl,
            upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::cleaning_procedures::cleaning_procedures::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure)
            .do_update()
            .set(self)
            .filter(
                procedure_template
                    .ne(excluded(procedure_template))
                    .or(procedure_template_cleaned_with_model
                        .ne(excluded(procedure_template_cleaned_with_model)))
                    .or(procedure_cleaned_with.ne(excluded(procedure_cleaned_with)))
                    .or(procedure_template_cleaned_model
                        .ne(excluded(procedure_template_cleaned_model)))
                    .or(procedure_cleaned.ne(excluded(procedure_cleaned))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
    for crate::codegen::structs_codegen::tables::cleaning_procedures::CleaningProcedure
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl,
            upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::cleaning_procedures::cleaning_procedures::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure)
            .do_update()
            .set(self)
            .filter(
                procedure_template
                    .ne(excluded(procedure_template))
                    .or(procedure_template_cleaned_with_model
                        .ne(excluded(procedure_template_cleaned_with_model)))
                    .or(procedure_cleaned_with.ne(excluded(procedure_cleaned_with)))
                    .or(procedure_template_cleaned_model
                        .ne(excluded(procedure_template_cleaned_model)))
                    .or(procedure_cleaned.ne(excluded(procedure_cleaned))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
