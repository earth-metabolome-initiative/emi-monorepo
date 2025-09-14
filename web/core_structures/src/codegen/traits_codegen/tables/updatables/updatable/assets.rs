impl<C: diesel::connection::LoadConnection> web_common_traits::database::Updatable<C>
    for crate::codegen::structs_codegen::tables::assets::Asset
{
    type UserId = i32;
    fn can_update(
        &self,
        user_id: Self::UserId,
        _conn: &mut C,
    ) -> Result<bool, diesel::result::Error> {
        if user_id == self.created_by {
            return Ok(true);
        }
        if user_id == self.updated_by {
            return Ok(true);
        }
        Ok(true)
    }
}
