impl web_common_traits::database::DispatchableInsertVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelAncestorBuilder
{
    type Row = crate::codegen::structs_codegen::tables::asset_model_ancestors::AssetModelAncestor;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::AssetModelAncestorAttribute,
    >;
}
impl web_common_traits::database::InsertableVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelAncestorBuilder
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelAncestor;
}
#[cfg(feature = "backend")]
impl web_common_traits::database::BackendInsertableVariant
    for crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelAncestorBuilder
where
    Self: web_common_traits::database::DispatchableInsertableVariant<diesel::PgConnection>,
{
}
impl<
    C: diesel::connection::LoadConnection,
> web_common_traits::database::DispatchableInsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelAncestorBuilder
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::asset_model_ancestors::AssetModelAncestor as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelAncestor as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::asset_model_ancestors::AssetModelAncestor as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::asset_model_ancestors::AssetModelAncestor,
    >,
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelAncestor,
        Row = crate::codegen::structs_codegen::tables::asset_model_ancestors::AssetModelAncestor,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::AssetModelAncestorAttribute,
        >,
    >,
{
    fn insert(self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelAncestor = self
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
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelAncestorBuilder
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::asset_model_ancestors::AssetModelAncestor as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelAncestor as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::asset_model_ancestors::AssetModelAncestor as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::asset_model_ancestors::AssetModelAncestor,
    >,
{
    fn try_insert(
        self,
        _user_id: i32,
        _conn: &mut C,
    ) -> Result<Self::InsertableVariant, Self::Error> {
        let descendant_model = self
            .descendant_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::AssetModelAncestorAttribute::DescendantModel,
                ),
            )?;
        let ancestor_model = self
            .ancestor_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::AssetModelAncestorAttribute::AncestorModel,
                ),
            )?;
        Ok(Self::InsertableVariant {
            descendant_model,
            ancestor_model,
        })
    }
}
