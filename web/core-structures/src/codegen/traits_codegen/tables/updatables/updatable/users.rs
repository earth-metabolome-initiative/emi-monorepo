#[cfg(feature = "postgres")]
impl web_common_traits::database::Updatable
    for crate::codegen::structs_codegen::tables::users::User
{
    type UserId = i32;
    type Conn = diesel_async::AsyncPgConnection;
    async fn can_update(
        &self,
        user_id: &Self::UserId,
        _conn: &mut Self::Conn,
    ) -> Result<bool, diesel::result::Error> {
        if *user_id == self.id {
            return Ok(true);
        }
        Ok(true)
    }
}
