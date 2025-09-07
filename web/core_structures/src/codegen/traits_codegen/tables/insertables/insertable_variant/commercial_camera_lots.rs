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
        <crate::CommercialCameraLot as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCameraLot as diesel::Insertable<
            <crate::CommercialCameraLot as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<'query, C, crate::CommercialCameraLot>,
    C: diesel::connection::LoadConnection,
    CameraModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
    CommercialProductLot: web_common_traits::database::TryInsertGeneric<
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
    Self: web_common_traits::database::MostConcreteTable,
{
    type Row = crate::CommercialCameraLot;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCameraLot;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::CommercialCameraLotAttribute,
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
        self.set_most_concrete_table("commercial_camera_lots");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCameraLot = self
            .try_insert(user_id, conn)?;
        if !insertable_struct
            .commercial_camera_lots_id_fkey1(conn)?
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
                    crate::codegen::structs_codegen::tables::insertables::CommercialCameraLotAttribute::ProductModel,
                ),
            )?;
        let id = if self.commercial_camera_lots_id_fkey1.is_complete() {
            let id = self
                .commercial_camera_lots_id_fkey1
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::CommercialCameraLotAttribute::Extension(
                        crate::codegen::structs_codegen::tables::insertables::CommercialCameraLotExtensionAttribute::CameraModel(
                            crate::codegen::structs_codegen::tables::insertables::CameraModelAttribute::Id,
                        ),
                    ))
                })?;
            let _ = self
                .commercial_camera_lots_id_fkey
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::CommercialCameraLotAttribute::Extension(
                        crate::codegen::structs_codegen::tables::insertables::CommercialCameraLotExtensionAttribute::CommercialProductLot(
                            crate::codegen::structs_codegen::tables::insertables::CommercialProductLotAttribute::Id,
                        ),
                    ))
                })?;
            id
        } else {
            let id = self
                .commercial_camera_lots_id_fkey
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::CommercialCameraLotAttribute::Extension(
                        crate::codegen::structs_codegen::tables::insertables::CommercialCameraLotExtensionAttribute::CommercialProductLot(
                            crate::codegen::structs_codegen::tables::insertables::CommercialProductLotAttribute::Id,
                        ),
                    ))
                })?;
            let _ = self
                .commercial_camera_lots_id_fkey1
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::CommercialCameraLotAttribute::Extension(
                        crate::codegen::structs_codegen::tables::insertables::CommercialCameraLotExtensionAttribute::CameraModel(
                            crate::codegen::structs_codegen::tables::insertables::CameraModelAttribute::Id,
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
