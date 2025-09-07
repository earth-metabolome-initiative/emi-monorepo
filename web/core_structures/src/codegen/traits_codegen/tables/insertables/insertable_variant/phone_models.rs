impl<
    C: diesel::connection::LoadConnection,
    CameraModel,
    PositioningDeviceModel,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertablePhoneModelBuilder<
    CameraModel,
    PositioningDeviceModel,
>
where
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization,
    diesel::query_builder::InsertStatement<
        <crate::PhoneModel as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertablePhoneModel as diesel::Insertable<
            <crate::PhoneModel as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<'query, C, crate::PhoneModel>,
    C: diesel::connection::LoadConnection,
    CameraModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
    PositioningDeviceModel: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    crate::CameraModel: diesel::Identifiable
        + web_common_traits::database::Updatable<C, UserId = i32>,
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
    crate::PositioningDeviceModel: diesel::Identifiable
        + web_common_traits::database::Updatable<C, UserId = i32>,
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
    Self: web_common_traits::database::MostConcreteTable,
{
    type Row = crate::PhoneModel;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertablePhoneModel;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::PhoneModelAttribute,
    >;
    type UserId = i32;
    fn insert(
        mut self,
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::Updatable;
        use web_common_traits::database::MostConcreteTable;
        self.set_most_concrete_table("phone_models");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertablePhoneModel = self
            .try_insert(user_id, conn)?;
        if !insertable_struct.phone_models_camera(conn)?.can_update(user_id, conn)? {
            return Err(
                generic_backend_request_errors::GenericBackendRequestError::Unauthorized
                    .into(),
            );
        }
        if !insertable_struct.phone_models_positioning(conn)?.can_update(user_id, conn)?
        {
            return Err(
                generic_backend_request_errors::GenericBackendRequestError::Unauthorized
                    .into(),
            );
        }
        Ok(
            diesel::insert_into(Self::Row::table())
                .values(insertable_struct)
                .get_result(conn)?,
        )
    }
    fn try_insert(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::InsertableVariant, Self::Error> {
        let id = if self.phone_models_camera.is_complete() {
            let id = self
                .phone_models_camera
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::PhoneModelAttribute::Extension(
                        crate::codegen::structs_codegen::tables::insertables::PhoneModelExtensionAttribute::CameraModel(
                            crate::codegen::structs_codegen::tables::insertables::CameraModelAttribute::Id,
                        ),
                    ))
                })?;
            let _ = self
                .phone_models_positioning
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::PhoneModelAttribute::Extension(
                        crate::codegen::structs_codegen::tables::insertables::PhoneModelExtensionAttribute::PositioningDeviceModel(
                            crate::codegen::structs_codegen::tables::insertables::PositioningDeviceModelAttribute::Id,
                        ),
                    ))
                })?;
            id
        } else {
            let id = self
                .phone_models_positioning
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::PhoneModelAttribute::Extension(
                        crate::codegen::structs_codegen::tables::insertables::PhoneModelExtensionAttribute::PositioningDeviceModel(
                            crate::codegen::structs_codegen::tables::insertables::PositioningDeviceModelAttribute::Id,
                        ),
                    ))
                })?;
            let _ = self
                .phone_models_camera
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::PhoneModelAttribute::Extension(
                        crate::codegen::structs_codegen::tables::insertables::PhoneModelExtensionAttribute::CameraModel(
                            crate::codegen::structs_codegen::tables::insertables::CameraModelAttribute::Id,
                        ),
                    ))
                })?;
            id
        };
        Ok(Self::InsertableVariant { id })
    }
}
