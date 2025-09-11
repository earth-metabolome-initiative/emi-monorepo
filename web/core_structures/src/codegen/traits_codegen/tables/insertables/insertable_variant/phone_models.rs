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
    CameraModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
    PositioningDeviceModel: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    crate::codegen::structs_codegen::tables::camera_models::CameraModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::camera_models::CameraModel: web_common_traits::database::Updatable<
        C,
        UserId = i32,
    >,
    crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel: web_common_traits::database::Updatable<
        C,
        UserId = i32,
    >,
    Self: web_common_traits::database::MostConcreteTable,
    crate::codegen::structs_codegen::tables::insertables::PhoneModelAttribute: web_common_traits::database::FromExtensionAttribute<
        crate::codegen::structs_codegen::tables::insertables::CameraModelAttribute,
        CameraModel,
        EffectiveExtensionAttribute = <CameraModel as web_common_traits::database::TryInsertGeneric<
            C,
        >>::Attribute,
    >,
    crate::codegen::structs_codegen::tables::insertables::PhoneModelAttribute: web_common_traits::database::FromExtensionAttribute<
        crate::codegen::structs_codegen::tables::insertables::PositioningDeviceModelAttribute,
        PositioningDeviceModel,
        EffectiveExtensionAttribute = <PositioningDeviceModel as web_common_traits::database::TryInsertGeneric<
            C,
        >>::Attribute,
    >,
{
    type Row = crate::codegen::structs_codegen::tables::phone_models::PhoneModel;
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
                    err.into_field_name(|attribute| {
                        <crate::codegen::structs_codegen::tables::insertables::PhoneModelAttribute as web_common_traits::database::FromExtensionAttribute<
                            crate::codegen::structs_codegen::tables::insertables::CameraModelAttribute,
                            CameraModel,
                        >>::from_extension_attribute(attribute)
                    })
                })?;
            let _ = self
                .phone_models_positioning
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|attribute| {
                        <crate::codegen::structs_codegen::tables::insertables::PhoneModelAttribute as web_common_traits::database::FromExtensionAttribute<
                            crate::codegen::structs_codegen::tables::insertables::PositioningDeviceModelAttribute,
                            PositioningDeviceModel,
                        >>::from_extension_attribute(attribute)
                    })
                })?;
            id
        } else {
            let id = self
                .phone_models_positioning
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|attribute| {
                        <crate::codegen::structs_codegen::tables::insertables::PhoneModelAttribute as web_common_traits::database::FromExtensionAttribute<
                            crate::codegen::structs_codegen::tables::insertables::PositioningDeviceModelAttribute,
                            PositioningDeviceModel,
                        >>::from_extension_attribute(attribute)
                    })
                })?;
            let _ = self
                .phone_models_camera
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|attribute| {
                        <crate::codegen::structs_codegen::tables::insertables::PhoneModelAttribute as web_common_traits::database::FromExtensionAttribute<
                            crate::codegen::structs_codegen::tables::insertables::CameraModelAttribute,
                            CameraModel,
                        >>::from_extension_attribute(attribute)
                    })
                })?;
            id
        };
        Ok(Self::InsertableVariant { id })
    }
}
