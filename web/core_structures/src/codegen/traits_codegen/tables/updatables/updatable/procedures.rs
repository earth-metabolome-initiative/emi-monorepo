impl<C: diesel::connection::LoadConnection> web_common_traits::database::Updatable<C>
    for crate::codegen::structs_codegen::tables::procedures::Procedure
where
    crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate:
        web_common_traits::database::Read<C>,
    crate::codegen::structs_codegen::tables::procedure_templates::ProcedureTemplate:
        web_common_traits::database::Updatable<C>,
{
    fn can_update(&self, user_id: i32, conn: &mut C) -> Result<bool, diesel::result::Error> {
        if user_id == self.created_by {
            return Ok(true);
        }
        if user_id == self.updated_by {
            return Ok(true);
        }
        if let Some(procedure_templates) = self.parent_procedure_template(conn)? {
            if !procedure_templates.can_update(user_id, conn)? {
                return Ok(false);
            }
        }
        if let Some(procedure_templates) = self.predecessor_procedure_template(conn)? {
            if !procedure_templates.can_update(user_id, conn)? {
                return Ok(false);
            }
        }
        if !self.procedure_template(conn)?.can_update(user_id, conn)? {
            return Ok(false);
        }
        Ok(true)
    }
}
