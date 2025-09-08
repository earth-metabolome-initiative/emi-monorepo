impl<C: diesel::connection::LoadConnection> web_common_traits::database::Updatable<C>
    for crate::PhoneModel
where
    crate::CameraModel: web_common_traits::database::Read<C>,
    crate::CameraModel: web_common_traits::database::Updatable<C, UserId = i32>,
    crate::PositioningDeviceModel: web_common_traits::database::Read<C>,
    crate::PositioningDeviceModel: web_common_traits::database::Updatable<C, UserId = i32>,
{
    type UserId = i32;
    fn can_update(
        &self,
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<bool, diesel::result::Error> {
        if !self.phone_models_camera(conn)?.can_update(user_id, conn)? {
            return Ok(false);
        }
        if !self.phone_models_positioning(conn)?.can_update(user_id, conn)? {
            return Ok(false);
        }
        Ok(true)
    }
}
