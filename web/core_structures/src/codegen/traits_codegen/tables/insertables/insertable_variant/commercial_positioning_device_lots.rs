impl<
    C: diesel::connection::LoadConnection,
    CommercialProductLot,
    PositioningDeviceModel,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPositioningDeviceLotBuilder<
    CommercialProductLot,
    PositioningDeviceModel,
>
where
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization,
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::commercial_positioning_device_lots::CommercialPositioningDeviceLot as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPositioningDeviceLot as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::commercial_positioning_device_lots::CommercialPositioningDeviceLot as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::commercial_positioning_device_lots::CommercialPositioningDeviceLot,
    >,
    C: diesel::connection::LoadConnection,
    CommercialProductLot: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    PositioningDeviceModel: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    crate::codegen::structs_codegen::tables::positioning_device_models::PositioningDeviceModel: diesel::Identifiable
        + web_common_traits::database::Updatable<C, UserId = i32>,
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
    Self: web_common_traits::database::MostConcreteTable,
{
    type Row = crate::codegen::structs_codegen::tables::commercial_positioning_device_lots::CommercialPositioningDeviceLot;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPositioningDeviceLot;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPositioningDeviceLotAttributes,
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
        self.set_most_concrete_table("commercial_positioning_device_lots");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPositioningDeviceLot = self
            .try_insert(user_id, conn)?;
        if !insertable_struct
            .commercial_positioning_device_lots_id_fkey1(conn)?
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
                    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPositioningDeviceLotAttributes::ProductModel,
                ),
            )?;
        let id = if self.commercial_positioning_device_lots_id_fkey.is_complete() {
            let id = self
                .commercial_positioning_device_lots_id_fkey
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPositioningDeviceLotAttributes::Extension(
                        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPositioningDeviceLotExtensionAttributes::CommercialProductLot(
                            crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotAttributes::Id,
                        ),
                    ))
                })?;
            let _ = self
                .commercial_positioning_device_lots_id_fkey1
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPositioningDeviceLotAttributes::Extension(
                        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPositioningDeviceLotExtensionAttributes::PositioningDeviceModel(
                            crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceModelAttributes::Id,
                        ),
                    ))
                })?;
            id
        } else {
            let id = self
                .commercial_positioning_device_lots_id_fkey1
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPositioningDeviceLotAttributes::Extension(
                        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPositioningDeviceLotExtensionAttributes::PositioningDeviceModel(
                            crate::codegen::structs_codegen::tables::insertables::InsertablePositioningDeviceModelAttributes::Id,
                        ),
                    ))
                })?;
            let _ = self
                .commercial_positioning_device_lots_id_fkey
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPositioningDeviceLotAttributes::Extension(
                        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPositioningDeviceLotExtensionAttributes::CommercialProductLot(
                            crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotAttributes::Id,
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
