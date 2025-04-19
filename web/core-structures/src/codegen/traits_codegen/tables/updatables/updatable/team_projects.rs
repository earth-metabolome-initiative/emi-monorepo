#[cfg(feature = "postgres")]
impl web_common_traits::database::Updatable
    for crate::codegen::structs_codegen::tables::team_projects::TeamProject
{
    type UserId = i32;
    type Conn = diesel_async::AsyncPgConnection;
    async fn can_update(
        &self,
        user_id: &Self::UserId,
        conn: &mut Self::Conn,
    ) -> Result<bool, diesel::result::Error> {
        if !self.team(conn).await?.can_update(user_id, conn).await? {
            return Ok(false);
        }
        if !self.project(conn).await?.can_update(user_id, conn).await? {
            return Ok(false);
        }
        Ok(true)
    }
}
