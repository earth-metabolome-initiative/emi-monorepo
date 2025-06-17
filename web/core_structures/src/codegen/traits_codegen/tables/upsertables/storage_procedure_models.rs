#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::storage_procedure_models::StorageProcedureModel
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        use diesel::BoolExpressionMethods;
        use diesel::RunQueryDsl;
        use crate::codegen::diesel_codegen::tables::storage_procedure_models::storage_procedure_models::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure_model_id)
            .do_update()
            .set(self)
            .filter(
                child_container_id
                    .ne(excluded(child_container_id))
                    .or(procedure_child_container_id.ne(excluded(procedure_child_container_id)))
                    .or(parent_container_id.ne(excluded(parent_container_id)))
                    .or(procedure_parent_container_id.ne(excluded(procedure_parent_container_id))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
    for crate::codegen::structs_codegen::tables::storage_procedure_models::StorageProcedureModel
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::ExpressionMethods;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        use diesel::BoolExpressionMethods;
        use diesel::RunQueryDsl;
        use crate::codegen::diesel_codegen::tables::storage_procedure_models::storage_procedure_models::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(procedure_model_id)
            .do_update()
            .set(self)
            .filter(
                child_container_id
                    .ne(excluded(child_container_id))
                    .or(procedure_child_container_id.ne(excluded(procedure_child_container_id)))
                    .or(parent_container_id.ne(excluded(parent_container_id)))
                    .or(procedure_parent_container_id.ne(excluded(procedure_parent_container_id))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
