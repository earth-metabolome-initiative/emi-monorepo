impl<
    C: diesel::connection::LoadConnection,
    AssetModel,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableDigitalAssetModelBuilder<
    AssetModel,
>
where
    diesel::query_builder::InsertStatement<
        <crate::DigitalAssetModel as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableDigitalAssetModel as diesel::Insertable<
            <crate::DigitalAssetModel as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<'query, C, crate::DigitalAssetModel>,
    AssetModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
    C: diesel::connection::LoadConnection,
    crate::AssetModel: web_common_traits::database::Read<C>,
    crate::AssetModel: web_common_traits::database::Updatable<C, UserId = i32>,
    Self: web_common_traits::database::MostConcreteTable,
{
    type Row = crate::DigitalAssetModel;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableDigitalAssetModel;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::DigitalAssetModelAttribute,
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
        self.set_most_concrete_table("digital_asset_models");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableDigitalAssetModel = self
            .try_insert(user_id, conn)?;
        if !insertable_struct.id(conn)?.can_update(user_id, conn)? {
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
        let id = self
            .id
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::DigitalAssetModelAttribute::Extension(
                    crate::codegen::structs_codegen::tables::insertables::DigitalAssetModelExtensionAttribute::AssetModel(
                        crate::codegen::structs_codegen::tables::insertables::AssetModelAttribute::Id,
                    ),
                ))
            })?;
        Ok(Self::InsertableVariant {
            id,
            parent_model: self.parent_model,
        })
    }
}
