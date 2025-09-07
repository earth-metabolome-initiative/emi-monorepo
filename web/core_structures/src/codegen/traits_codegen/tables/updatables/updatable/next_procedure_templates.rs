impl<C: diesel::connection::LoadConnection> web_common_traits::database::Updatable<C>
for crate::NextProcedureTemplate
where
    crate::ProcedureTemplate: diesel::Identifiable,
    <crate::ProcedureTemplate as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
        <crate::ProcedureTemplate as diesel::Identifiable>::Id,
    >,
    <<crate::ProcedureTemplate as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::ProcedureTemplate as diesel::Identifiable>::Id,
    >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
    <<<crate::ProcedureTemplate as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::ProcedureTemplate as diesel::Identifiable>::Id,
    >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
        'a,
        C,
        crate::ProcedureTemplate,
    >,
    crate::ProcedureTemplate: web_common_traits::database::Updatable<C, UserId = i32>,
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
        if !self.parent(conn)?.can_update(user_id, conn)? {
            return Ok(false);
        }
        if !self.predecessor(conn)?.can_update(user_id, conn)? {
            return Ok(false);
        }
        if !self.successor(conn)?.can_update(user_id, conn)? {
            return Ok(false);
        }
        Ok(true)
    }
}
