#[cfg(feature = "postgres")]
impl web_common_traits::database::Updatable
for crate::codegen::structs_codegen::tables::step_model_trackable_categories::StepModelTrackableCategory {
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
        if !self.id(conn).await?.can_update(user_id, conn).await? {
            return Ok(false);
        }
        if !self.trackable_category(conn).await?.can_update(user_id, conn).await? {
            return Ok(false);
        }
        Ok(true)
    }
}
