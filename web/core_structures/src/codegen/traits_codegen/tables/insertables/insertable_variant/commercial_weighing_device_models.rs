impl<
    C: diesel::connection::LoadConnection,
    CommercialProduct,
    WeighingDeviceModel,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialWeighingDeviceModelBuilder<
    CommercialProduct,
    WeighingDeviceModel,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::commercial_weighing_device_models::CommercialWeighingDeviceModel as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialWeighingDeviceModel as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::commercial_weighing_device_models::CommercialWeighingDeviceModel as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::commercial_weighing_device_models::CommercialWeighingDeviceModel,
    >,
    C: diesel::connection::LoadConnection,
    CommercialProduct: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    WeighingDeviceModel: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    crate::codegen::structs_codegen::tables::weighing_device_models::WeighingDeviceModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::weighing_device_models::WeighingDeviceModel: web_common_traits::database::Updatable<
        C,
        UserId = i32,
    >,
    Self: web_common_traits::database::MostConcreteTable,
    crate::codegen::structs_codegen::tables::insertables::CommercialWeighingDeviceModelAttribute: web_common_traits::database::FromExtensionAttribute<
        crate::codegen::structs_codegen::tables::insertables::WeighingDeviceModelAttribute,
        WeighingDeviceModel,
        EffectiveExtensionAttribute = <WeighingDeviceModel as web_common_traits::database::TryInsertGeneric<
            C,
        >>::Attribute,
    >,
    crate::codegen::structs_codegen::tables::insertables::CommercialWeighingDeviceModelAttribute: web_common_traits::database::FromExtensionAttribute<
        crate::codegen::structs_codegen::tables::insertables::CommercialProductAttribute,
        CommercialProduct,
        EffectiveExtensionAttribute = <CommercialProduct as web_common_traits::database::TryInsertGeneric<
            C,
        >>::Attribute,
    >,
{
    type Row = crate::codegen::structs_codegen::tables::commercial_weighing_device_models::CommercialWeighingDeviceModel;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialWeighingDeviceModel;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::CommercialWeighingDeviceModelAttribute,
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
        self.set_most_concrete_table("commercial_weighing_device_models");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialWeighingDeviceModel = self
            .try_insert(user_id, conn)?;
        if !insertable_struct
            .commercial_weighing_device_models_id_fkey(conn)?
            .can_update(user_id, conn)?
        {
            return Err(
                generic_backend_request_errors::GenericBackendRequestError::Unauthorized
                    .into(),
            );
        }
        if !insertable_struct.weighing_device_model(conn)?.can_update(user_id, conn)? {
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
        let weighing_device_model = self
            .weighing_device_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::CommercialWeighingDeviceModelAttribute::WeighingDeviceModel,
                ),
            )?;
        let id = if self.commercial_weighing_device_models_id_fkey1.is_complete() {
            let id = self
                .commercial_weighing_device_models_id_fkey1
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|attribute| {
                        <crate::codegen::structs_codegen::tables::insertables::CommercialWeighingDeviceModelAttribute as web_common_traits::database::FromExtensionAttribute<
                            crate::codegen::structs_codegen::tables::insertables::CommercialProductAttribute,
                            CommercialProduct,
                        >>::from_extension_attribute(attribute)
                    })
                })?;
            let _ = self
                .commercial_weighing_device_models_id_fkey
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|attribute| {
                        <crate::codegen::structs_codegen::tables::insertables::CommercialWeighingDeviceModelAttribute as web_common_traits::database::FromExtensionAttribute<
                            crate::codegen::structs_codegen::tables::insertables::WeighingDeviceModelAttribute,
                            WeighingDeviceModel,
                        >>::from_extension_attribute(attribute)
                    })
                })?;
            id
        } else {
            let id = self
                .commercial_weighing_device_models_id_fkey
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|attribute| {
                        <crate::codegen::structs_codegen::tables::insertables::CommercialWeighingDeviceModelAttribute as web_common_traits::database::FromExtensionAttribute<
                            crate::codegen::structs_codegen::tables::insertables::WeighingDeviceModelAttribute,
                            WeighingDeviceModel,
                        >>::from_extension_attribute(attribute)
                    })
                })?;
            let _ = self
                .commercial_weighing_device_models_id_fkey1
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|attribute| {
                        <crate::codegen::structs_codegen::tables::insertables::CommercialWeighingDeviceModelAttribute as web_common_traits::database::FromExtensionAttribute<
                            crate::codegen::structs_codegen::tables::insertables::CommercialProductAttribute,
                            CommercialProduct,
                        >>::from_extension_attribute(attribute)
                    })
                })?;
            id
        };
        Ok(Self::InsertableVariant {
            id,
            weighing_device_model,
        })
    }
}
