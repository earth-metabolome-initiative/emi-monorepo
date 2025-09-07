impl<C: diesel::connection::LoadConnection> web_common_traits::database::Updatable<C>
for crate::PhoneModel
where
    crate::CameraModel: diesel::Identifiable,
    <crate::CameraModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
        <crate::CameraModel as diesel::Identifiable>::Id,
    >,
    <<crate::CameraModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::CameraModel as diesel::Identifiable>::Id,
    >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
    <<<crate::CameraModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::CameraModel as diesel::Identifiable>::Id,
    >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
        'a,
        C,
        crate::CameraModel,
    >,
    crate::CameraModel: web_common_traits::database::Updatable<C, UserId = i32>,
    crate::PositioningDeviceModel: diesel::Identifiable,
    <crate::PositioningDeviceModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
        <crate::PositioningDeviceModel as diesel::Identifiable>::Id,
    >,
    <<crate::PositioningDeviceModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::PositioningDeviceModel as diesel::Identifiable>::Id,
    >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
    <<<crate::PositioningDeviceModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::PositioningDeviceModel as diesel::Identifiable>::Id,
    >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
        'a,
        C,
        crate::PositioningDeviceModel,
    >,
    crate::PositioningDeviceModel: web_common_traits::database::Updatable<
        C,
        UserId = i32,
    >,
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
