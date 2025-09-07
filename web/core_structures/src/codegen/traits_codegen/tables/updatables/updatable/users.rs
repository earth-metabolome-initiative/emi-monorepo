impl<C: diesel::connection::LoadConnection> web_common_traits::database::Updatable<C>
    for crate::User
{
    type UserId = i32;
    fn can_update(
        &self,
        user_id: Self::UserId,
        _conn: &mut C,
    ) -> Result<bool, diesel::result::Error> {
        if user_id == self.id {
            return Ok(true);
        }
        Ok(true)
    }
}
