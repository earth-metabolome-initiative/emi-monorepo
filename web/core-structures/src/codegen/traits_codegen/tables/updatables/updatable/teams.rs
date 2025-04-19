#[cfg(feature = "postgres")]
impl web_common_traits::database::Updatable
    for crate::codegen::structs_codegen::tables::teams::Team
{
    type UserId = i32;
    type Conn = diesel_async::AsyncPgConnection;
    async fn can_update(
        &self,
        user_id: &Self::UserId,
        conn: &mut Self::Conn,
    ) -> Result<bool, diesel::result::Error> {
        if *user_id == self.created_by {
            return Ok(true);
        }
        if *user_id == self.updated_by {
            return Ok(true);
        }
        if let Some(teams) = self.parent_team(conn).await? {
            if !std::boxed::Box::pin(teams.can_update(user_id, conn)).await? {
                return Ok(false);
            }
        }
        Ok(true)
    }
}
