impl<Asset> web_common_traits::database::DispatchableInsertVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetBuilder<Asset>
{
    type Row = crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::PhysicalAssetAttribute,
    >;
}
impl<Asset> web_common_traits::database::InsertableVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetBuilder<Asset>
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAsset;
}
#[cfg(feature = "backend")]
impl<Asset> web_common_traits::database::BackendInsertableVariant
    for crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetBuilder<Asset>
where
    Self: web_common_traits::database::DispatchableInsertableVariant<diesel::PgConnection>,
{
}
impl<
    C: diesel::connection::LoadConnection,
    Asset,
> web_common_traits::database::DispatchableInsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetBuilder<
    Asset,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAsset as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset,
    >,
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAsset,
        Row = crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PhysicalAssetAttribute,
        >,
    >,
    Self: web_common_traits::database::MostConcreteTable,
{
    fn insert(mut self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        use web_common_traits::database::MostConcreteTable;
        self.set_most_concrete_table("physical_assets");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAsset = self
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
    Asset,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetBuilder<
    Asset,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAsset as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::physical_assets::PhysicalAsset,
    >,
    Self::Error: web_common_traits::database::FromExtension<
        <Asset as web_common_traits::database::TryInsertGeneric<C>>::Error,
    >,
    Asset: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
{
    fn try_insert(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::InsertableVariant, Self::Error> {
        use web_common_traits::database::FromExtension;
        let model = self
            .model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::PhysicalAssetAttribute::Model,
                ),
            )?;
        let id = self
            .id
            .mint_primary_key(user_id, conn)
            .map_err(Self::Error::from_extension)?;
        Ok(Self::InsertableVariant {
            id,
            model,
        })
    }
}
