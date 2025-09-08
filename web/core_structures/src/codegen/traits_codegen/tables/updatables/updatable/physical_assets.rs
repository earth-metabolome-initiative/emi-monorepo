impl<C: diesel::connection::LoadConnection> web_common_traits::database::Updatable<C>
    for crate::PhysicalAsset
where
    crate::Asset: web_common_traits::database::Read<C>,
    crate::Asset: web_common_traits::database::Updatable<C, UserId = i32>,
    crate::PhysicalAssetModel: web_common_traits::database::Read<C>,
    crate::PhysicalAssetModel: web_common_traits::database::Updatable<C, UserId = i32>,
{
    type UserId = i32;
    fn can_update(
        &self,
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<bool, diesel::result::Error> {
        if !self.id(conn)?.can_update(user_id, conn)? {
            return Ok(false);
        }
        if !self.model(conn)?.can_update(user_id, conn)? {
            return Ok(false);
        }
        Ok(true)
    }
}
