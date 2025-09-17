impl web_common_traits::database::DispatchableInsertVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableAssetBuilder
{
    type Row = crate::codegen::structs_codegen::tables::assets::Asset;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::AssetAttribute,
    >;
}
impl web_common_traits::database::InsertableVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableAssetBuilder
{
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableAsset;
}
#[cfg(feature = "backend")]
impl web_common_traits::database::BackendInsertableVariant
    for crate::codegen::structs_codegen::tables::insertables::InsertableAssetBuilder
where
    Self: web_common_traits::database::DispatchableInsertableVariant<diesel::PgConnection>,
{
}
impl<
    C: diesel::connection::LoadConnection,
> web_common_traits::database::DispatchableInsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableAssetBuilder
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::assets::Asset as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableAsset as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::assets::Asset as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::assets::Asset,
    >,
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableAsset,
        Row = crate::codegen::structs_codegen::tables::assets::Asset,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::AssetAttribute,
        >,
    >,
    Self: web_common_traits::database::MostConcreteTable,
{
    fn insert(mut self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        use web_common_traits::database::MostConcreteTable;
        self.set_most_concrete_table("assets");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableAsset = self
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
for crate::codegen::structs_codegen::tables::insertables::InsertableAssetBuilder
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::assets::Asset as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableAsset as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::assets::Asset as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::assets::Asset,
    >,
    Self: web_common_traits::database::MostConcreteTable,
{
    fn try_insert(
        self,
        _user_id: i32,
        _conn: &mut C,
    ) -> Result<Self::InsertableVariant, Self::Error> {
        let id = self
            .id
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::AssetAttribute::Id,
                ),
            )?;
        let most_concrete_table = self
            .most_concrete_table
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::AssetAttribute::MostConcreteTable,
                ),
            )?;
        let model = self
            .model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::AssetAttribute::Model,
                ),
            )?;
        let created_by = self
            .created_by
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::AssetAttribute::CreatedBy,
                ),
            )?;
        let created_at = self
            .created_at
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::AssetAttribute::CreatedAt,
                ),
            )?;
        let updated_by = self
            .updated_by
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::AssetAttribute::UpdatedBy,
                ),
            )?;
        let updated_at = self
            .updated_at
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::AssetAttribute::UpdatedAt,
                ),
            )?;
        Ok(Self::InsertableVariant {
            id,
            most_concrete_table,
            name: self.name,
            description: self.description,
            model,
            created_by,
            created_at,
            updated_by,
            updated_at,
        })
    }
}
