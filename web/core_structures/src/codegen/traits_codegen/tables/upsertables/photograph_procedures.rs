#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::photograph_procedures::PhotographProcedure
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl,
            upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure)
            .do_update()
            .set(self)
            .filter(
                procedure_template
                    .ne(excluded(procedure_template))
                    .or(photographed_asset.ne(excluded(photographed_asset)))
                    .or(procedure_template_photographed_asset_model
                        .ne(excluded(procedure_template_photographed_asset_model)))
                    .or(procedure_photographed_asset.ne(excluded(procedure_photographed_asset)))
                    .or(photographed_with.ne(excluded(photographed_with)))
                    .or(procedure_template_photographed_with_model
                        .ne(excluded(procedure_template_photographed_with_model)))
                    .or(procedure_photographed_with.ne(excluded(procedure_photographed_with))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
    for crate::codegen::structs_codegen::tables::photograph_procedures::PhotographProcedure
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl,
            upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::photograph_procedures::photograph_procedures::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure)
            .do_update()
            .set(self)
            .filter(
                procedure_template
                    .ne(excluded(procedure_template))
                    .or(photographed_asset.ne(excluded(photographed_asset)))
                    .or(procedure_template_photographed_asset_model
                        .ne(excluded(procedure_template_photographed_asset_model)))
                    .or(procedure_photographed_asset.ne(excluded(procedure_photographed_asset)))
                    .or(photographed_with.ne(excluded(photographed_with)))
                    .or(procedure_template_photographed_with_model
                        .ne(excluded(procedure_template_photographed_with_model)))
                    .or(procedure_photographed_with.ne(excluded(procedure_photographed_with))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
