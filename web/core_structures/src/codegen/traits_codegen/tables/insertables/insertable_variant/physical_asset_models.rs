impl<AssetModel> web_common_traits::database::DispatchableInsertVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetModelBuilder<
        AssetModel,
    >
{
    type Row = crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelAttribute,
    >;
}
impl<AssetModel> web_common_traits::database::InsertableVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetModelBuilder<
        AssetModel,
    >
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetModel;
}
#[cfg(feature = "backend")]
impl<AssetModel> web_common_traits::database::BackendInsertableVariant
    for crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetModelBuilder<
        AssetModel,
    >
where
    Self: web_common_traits::database::DispatchableInsertableVariant<diesel::PgConnection>,
{
}
impl<
    C: diesel::connection::LoadConnection,
    AssetModel,
> web_common_traits::database::DispatchableInsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetModelBuilder<
    AssetModel,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetModel as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel,
    >,
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetModel,
        Row = crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::PhysicalAssetModelAttribute,
        >,
    >,
    Self: web_common_traits::database::MostConcreteTable,
{
    fn insert(mut self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        use web_common_traits::database::MostConcreteTable;
        self.set_most_concrete_table("physical_asset_models");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetModel = self
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
    AssetModel,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetModelBuilder<
    AssetModel,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertablePhysicalAssetModel as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::physical_asset_models::PhysicalAssetModel,
    >,
    Self::Error: web_common_traits::database::FromExtension<
        <AssetModel as web_common_traits::database::TryInsertGeneric<C>>::Error,
    >,
    AssetModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
{
    fn try_insert(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::InsertableVariant, Self::Error> {
        use web_common_traits::database::FromExtension;
        let id = self
            .id
            .mint_primary_key(user_id, conn)
            .map_err(Self::Error::from_extension)?;
        Ok(Self::InsertableVariant {
            id,
            parent_model: self.parent_model,
        })
    }
}
