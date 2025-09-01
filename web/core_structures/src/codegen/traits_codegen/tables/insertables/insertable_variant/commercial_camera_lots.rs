impl<
    C: diesel::connection::LoadConnection,
    CommercialProductLot,
    CameraModel,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCameraLotBuilder<
    CommercialProductLot,
    CameraModel,
>
where
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization,
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::commercial_camera_lots::CommercialCameraLot as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCameraLot as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::commercial_camera_lots::CommercialCameraLot as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::commercial_camera_lots::CommercialCameraLot,
    >,
    C: diesel::connection::LoadConnection,
    CameraModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
    CommercialProductLot: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelBuilder: web_common_traits::database::TryInsertGeneric<
        C,
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelAttributes,
        PrimaryKey = i32,
    >,
{
    type Row = crate::codegen::structs_codegen::tables::commercial_camera_lots::CommercialCameraLot;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCameraLot;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCameraLotAttributes,
    >;
    type UserId = i32;
    fn insert(
        self,
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCameraLot = self
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
                    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCameraLotAttributes::ProductModelId,
                ),
            )?;
        let id = if self.commercial_camera_lots_id_fkey1.is_complete() {
            let id = self
                .commercial_camera_lots_id_fkey1
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCameraLotAttributes::Extension(
                        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCameraLotExtensionAttributes::CameraModel(
                            crate::codegen::structs_codegen::tables::insertables::InsertableCameraModelAttributes::Id,
                        ),
                    ))
                })?;
            let _ = self
                .commercial_camera_lots_id_fkey
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCameraLotAttributes::Extension(
                        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCameraLotExtensionAttributes::CommercialProductLot(
                            crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotAttributes::Id,
                        ),
                    ))
                })?;
            id
        } else {
            let id = self
                .commercial_camera_lots_id_fkey
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCameraLotAttributes::Extension(
                        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCameraLotExtensionAttributes::CommercialProductLot(
                            crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotAttributes::Id,
                        ),
                    ))
                })?;
            let _ = self
                .commercial_camera_lots_id_fkey1
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCameraLotAttributes::Extension(
                        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCameraLotExtensionAttributes::CameraModel(
                            crate::codegen::structs_codegen::tables::insertables::InsertableCameraModelAttributes::Id,
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
