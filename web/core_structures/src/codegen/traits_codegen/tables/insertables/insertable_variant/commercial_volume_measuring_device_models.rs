impl<
    C: diesel::connection::LoadConnection,
    CommercialProduct,
    VolumeMeasuringDeviceModel,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialVolumeMeasuringDeviceModelBuilder<
    CommercialProduct,
    VolumeMeasuringDeviceModel,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_models::CommercialVolumeMeasuringDeviceModel as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialVolumeMeasuringDeviceModel as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_models::CommercialVolumeMeasuringDeviceModel as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_models::CommercialVolumeMeasuringDeviceModel,
    >,
    C: diesel::connection::LoadConnection,
    CommercialProduct: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    VolumeMeasuringDeviceModel: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    crate::codegen::structs_codegen::tables::volume_measuring_device_models::VolumeMeasuringDeviceModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::volume_measuring_device_models::VolumeMeasuringDeviceModel: web_common_traits::database::Updatable<
        C,
        UserId = i32,
    >,
    Self: web_common_traits::database::MostConcreteTable,
    crate::codegen::structs_codegen::tables::insertables::CommercialVolumeMeasuringDeviceModelAttribute: web_common_traits::database::FromExtensionAttribute<
        crate::codegen::structs_codegen::tables::insertables::VolumeMeasuringDeviceModelAttribute,
        VolumeMeasuringDeviceModel,
        EffectiveExtensionAttribute = <VolumeMeasuringDeviceModel as web_common_traits::database::TryInsertGeneric<
            C,
        >>::Attribute,
    >,
    crate::codegen::structs_codegen::tables::insertables::CommercialVolumeMeasuringDeviceModelAttribute: web_common_traits::database::FromExtensionAttribute<
        crate::codegen::structs_codegen::tables::insertables::CommercialProductAttribute,
        CommercialProduct,
        EffectiveExtensionAttribute = <CommercialProduct as web_common_traits::database::TryInsertGeneric<
            C,
        >>::Attribute,
    >,
{
    type Row = crate::codegen::structs_codegen::tables::commercial_volume_measuring_device_models::CommercialVolumeMeasuringDeviceModel;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialVolumeMeasuringDeviceModel;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::CommercialVolumeMeasuringDeviceModelAttribute,
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
        self.set_most_concrete_table("commercial_volume_measuring_device_models");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialVolumeMeasuringDeviceModel = self
            .try_insert(user_id, conn)?;
        if !insertable_struct
            .volume_measuring_device_model(conn)?
            .can_update(user_id, conn)?
        {
            return Err(
                generic_backend_request_errors::GenericBackendRequestError::Unauthorized
                    .into(),
            );
        }
        if !insertable_struct
            .commercial_volume_measuring_device_models_id_fkey(conn)?
            .can_update(user_id, conn)?
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
        let volume_measuring_device_model = self
            .volume_measuring_device_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::CommercialVolumeMeasuringDeviceModelAttribute::VolumeMeasuringDeviceModel,
                ),
            )?;
        let id = if self.commercial_volume_measuring_device_models_id_fkey1.is_complete()
        {
            let id = self
                .commercial_volume_measuring_device_models_id_fkey1
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|attribute| {
                        <crate::codegen::structs_codegen::tables::insertables::CommercialVolumeMeasuringDeviceModelAttribute as web_common_traits::database::FromExtensionAttribute<
                            crate::codegen::structs_codegen::tables::insertables::CommercialProductAttribute,
                            CommercialProduct,
                        >>::from_extension_attribute(attribute)
                    })
                })?;
            let _ = self
                .commercial_volume_measuring_device_models_id_fkey
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|attribute| {
                        <crate::codegen::structs_codegen::tables::insertables::CommercialVolumeMeasuringDeviceModelAttribute as web_common_traits::database::FromExtensionAttribute<
                            crate::codegen::structs_codegen::tables::insertables::VolumeMeasuringDeviceModelAttribute,
                            VolumeMeasuringDeviceModel,
                        >>::from_extension_attribute(attribute)
                    })
                })?;
            id
        } else {
            let id = self
                .commercial_volume_measuring_device_models_id_fkey
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|attribute| {
                        <crate::codegen::structs_codegen::tables::insertables::CommercialVolumeMeasuringDeviceModelAttribute as web_common_traits::database::FromExtensionAttribute<
                            crate::codegen::structs_codegen::tables::insertables::VolumeMeasuringDeviceModelAttribute,
                            VolumeMeasuringDeviceModel,
                        >>::from_extension_attribute(attribute)
                    })
                })?;
            let _ = self
                .commercial_volume_measuring_device_models_id_fkey1
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|attribute| {
                        <crate::codegen::structs_codegen::tables::insertables::CommercialVolumeMeasuringDeviceModelAttribute as web_common_traits::database::FromExtensionAttribute<
                            crate::codegen::structs_codegen::tables::insertables::CommercialProductAttribute,
                            CommercialProduct,
                        >>::from_extension_attribute(attribute)
                    })
                })?;
            id
        };
        Ok(Self::InsertableVariant {
            id,
            volume_measuring_device_model,
        })
    }
}
