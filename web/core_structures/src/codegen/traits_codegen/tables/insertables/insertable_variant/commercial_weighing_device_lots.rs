impl<
    C: diesel::connection::LoadConnection,
    CommercialProductLot,
    WeighingDeviceModel,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialWeighingDeviceLotBuilder<
    CommercialProductLot,
    WeighingDeviceModel,
>
where
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization,
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::commercial_weighing_device_lots::CommercialWeighingDeviceLot as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialWeighingDeviceLot as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::commercial_weighing_device_lots::CommercialWeighingDeviceLot as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::commercial_weighing_device_lots::CommercialWeighingDeviceLot,
    >,
    C: diesel::connection::LoadConnection,
    CommercialProductLot: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    WeighingDeviceModel: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelBuilder: web_common_traits::database::TryInsertGeneric<
        C,
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelAttributes,
        PrimaryKey = i32,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialWeighingDeviceLotAttributes,
    >,
{
    type Row = crate::codegen::structs_codegen::tables::commercial_weighing_device_lots::CommercialWeighingDeviceLot;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialWeighingDeviceLot;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialWeighingDeviceLotAttributes,
    >;
    type UserId = i32;
    fn insert(
        mut self,
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        self = <Self as crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable>::most_concrete_table(
            self,
            "commercial_weighing_device_lots",
        )?;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialWeighingDeviceLot = self
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
        let product_model_id = self
            .product_model_id
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialWeighingDeviceLotAttributes::ProductModelId,
                ),
            )?;
        let id = if self.commercial_weighing_device_lots_id_fkey.is_complete() {
            let id = self
                .commercial_weighing_device_lots_id_fkey
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertableCommercialWeighingDeviceLotAttributes::Extension(
                        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialWeighingDeviceLotExtensionAttributes::CommercialProductLot(
                            crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotAttributes::Id,
                        ),
                    ))
                })?;
            let _ = self
                .commercial_weighing_device_lots_id_fkey1
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertableCommercialWeighingDeviceLotAttributes::Extension(
                        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialWeighingDeviceLotExtensionAttributes::WeighingDeviceModel(
                            crate::codegen::structs_codegen::tables::insertables::InsertableWeighingDeviceModelAttributes::Id,
                        ),
                    ))
                })?;
            id
        } else {
            let id = self
                .commercial_weighing_device_lots_id_fkey1
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertableCommercialWeighingDeviceLotAttributes::Extension(
                        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialWeighingDeviceLotExtensionAttributes::WeighingDeviceModel(
                            crate::codegen::structs_codegen::tables::insertables::InsertableWeighingDeviceModelAttributes::Id,
                        ),
                    ))
                })?;
            let _ = self
                .commercial_weighing_device_lots_id_fkey
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertableCommercialWeighingDeviceLotAttributes::Extension(
                        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialWeighingDeviceLotExtensionAttributes::CommercialProductLot(
                            crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotAttributes::Id,
                        ),
                    ))
                })?;
            id
        };
        Ok(Self::InsertableVariant {
            id,
            product_model_id,
        })
    }
}
