#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::packaging_step_models::PackagingStepModel
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl,
            upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::packaging_step_models::packaging_step_models::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(id)
            .do_update()
            .set(self)
            .filter(
                packaging_model_id
                    .ne(excluded(packaging_model_id))
                    .or(created_at.ne(excluded(created_at)))
                    .or(updated_at.ne(excluded(updated_at)))
                    .or(created_by.ne(excluded(created_by)))
                    .or(updated_by.ne(excluded(updated_by))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
    for crate::codegen::structs_codegen::tables::packaging_step_models::PackagingStepModel
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl,
            upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::packaging_step_models::packaging_step_models::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(id)
            .do_update()
            .set(self)
            .filter(
                packaging_model_id
                    .ne(excluded(packaging_model_id))
                    .or(created_at.ne(excluded(created_at)))
                    .or(updated_at.ne(excluded(updated_at)))
                    .or(created_by.ne(excluded(created_by)))
                    .or(updated_by.ne(excluded(updated_by))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
