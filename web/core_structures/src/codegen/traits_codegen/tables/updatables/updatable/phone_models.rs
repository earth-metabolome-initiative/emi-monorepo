impl<C: diesel::connection::LoadConnection> web_common_traits::database::Updatable<C>
for crate::codegen::structs_codegen::tables::phone_models::PhoneModel
where
    crate::codegen::structs_codegen::tables::camera_models::CameraModel: diesel::Identifiable,
    <crate::codegen::structs_codegen::tables::camera_models::CameraModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::camera_models::CameraModel as diesel::Identifiable>::Id,
    >,
    <<crate::codegen::structs_codegen::tables::camera_models::CameraModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::camera_models::CameraModel as diesel::Identifiable>::Id,
    >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
    <<<crate::codegen::structs_codegen::tables::camera_models::CameraModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::camera_models::CameraModel as diesel::Identifiable>::Id,
    >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
        'a,
        C,
        crate::codegen::structs_codegen::tables::camera_models::CameraModel,
    >,
    crate::codegen::structs_codegen::tables::camera_models::CameraModel: web_common_traits::database::Updatable<
        C,
        UserId = i32,
    >,
    crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel: diesel::Identifiable,
    <crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel as diesel::Identifiable>::Id,
    >,
    <<crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel as diesel::Identifiable>::Id,
    >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
    <<<crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel as diesel::Identifiable>::Id,
    >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
        'a,
        C,
        crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel,
    >,
    crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel: web_common_traits::database::Updatable<
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
        if !self.phone_models_id_fkey(conn)?.can_update(user_id, conn)? {
            return Ok(false);
        }
        if !self.phone_models_id_fkey1(conn)?.can_update(user_id, conn)? {
            return Ok(false);
        }
        Ok(true)
    }
}
