#[cfg(feature = "postgres")]
impl web_common_traits::database::Updatable
    for crate::codegen::structs_codegen::tables::aliquoting_step_models::AliquotingStepModel
{
    type UserId = i32;
    type Conn = diesel_async::AsyncPgConnection;
    async fn can_update(
        &self,
        user_id: &Self::UserId,
        _conn: &mut Self::Conn,
    ) -> Result<bool, diesel::result::Error> {
        if *user_id == self.created_by {
            return Ok(true);
        }
        if *user_id == self.updated_by {
            return Ok(true);
        }
        Ok(true)
    }
}
