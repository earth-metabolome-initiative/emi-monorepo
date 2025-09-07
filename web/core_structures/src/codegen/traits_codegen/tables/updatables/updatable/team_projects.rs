impl<C: diesel::connection::LoadConnection> web_common_traits::database::Updatable<C>
for crate::TeamProject
where
    crate::Project: diesel::Identifiable,
    <crate::Project as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
        <crate::Project as diesel::Identifiable>::Id,
    >,
    <<crate::Project as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::Project as diesel::Identifiable>::Id,
    >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
    <<<crate::Project as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::Project as diesel::Identifiable>::Id,
    >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
        'a,
        C,
        crate::Project,
    >,
    crate::Project: web_common_traits::database::Updatable<C, UserId = i32>,
    crate::Team: diesel::Identifiable,
    <crate::Team as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
        <crate::Team as diesel::Identifiable>::Id,
    >,
    <<crate::Team as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::Team as diesel::Identifiable>::Id,
    >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
    <<<crate::Team as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::Team as diesel::Identifiable>::Id,
    >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
        'a,
        C,
        crate::Team,
    >,
    crate::Team: web_common_traits::database::Updatable<C, UserId = i32>,
{
    type UserId = i32;
    fn can_update(
        &self,
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<bool, diesel::result::Error> {
        if !self.project(conn)?.can_update(user_id, conn)? {
            return Ok(false);
        }
        if !self.team(conn)?.can_update(user_id, conn)? {
            return Ok(false);
        }
        Ok(true)
    }
}
