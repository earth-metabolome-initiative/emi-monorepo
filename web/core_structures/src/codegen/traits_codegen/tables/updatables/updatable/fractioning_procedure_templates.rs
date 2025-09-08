impl<C: diesel::connection::LoadConnection> web_common_traits::database::Updatable<C>
    for crate::FractioningProcedureTemplate
where
    crate::ProcedureTemplate: web_common_traits::database::Read<C>,
    crate::ProcedureTemplate: web_common_traits::database::Updatable<C, UserId = i32>,
    crate::ProcedureTemplateAssetModel: web_common_traits::database::Read<C>,
    crate::ProcedureTemplateAssetModel: web_common_traits::database::Updatable<C, UserId = i32>,
{
    type UserId = i32;
    fn can_update(
        &self,
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<bool, diesel::result::Error> {
        if !self.procedure_template(conn)?.can_update(user_id, conn)? {
            return Ok(false);
        }
        if !self.procedure_template_weighed_with_model(conn)?.can_update(user_id, conn)? {
            return Ok(false);
        }
        if !self.procedure_template_fragment_placed_into_model(conn)?.can_update(user_id, conn)? {
            return Ok(false);
        }
        Ok(true)
    }
}
