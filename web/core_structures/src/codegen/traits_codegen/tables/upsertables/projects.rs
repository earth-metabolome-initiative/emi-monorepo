#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::projects::Project
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use crate::codegen::diesel_codegen::tables::projects::projects::*;
        use diesel::BoolExpressionMethods;
        use diesel::ExpressionMethods;
        use diesel::RunQueryDsl;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(id)
            .do_update()
            .set(self)
            .filter(
                name.ne(excluded(name))
                    .or(description.ne(excluded(description)))
                    .or(state_id.ne(excluded(state_id)))
                    .or(icon.ne(excluded(icon)))
                    .or(color_id.ne(excluded(color_id)))
                    .or(parent_project_id.ne(excluded(parent_project_id)))
                    .or(budget.ne(excluded(budget)))
                    .or(expenses.ne(excluded(expenses)))
                    .or(created_by.ne(excluded(created_by)))
                    .or(created_at.ne(excluded(created_at)))
                    .or(updated_by.ne(excluded(updated_by)))
                    .or(updated_at.ne(excluded(updated_at)))
                    .or(expected_end_date.ne(excluded(expected_end_date)))
                    .or(end_date.ne(excluded(end_date))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
    for crate::codegen::structs_codegen::tables::projects::Project
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use crate::codegen::diesel_codegen::tables::projects::projects::*;
        use diesel::BoolExpressionMethods;
        use diesel::ExpressionMethods;
        use diesel::RunQueryDsl;
        use diesel::query_dsl::methods::FilterDsl;
        use diesel::upsert::excluded;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(id)
            .do_update()
            .set(self)
            .filter(
                name.ne(excluded(name))
                    .or(description.ne(excluded(description)))
                    .or(state_id.ne(excluded(state_id)))
                    .or(icon.ne(excluded(icon)))
                    .or(color_id.ne(excluded(color_id)))
                    .or(parent_project_id.ne(excluded(parent_project_id)))
                    .or(budget.ne(excluded(budget)))
                    .or(expenses.ne(excluded(expenses)))
                    .or(created_by.ne(excluded(created_by)))
                    .or(created_at.ne(excluded(created_at)))
                    .or(updated_by.ne(excluded(updated_by)))
                    .or(updated_at.ne(excluded(updated_at)))
                    .or(expected_end_date.ne(excluded(expected_end_date)))
                    .or(end_date.ne(excluded(end_date))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
