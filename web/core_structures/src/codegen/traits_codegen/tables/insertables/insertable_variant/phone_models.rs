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
        <crate::codegen::structs_codegen::tables::phone_models::PhoneModel as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertablePhoneModel as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::phone_models::PhoneModel as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::phone_models::PhoneModel,
    >,
    C: diesel::connection::LoadConnection,
    CameraModel: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
    PositioningDeviceModel: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
{
    type Row = crate::codegen::structs_codegen::tables::phone_models::PhoneModel;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertablePhoneModel;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::InsertablePhoneModelAttributes,
    >;
    type UserId = i32;
    fn insert(
        self,
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertablePhoneModel = self
            .try_insert(user_id, conn)?;
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
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertablePhoneModelAttributes::Extension(
                        crate::codegen::structs_codegen::tables::insertables::InsertablePhoneModelExtensionAttributes::CameraModel(
                            crate::codegen::structs_codegen::tables::insertables::InsertableCameraModelAttributes::Id,
                        ),
                    ))
                })?;
            let _ = self
                .phone_models_positioning
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertablePhoneModelAttributes::Extension(
                        crate::codegen::structs_codegen::tables::insertables::InsertablePhoneModelExtensionAttributes::PositioningDeviceModel(
                            crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceModelAttributes::Id,
                        ),
                    ))
                })?;
            id
        } else {
            let id = self
                .phone_models_positioning
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertablePhoneModelAttributes::Extension(
                        crate::codegen::structs_codegen::tables::insertables::InsertablePhoneModelExtensionAttributes::PositioningDeviceModel(
                            crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceModelAttributes::Id,
                        ),
                    ))
                })?;
            let _ = self
                .phone_models_camera
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertablePhoneModelAttributes::Extension(
                        crate::codegen::structs_codegen::tables::insertables::InsertablePhoneModelExtensionAttributes::CameraModel(
                            crate::codegen::structs_codegen::tables::insertables::InsertableCameraModelAttributes::Id,
                        ),
                    ))
                })?;
            id
        };
        Ok(Self::InsertableVariant { id })
    }
}
