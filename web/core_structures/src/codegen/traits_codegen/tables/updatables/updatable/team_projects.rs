impl<C: diesel::connection::LoadConnection> web_common_traits::database::Updatable<C>
    for crate::TeamProject
where
    crate::Project: web_common_traits::database::Read<C>,
    crate::Project: web_common_traits::database::Updatable<C, UserId = i32>,
    crate::Team: web_common_traits::database::Read<C>,
    crate::Team: web_common_traits::database::Updatable<C, UserId = i32>,
{
    type UserId = i32;
    fn can_update(
        &self,
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<bool, diesel::result::Error> {
        if !self.project(conn)?.can_update(user_id, conn)? {
            return Ok(false);
        }
        if !self.team(conn)?.can_update(user_id, conn)? {
            return Ok(false);
        }
        Ok(true)
    }
}
