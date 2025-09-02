impl<
    C: diesel::connection::LoadConnection,
    AssetModel,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableDigitalAssetModelBuilder<
    AssetModel,
>
where
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization,
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableDigitalAssetModel as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel,
    >,
    AssetModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
    C: diesel::connection::LoadConnection,
    crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelBuilder: web_common_traits::database::TryInsertGeneric<
        C,
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelAttributes,
        PrimaryKey = i32,
    >,
    Self: crate::codegen::structs_codegen::tables::insertables::AssetModelBuildable<
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableDigitalAssetModelAttributes,
    >,
{
    type Row = crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableDigitalAssetModel;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::InsertableDigitalAssetModelAttributes,
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
            "digital_asset_models",
        )?;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableDigitalAssetModel = self
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
        let id = self
            .id
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertableDigitalAssetModelAttributes::Extension(
                    crate::codegen::structs_codegen::tables::insertables::InsertableDigitalAssetModelExtensionAttributes::AssetModel(
                        crate::codegen::structs_codegen::tables::insertables::InsertableAssetModelAttributes::Id,
                    ),
                ))
            })?;
        Ok(Self::InsertableVariant {
            id,
            parent_model_id: self.parent_model_id,
        })
    }
}
