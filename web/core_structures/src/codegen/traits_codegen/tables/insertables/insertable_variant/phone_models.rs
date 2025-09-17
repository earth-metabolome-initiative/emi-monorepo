impl<CameraModel, PositioningDeviceModel>
    web_common_traits::database::DispatchableInsertVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertablePhoneModelBuilder<
        CameraModel,
        PositioningDeviceModel,
    >
{
    type Row = crate::codegen::structs_codegen::tables::phone_models::PhoneModel;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::PhoneModelAttribute,
    >;
}
impl<CameraModel, PositioningDeviceModel> web_common_traits::database::InsertableVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertablePhoneModelBuilder<
        CameraModel,
        PositioningDeviceModel,
    >
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertablePhoneModel;
}
#[cfg(feature = "backend")]
impl<CameraModel, PositioningDeviceModel> web_common_traits::database::BackendInsertableVariant
    for crate::codegen::structs_codegen::tables::insertables::InsertablePhoneModelBuilder<
        CameraModel,
        PositioningDeviceModel,
    >
where
    Self: web_common_traits::database::DispatchableInsertableVariant<diesel::PgConnection>,
{
}
impl<
    C: diesel::connection::LoadConnection,
    CameraModel,
    PositioningDeviceModel,
> web_common_traits::database::DispatchableInsertableVariant<C>
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
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertablePhoneModel,
        Row = crate::codegen::structs_codegen::tables::phone_models::PhoneModel,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PhoneModelAttribute,
        >,
    >,
    CameraModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
    PositioningDeviceModel: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    Self: web_common_traits::database::MostConcreteTable,
    crate::codegen::structs_codegen::tables::insertables::PhoneModelExtensionAttribute: From<
        <CameraModel as common_traits::builder::Attributed>::Attribute,
    >,
    crate::codegen::structs_codegen::tables::insertables::PhoneModelExtensionAttribute: From<
        <PositioningDeviceModel as common_traits::builder::Attributed>::Attribute,
    >,
{
    fn insert(mut self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        use web_common_traits::database::MostConcreteTable;
        self.set_most_concrete_table("phone_models");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertablePhoneModel = self
            .try_insert(user_id, conn)?;
        Ok(
            diesel::insert_into(Self::table())
                .values(insertable_struct)
                .get_result(conn)?,
        )
    }
}
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
    CameraModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
    PositioningDeviceModel: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    Self: web_common_traits::database::MostConcreteTable,
    crate::codegen::structs_codegen::tables::insertables::PhoneModelExtensionAttribute: From<
        <CameraModel as common_traits::builder::Attributed>::Attribute,
    >,
    crate::codegen::structs_codegen::tables::insertables::PhoneModelExtensionAttribute: From<
        <PositioningDeviceModel as common_traits::builder::Attributed>::Attribute,
    >,
{
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
                        crate::codegen::structs_codegen::tables::insertables::PhoneModelAttribute::Extension(
                            From::from(attribute),
                        )
                    })
                })?;
            let _ = self
                .phone_models_positioning
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|attribute| {
                        crate::codegen::structs_codegen::tables::insertables::PhoneModelAttribute::Extension(
                            From::from(attribute),
                        )
                    })
                })?;
            id
        } else {
            let id = self
                .phone_models_positioning
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|attribute| {
                        crate::codegen::structs_codegen::tables::insertables::PhoneModelAttribute::Extension(
                            From::from(attribute),
                        )
                    })
                })?;
            let _ = self
                .phone_models_camera
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|attribute| {
                        crate::codegen::structs_codegen::tables::insertables::PhoneModelAttribute::Extension(
                            From::from(attribute),
                        )
                    })
                })?;
            id
        };
        Ok(Self::InsertableVariant { id })
    }
}
