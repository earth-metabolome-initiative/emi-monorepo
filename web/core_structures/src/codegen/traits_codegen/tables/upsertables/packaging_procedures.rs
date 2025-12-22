#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::packaging_procedures::PackagingProcedure
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl,
            upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::packaging_procedures::packaging_procedures::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure)
            .do_update()
            .set(self)
            .filter(
                procedure_template
                    .ne(excluded(procedure_template_id))
                    .or(sample.ne(excluded(sample)))
                    .or(sample_model.ne(excluded(sample_model)))
                    .or(procedure_template_sample_model
                        .ne(excluded(procedure_template_sample_model)))
                    .or(procedure_sample.ne(excluded(procedure_sample)))
                    .or(packaged_with_model.ne(excluded(packaged_with_model)))
                    .or(procedure_template_packaged_with_model
                        .ne(excluded(procedure_template_packaged_with_model)))
                    .or(procedure_packaged_with.ne(excluded(procedure_packaged_with))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
    for crate::codegen::structs_codegen::tables::packaging_procedures::PackagingProcedure
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl,
            upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::packaging_procedures::packaging_procedures::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure)
            .do_update()
            .set(self)
            .filter(
                procedure_template
                    .ne(excluded(procedure_template_id))
                    .or(sample.ne(excluded(sample)))
                    .or(sample_model.ne(excluded(sample_model)))
                    .or(procedure_template_sample_model
                        .ne(excluded(procedure_template_sample_model)))
                    .or(procedure_sample.ne(excluded(procedure_sample)))
                    .or(packaged_with_model.ne(excluded(packaged_with_model)))
                    .or(procedure_template_packaged_with_model
                        .ne(excluded(procedure_template_packaged_with_model)))
                    .or(procedure_packaged_with.ne(excluded(procedure_packaged_with))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
