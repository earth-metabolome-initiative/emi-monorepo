#[cfg(feature = "postgres")]
impl web_common_traits::prelude::Upsertable<diesel::PgConnection>
    for crate::codegen::structs_codegen::tables::teams::Team
{
    fn upsert(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl,
            upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::teams::teams::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(id)
            .do_update()
            .set(self)
            .filter(
                name.ne(excluded(name))
                    .or(description.ne(excluded(description)))
                    .or(icon.ne(excluded(icon)))
                    .or(color_id.ne(excluded(color_id)))
                    .or(state_id.ne(excluded(state_id)))
                    .or(parent_team_id.ne(excluded(parent_team_id)))
                    .or(created_by.ne(excluded(created_by)))
                    .or(created_at.ne(excluded(created_at)))
                    .or(updated_by.ne(excluded(updated_by)))
                    .or(updated_at.ne(excluded(updated_at))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
#[cfg(feature = "sqlite")]
impl web_common_traits::prelude::Upsertable<diesel::SqliteConnection>
    for crate::codegen::structs_codegen::tables::teams::Team
{
    fn upsert(
        &self,
        conn: &mut diesel::SqliteConnection,
    ) -> Result<Option<Self>, diesel::result::Error> {
        use diesel::{
            BoolExpressionMethods, ExpressionMethods, RunQueryDsl, query_dsl::methods::FilterDsl,
            upsert::excluded,
        };

        use crate::codegen::diesel_codegen::tables::teams::teams::*;
        diesel::insert_into(table)
            .values(self)
            .on_conflict(id)
            .do_update()
            .set(self)
            .filter(
                name.ne(excluded(name))
                    .or(description.ne(excluded(description)))
                    .or(icon.ne(excluded(icon)))
                    .or(color_id.ne(excluded(color_id)))
                    .or(state_id.ne(excluded(state_id)))
                    .or(parent_team_id.ne(excluded(parent_team_id)))
                    .or(created_by.ne(excluded(created_by)))
                    .or(created_at.ne(excluded(created_at)))
                    .or(updated_by.ne(excluded(updated_by)))
                    .or(updated_at.ne(excluded(updated_at))),
            )
            .get_results(conn)
            .map(|mut result| result.pop())
    }
}
