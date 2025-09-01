impl<C: diesel::connection::LoadConnection> web_common_traits::database::Updatable<C>
for crate::codegen::structs_codegen::tables::digital_assets::DigitalAsset
where
    crate::codegen::structs_codegen::tables::assets::Asset: diesel::Identifiable,
    <crate::codegen::structs_codegen::tables::assets::Asset as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::assets::Asset as diesel::Identifiable>::Id,
    >,
    <<crate::codegen::structs_codegen::tables::assets::Asset as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::assets::Asset as diesel::Identifiable>::Id,
    >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
    <<<crate::codegen::structs_codegen::tables::assets::Asset as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::assets::Asset as diesel::Identifiable>::Id,
    >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
        'a,
        C,
        crate::codegen::structs_codegen::tables::assets::Asset,
    >,
    crate::codegen::structs_codegen::tables::assets::Asset: web_common_traits::database::Updatable<
        C,
        UserId = i32,
    >,
    crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel: diesel::Identifiable,
    <crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel as diesel::Identifiable>::Id,
    >,
    <<crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel as diesel::Identifiable>::Id,
    >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
    <<<crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel as diesel::Identifiable>::Id,
    >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
        'a,
        C,
        crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel,
    >,
    crate::codegen::structs_codegen::tables::digital_asset_models::DigitalAssetModel: web_common_traits::database::Updatable<
        C,
        UserId = i32,
    >,
{
    type UserId = i32;
    fn can_update(
        &self,
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<bool, diesel::result::Error> {
        if !self.id(conn)?.can_update(user_id, conn)? {
            return Ok(false);
        }
        if !self.model(conn)?.can_update(user_id, conn)? {
            return Ok(false);
        }
        Ok(true)
    }
}
