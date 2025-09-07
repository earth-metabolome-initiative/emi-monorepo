#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection> for crate::StorageProcedure {
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl,
            upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::storage_procedures::storage_procedures::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure)
            .do_update()
            .set(self)
            .filter(
                procedure_template
                    .ne(excluded(procedure_template))
                    .or(stored_asset.ne(excluded(stored_asset)))
                    .or(stored_asset_model.ne(excluded(stored_asset_model)))
                    .or(procedure_template_stored_asset_model
                        .ne(excluded(procedure_template_stored_asset_model)))
                    .or(procedure_stored_asset.ne(excluded(procedure_stored_asset)))
                    .or(stored_into.ne(excluded(stored_into)))
                    .or(stored_into_model.ne(excluded(stored_into_model)))
                    .or(procedure_template_stored_into_model
                        .ne(excluded(procedure_template_stored_into_model)))
                    .or(procedure_stored_into.ne(excluded(procedure_stored_into))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection> for crate::StorageProcedure {
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl,
            upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::storage_procedures::storage_procedures::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure)
            .do_update()
            .set(self)
            .filter(
                procedure_template
                    .ne(excluded(procedure_template))
                    .or(stored_asset.ne(excluded(stored_asset)))
                    .or(stored_asset_model.ne(excluded(stored_asset_model)))
                    .or(procedure_template_stored_asset_model
                        .ne(excluded(procedure_template_stored_asset_model)))
                    .or(procedure_stored_asset.ne(excluded(procedure_stored_asset)))
                    .or(stored_into.ne(excluded(stored_into)))
                    .or(stored_into_model.ne(excluded(stored_into_model)))
                    .or(procedure_template_stored_into_model
                        .ne(excluded(procedure_template_stored_into_model)))
                    .or(procedure_stored_into.ne(excluded(procedure_stored_into))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
