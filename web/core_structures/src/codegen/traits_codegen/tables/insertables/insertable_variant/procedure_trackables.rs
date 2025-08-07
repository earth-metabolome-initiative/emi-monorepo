impl<
    C: diesel::connection::LoadConnection,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTrackableBuilder
where
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization,
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTrackable as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable,
    >,
    crate::codegen::structs_codegen::tables::trackable_ancestors::TrackableAncestor: diesel::Identifiable
        + web_common_traits::database::Updatable<C, UserId = i32>,
    <crate::codegen::structs_codegen::tables::trackable_ancestors::TrackableAncestor as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::trackable_ancestors::TrackableAncestor as diesel::Identifiable>::Id,
    >,
    <<crate::codegen::structs_codegen::tables::trackable_ancestors::TrackableAncestor as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::trackable_ancestors::TrackableAncestor as diesel::Identifiable>::Id,
    >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
    <<<crate::codegen::structs_codegen::tables::trackable_ancestors::TrackableAncestor as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::trackable_ancestors::TrackableAncestor as diesel::Identifiable>::Id,
    >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
        'a,
        C,
        crate::codegen::structs_codegen::tables::trackable_ancestors::TrackableAncestor,
    >,
    C: diesel::connection::LoadConnection,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureBuilder: web_common_traits::database::TryInsertGeneric<
        C,
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureAttributes,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
{
    type Row = crate::codegen::structs_codegen::tables::procedure_trackables::ProcedureTrackable;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTrackable;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTrackableAttributes,
    >;
    type UserId = i32;
    fn insert(
        self,
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::Updatable;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTrackable = self
            .try_insert(user_id, conn)?;
        if !insertable_struct
            .procedure_trackables_trackable_id_ancestor_trackable_id_fkey(conn)?
            .can_update(user_id, conn)?
        {
            return Err(
                generic_backend_request_errors::GenericBackendRequestError::Unauthorized
                    .into(),
            );
        }
        Ok(
            diesel::insert_into(Self::Row::table())
                .values(insertable_struct)
                .get_result(conn)?,
        )
    }
    fn try_insert(
        self,
        _user_id: i32,
        _conn: &mut C,
    ) -> Result<Self::InsertableVariant, Self::Error> {
        let procedure_id = self
            .procedure_id
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTrackableAttributes::ProcedureId,
                ),
            )?;
        let procedure_model_id = self
            .procedure_model_id
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTrackableAttributes::ProcedureModelId,
                ),
            )?;
        let trackable_id = self
            .trackable_id
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTrackableAttributes::TrackableId,
                ),
            )?;
        let procedure_model_trackable_id = self
            .procedure_model_trackable_id
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTrackableAttributes::ProcedureModelTrackableId,
                ),
            )?;
        let ancestor_trackable_id = self
            .ancestor_trackable_id
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTrackableAttributes::AncestorTrackableId,
                ),
            )?;
        let parent_trackable_id = self
            .parent_trackable_id
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTrackableAttributes::ParentTrackableId,
                ),
            )?;
        let created_by = self
            .created_by
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTrackableAttributes::CreatedBy,
                ),
            )?;
        let created_at = self
            .created_at
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureTrackableAttributes::CreatedAt,
                ),
            )?;
        Ok(Self::InsertableVariant {
            procedure_id,
            procedure_model_id,
            trackable_id,
            procedure_model_trackable_id,
            ancestor_trackable_id,
            parent_trackable_id,
            created_by,
            created_at,
        })
    }
}
