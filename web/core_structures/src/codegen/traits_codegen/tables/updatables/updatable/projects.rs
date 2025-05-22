#[cfg(feature = "postgres")]
impl web_common_traits::database::Updatable
    for crate::codegen::structs_codegen::tables::projects::Project
{
    type UserId = i32;
    type Conn = diesel_async::AsyncPgConnection;
    async fn can_update(
        &self,
        user_id: &Self::UserId,
        conn: &mut Self::Conn,
    ) -> Result<bool, diesel::result::Error> {
        use diesel::{BoolExpressionMethods, ExpressionMethods, JoinOnDsl, QueryDsl};
        use diesel_async::RunQueryDsl;
        if *user_id == self.created_by {
            return Ok(true);
        }
        if *user_id == self.updated_by {
            return Ok(true);
        }
        if crate::codegen::diesel_codegen::tables::team_members::team_members::table
            .inner_join(
                crate::codegen::diesel_codegen::tables::team_projects::team_projects::table
                    .on(
                        crate::codegen::diesel_codegen::tables::team_members::team_members::team_id
                            .eq(
                                crate::codegen::diesel_codegen::tables::team_projects::team_projects::team_id,
                            ),
                    ),
            )
            .filter(
                crate::codegen::diesel_codegen::tables::team_members::team_members::member_id
                    .eq(user_id)
                    .and(
                        crate::codegen::diesel_codegen::tables::team_projects::team_projects::project_id
                            .eq(self.id),
                    ),
            )
            .count()
            .get_result::<i64>(conn)
            .await? > 0
        {
            return Ok(true);
        }
        if let Some(projects) = self.parent_project(conn).await? {
            if !std::boxed::Box::pin(projects.can_update(user_id, conn)).await? {
                return Ok(false);
            }
        }
        Ok(true)
    }
}
