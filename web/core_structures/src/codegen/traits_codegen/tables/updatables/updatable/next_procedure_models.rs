impl<C: diesel::connection::LoadConnection> web_common_traits::database::Updatable<C>
for crate::codegen::structs_codegen::tables::next_procedure_models::NextProcedureModel
where
    crate::codegen::structs_codegen::tables::parent_procedure_models::ParentProcedureModel: diesel::Identifiable,
    <crate::codegen::structs_codegen::tables::parent_procedure_models::ParentProcedureModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::parent_procedure_models::ParentProcedureModel as diesel::Identifiable>::Id,
    >,
    <<crate::codegen::structs_codegen::tables::parent_procedure_models::ParentProcedureModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::parent_procedure_models::ParentProcedureModel as diesel::Identifiable>::Id,
    >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
    <<<crate::codegen::structs_codegen::tables::parent_procedure_models::ParentProcedureModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::parent_procedure_models::ParentProcedureModel as diesel::Identifiable>::Id,
    >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
        'a,
        C,
        crate::codegen::structs_codegen::tables::parent_procedure_models::ParentProcedureModel,
    >,
    crate::codegen::structs_codegen::tables::parent_procedure_models::ParentProcedureModel: web_common_traits::database::Updatable<
        C,
        UserId = i32,
    >,
    crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel: diesel::Identifiable,
    <crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel as diesel::Identifiable>::Id,
    >,
    <<crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel as diesel::Identifiable>::Id,
    >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
    <<<crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel as diesel::Identifiable>::Id,
    >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
        'a,
        C,
        crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel,
    >,
    crate::codegen::structs_codegen::tables::procedure_models::ProcedureModel: web_common_traits::database::Updatable<
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
        if user_id == self.created_by {
            return Ok(true);
        }
        if !self.parent(conn)?.can_update(user_id, conn)? {
            return Ok(false);
        }
        if !self.current(conn)?.can_update(user_id, conn)? {
            return Ok(false);
        }
        if !self.successor(conn)?.can_update(user_id, conn)? {
            return Ok(false);
        }
        if !self
            .next_procedure_models_parent_id_current_id_fkey(conn)?
            .can_update(user_id, conn)?
        {
            return Ok(false);
        }
        if !self
            .next_procedure_models_parent_id_successor_id_fkey(conn)?
            .can_update(user_id, conn)?
        {
            return Ok(false);
        }
        Ok(true)
    }
}
