impl<C: diesel::connection::LoadConnection> web_common_traits::database::Updatable<C>
for crate::PhysicalAsset
where
    crate::Asset: diesel::Identifiable,
    <crate::Asset as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
        <crate::Asset as diesel::Identifiable>::Id,
    >,
    <<crate::Asset as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::Asset as diesel::Identifiable>::Id,
    >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
    <<<crate::Asset as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::Asset as diesel::Identifiable>::Id,
    >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
        'a,
        C,
        crate::Asset,
    >,
    crate::Asset: web_common_traits::database::Updatable<C, UserId = i32>,
    crate::PhysicalAssetModel: diesel::Identifiable,
    <crate::PhysicalAssetModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
        <crate::PhysicalAssetModel as diesel::Identifiable>::Id,
    >,
    <<crate::PhysicalAssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::PhysicalAssetModel as diesel::Identifiable>::Id,
    >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
    <<<crate::PhysicalAssetModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::PhysicalAssetModel as diesel::Identifiable>::Id,
    >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
        'a,
        C,
        crate::PhysicalAssetModel,
    >,
    crate::PhysicalAssetModel: web_common_traits::database::Updatable<C, UserId = i32>,
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
