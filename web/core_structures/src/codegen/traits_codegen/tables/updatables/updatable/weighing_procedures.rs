impl<C: diesel::connection::LoadConnection> web_common_traits::database::Updatable<C>
for crate::codegen::structs_codegen::tables::weighing_procedures::WeighingProcedure
where
    crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable: diesel::Identifiable,
    <crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable as diesel::Identifiable>::Id,
    >,
    <<crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable as diesel::Identifiable>::Id,
    >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
    <<<crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable as diesel::Identifiable>::Id,
    >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
        'a,
        C,
        crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable,
    >,
    crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable: web_common_traits::database::Updatable<
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
        if !self
            .weighing_procedures_procedure_id_weighted_with_fkey(conn)?
            .can_update(user_id, conn)?
        {
            return Ok(false);
        }
        if !self
            .weighing_procedures_procedure_id_weighted_container_id_fkey(conn)?
            .can_update(user_id, conn)?
        {
            return Ok(false);
        }
        Ok(true)
    }
}
