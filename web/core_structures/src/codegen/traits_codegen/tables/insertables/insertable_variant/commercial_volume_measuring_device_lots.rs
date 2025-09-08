impl<
    C: diesel::connection::LoadConnection,
    CommercialProductLot,
    VolumeMeasuringDeviceModel,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialVolumeMeasuringDeviceLotBuilder<
    CommercialProductLot,
    VolumeMeasuringDeviceModel,
>
where
    diesel::query_builder::InsertStatement<
        <crate::CommercialVolumeMeasuringDeviceLot as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialVolumeMeasuringDeviceLot as diesel::Insertable<
            <crate::CommercialVolumeMeasuringDeviceLot as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::CommercialVolumeMeasuringDeviceLot,
    >,
    C: diesel::connection::LoadConnection,
    CommercialProductLot: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    VolumeMeasuringDeviceModel: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    crate::VolumeMeasuringDeviceModel: web_common_traits::database::Read<C>,
    crate::VolumeMeasuringDeviceModel: web_common_traits::database::Updatable<
        C,
        UserId = i32,
    >,
    Self: web_common_traits::database::MostConcreteTable,
{
    type Row = crate::CommercialVolumeMeasuringDeviceLot;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialVolumeMeasuringDeviceLot;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::CommercialVolumeMeasuringDeviceLotAttribute,
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
        self.set_most_concrete_table("commercial_volume_measuring_device_lots");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialVolumeMeasuringDeviceLot = self
            .try_insert(user_id, conn)?;
        if !insertable_struct
            .commercial_volume_measuring_device_lots_id_fkey1(conn)?
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
        let product_model = self
            .product_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::CommercialVolumeMeasuringDeviceLotAttribute::ProductModel,
                ),
            )?;
        let id = if self.commercial_volume_measuring_device_lots_id_fkey.is_complete() {
            let id = self
                .commercial_volume_measuring_device_lots_id_fkey
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::CommercialVolumeMeasuringDeviceLotAttribute::Extension(
                        crate::codegen::structs_codegen::tables::insertables::CommercialVolumeMeasuringDeviceLotExtensionAttribute::CommercialProductLot(
                            crate::codegen::structs_codegen::tables::insertables::CommercialProductLotAttribute::Id,
                        ),
                    ))
                })?;
            let _ = self
                .commercial_volume_measuring_device_lots_id_fkey1
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::CommercialVolumeMeasuringDeviceLotAttribute::Extension(
                        crate::codegen::structs_codegen::tables::insertables::CommercialVolumeMeasuringDeviceLotExtensionAttribute::VolumeMeasuringDeviceModel(
                            crate::codegen::structs_codegen::tables::insertables::VolumeMeasuringDeviceModelAttribute::Id,
                        ),
                    ))
                })?;
            id
        } else {
            let id = self
                .commercial_volume_measuring_device_lots_id_fkey1
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::CommercialVolumeMeasuringDeviceLotAttribute::Extension(
                        crate::codegen::structs_codegen::tables::insertables::CommercialVolumeMeasuringDeviceLotExtensionAttribute::VolumeMeasuringDeviceModel(
                            crate::codegen::structs_codegen::tables::insertables::VolumeMeasuringDeviceModelAttribute::Id,
                        ),
                    ))
                })?;
            let _ = self
                .commercial_volume_measuring_device_lots_id_fkey
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::CommercialVolumeMeasuringDeviceLotAttribute::Extension(
                        crate::codegen::structs_codegen::tables::insertables::CommercialVolumeMeasuringDeviceLotExtensionAttribute::CommercialProductLot(
                            crate::codegen::structs_codegen::tables::insertables::CommercialProductLotAttribute::Id,
                        ),
                    ))
                })?;
            id
        };
        Ok(Self::InsertableVariant {
            id,
            product_model,
        })
    }
}
