impl<
    C: diesel::connection::LoadConnection,
    PhysicalAsset,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableWeighingDeviceBuilder<
    PhysicalAsset,
>
where
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization,
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::weighing_devices::WeighingDevice as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableWeighingDevice as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::weighing_devices::WeighingDevice as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::weighing_devices::WeighingDevice,
    >,
    C: diesel::connection::LoadConnection,
    PhysicalAsset: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableAssetBuilder: web_common_traits::database::TryInsertGeneric<
        C,
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableAssetAttributes,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::AssetBuildable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableWeighingDeviceAttributes,
    >,
{
    type Row = crate::codegen::structs_codegen::tables::weighing_devices::WeighingDevice;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableWeighingDevice;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::InsertableWeighingDeviceAttributes,
    >;
    type UserId = i32;
    fn insert(
        mut self,
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        self = <Self as crate::codegen::structs_codegen::tables::insertables::AssetBuildable>::most_concrete_table(
            self,
            "weighing_devices",
        )?;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableWeighingDevice = self
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
        let model_id = self
            .model_id
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingDeviceAttributes::ModelId,
                ),
            )?;
        let id = self
            .id
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertableWeighingDeviceAttributes::Extension(
                    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingDeviceExtensionAttributes::PhysicalAsset(
                        crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetAttributes::Id,
                    ),
                ))
            })?;
        Ok(Self::InsertableVariant {
            id,
            model_id,
        })
    }
}
