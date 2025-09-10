impl<C: diesel::connection::LoadConnection> web_common_traits::database::Updatable<C>
    for crate::codegen::structs_codegen::tables::parent_procedure_templates::ParentProcedureTemplate
where
    crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate:
        web_common_traits::database::Read<C>,
    crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate:
        web_common_traits::database::Updatable<C, UserId = i32>,
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
        if !self.child(conn)?.can_update(user_id, conn)? {
            return Ok(false);
        }
        if !self.parent(conn)?.can_update(user_id, conn)? {
            return Ok(false);
        }
        Ok(true)
    }
}
