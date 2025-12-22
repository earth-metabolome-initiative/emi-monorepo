#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::harvesting_procedures::HarvestingProcedure
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl,
            upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::harvesting_procedures::harvesting_procedures::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure)
            .do_update()
            .set(self)
            .filter(
                procedure_template
                    .ne(excluded(procedure_template_id))
                    .or(sample_source.ne(excluded(sample_source)))
                    .or(procedure_template_sample_source_model
                        .ne(excluded(procedure_template_sample_source_model)))
                    .or(procedure_sample_source.ne(excluded(procedure_sample_source)))
                    .or(sample.ne(excluded(sample)))
                    .or(procedure_template_sample_model
                        .ne(excluded(procedure_template_sample_model)))
                    .or(procedure_sample.ne(excluded(procedure_sample))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
    for crate::codegen::structs_codegen::tables::harvesting_procedures::HarvestingProcedure
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl,
            upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::harvesting_procedures::harvesting_procedures::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure)
            .do_update()
            .set(self)
            .filter(
                procedure_template
                    .ne(excluded(procedure_template_id))
                    .or(sample_source.ne(excluded(sample_source)))
                    .or(procedure_template_sample_source_model
                        .ne(excluded(procedure_template_sample_source_model)))
                    .or(procedure_sample_source.ne(excluded(procedure_sample_source)))
                    .or(sample.ne(excluded(sample)))
                    .or(procedure_template_sample_model
                        .ne(excluded(procedure_template_sample_model)))
                    .or(procedure_sample.ne(excluded(procedure_sample))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
