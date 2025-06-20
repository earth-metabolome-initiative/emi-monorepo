impl<C: diesel::connection::LoadConnection> web_common_traits::database::Updatable<C>
for crate::codegen::structs_codegen::tables::teams::Team
where
    crate::codegen::structs_codegen::tables::teams::Team: diesel::Identifiable,
    <crate::codegen::structs_codegen::tables::teams::Team as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::teams::Team as diesel::Identifiable>::Id,
    >,
    <<crate::codegen::structs_codegen::tables::teams::Team as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::teams::Team as diesel::Identifiable>::Id,
    >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
    <<<crate::codegen::structs_codegen::tables::teams::Team as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::teams::Team as diesel::Identifiable>::Id,
    >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
        'a,
        C,
        crate::codegen::structs_codegen::tables::teams::Team,
    >,
{
    type UserId = i32;
    fn can_update(
        &self,
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<bool, diesel::result::Error> {
        if user_id == self.created_by {
            return Ok(true);
        }
        if user_id == self.updated_by {
            return Ok(true);
        }
        if let Some(teams) = self.parent_team(conn)? {
            if !teams.can_update(user_id, conn)? {
                return Ok(false);
            }
        }
        Ok(true)
    }
}
