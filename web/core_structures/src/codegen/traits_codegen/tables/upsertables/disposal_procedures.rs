#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection> for crate::DisposalProcedure {
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl,
            upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::disposal_procedures::disposal_procedures::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure)
            .do_update()
            .set(self)
            .filter(
                procedure_template
                    .ne(excluded(procedure_template))
                    .or(disposed_asset.ne(excluded(disposed_asset)))
                    .or(procedure_template_disposed_asset_model
                        .ne(excluded(procedure_template_disposed_asset_model)))
                    .or(procedure_disposed_asset.ne(excluded(procedure_disposed_asset))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection> for crate::DisposalProcedure {
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl,
            upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::disposal_procedures::disposal_procedures::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure)
            .do_update()
            .set(self)
            .filter(
                procedure_template
                    .ne(excluded(procedure_template))
                    .or(disposed_asset.ne(excluded(disposed_asset)))
                    .or(procedure_template_disposed_asset_model
                        .ne(excluded(procedure_template_disposed_asset_model)))
                    .or(procedure_disposed_asset.ne(excluded(procedure_disposed_asset))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
