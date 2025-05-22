#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::project_workflow_models::ProjectWorkflowModel
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
        use crate::codegen::diesel_codegen::tables::project_workflow_models::project_workflow_models::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(id)
            .do_update()
            .set(self)
            .filter(
                name.ne(excluded(name))
                    .or(description.ne(excluded(description)))
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
    for crate::codegen::structs_codegen::tables::project_workflow_models::ProjectWorkflowModel
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
        use crate::codegen::diesel_codegen::tables::project_workflow_models::project_workflow_models::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(id)
            .do_update()
            .set(self)
            .filter(
                name.ne(excluded(name))
                    .or(description.ne(excluded(description)))
                    .or(created_at.ne(excluded(created_at)))
                    .or(updated_at.ne(excluded(updated_at)))
                    .or(created_by.ne(excluded(created_by)))
                    .or(updated_by.ne(excluded(updated_by))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
