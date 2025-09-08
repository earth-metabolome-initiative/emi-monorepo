impl<
    C: diesel::connection::LoadConnection,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableAssetCompatibilityRuleBuilder
where
    diesel::query_builder::InsertStatement<
        <crate::AssetCompatibilityRule as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableAssetCompatibilityRule as diesel::Insertable<
            <crate::AssetCompatibilityRule as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::AssetCompatibilityRule,
    >,
    C: diesel::connection::LoadConnection,
    crate::AssetModel: web_common_traits::database::Read<C>,
    crate::AssetModel: web_common_traits::database::Updatable<C, UserId = i32>,
{
    type Row = crate::AssetCompatibilityRule;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableAssetCompatibilityRule;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::AssetCompatibilityRuleAttribute,
    >;
    type UserId = i32;
    fn insert(
        self,
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::Updatable;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableAssetCompatibilityRule = self
            .try_insert(user_id, conn)?;
        if !insertable_struct.left_asset_model(conn)?.can_update(user_id, conn)? {
            return Err(
                generic_backend_request_errors::GenericBackendRequestError::Unauthorized
                    .into(),
            );
        }
        if !insertable_struct.right_asset_model(conn)?.can_update(user_id, conn)? {
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
        _user_id: i32,
        _conn: &mut C,
    ) -> Result<Self::InsertableVariant, Self::Error> {
        let left_asset_model = self
            .left_asset_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::AssetCompatibilityRuleAttribute::LeftAssetModel,
                ),
            )?;
        let right_asset_model = self
            .right_asset_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::AssetCompatibilityRuleAttribute::RightAssetModel,
                ),
            )?;
        let created_by = self
            .created_by
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::AssetCompatibilityRuleAttribute::CreatedBy,
                ),
            )?;
        let created_at = self
            .created_at
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::AssetCompatibilityRuleAttribute::CreatedAt,
                ),
            )?;
        Ok(Self::InsertableVariant {
            left_asset_model,
            right_asset_model,
            created_by,
            created_at,
        })
    }
}
