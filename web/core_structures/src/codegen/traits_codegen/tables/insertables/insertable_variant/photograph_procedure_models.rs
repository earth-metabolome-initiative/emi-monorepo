impl<
    C: diesel::connection::LoadConnection,
    ProcedureModel,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureModelBuilder<
    ProcedureModel,
>
where
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization,
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::photograph_procedure_models::PhotographProcedureModel as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureModel as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::photograph_procedure_models::PhotographProcedureModel as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::photograph_procedure_models::PhotographProcedureModel,
    >,
    C: diesel::connection::LoadConnection,
    ProcedureModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
    crate::codegen::structs_codegen::tables::camera_models::CameraModel: diesel::Identifiable
        + web_common_traits::database::Updatable<C, UserId = i32>,
    <crate::codegen::structs_codegen::tables::camera_models::CameraModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::camera_models::CameraModel as diesel::Identifiable>::Id,
    >,
    <<crate::codegen::structs_codegen::tables::camera_models::CameraModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::camera_models::CameraModel as diesel::Identifiable>::Id,
    >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
    <<<crate::codegen::structs_codegen::tables::camera_models::CameraModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::camera_models::CameraModel as diesel::Identifiable>::Id,
    >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
        'a,
        C,
        crate::codegen::structs_codegen::tables::camera_models::CameraModel,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder: web_common_traits::database::TryInsertGeneric<
        C,
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes,
        PrimaryKey = i32,
    >,
{
    type Row = crate::codegen::structs_codegen::tables::photograph_procedure_models::PhotographProcedureModel;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureModel;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureModelAttributes,
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
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureModel = self
            .try_insert(user_id, conn)?;
        if !insertable_struct.photographed_with(conn)?.can_update(user_id, conn)? {
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
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::InsertableVariant, Self::Error> {
        use web_common_traits::database::TryInsertGeneric;
        let photographed_with = self
            .photographed_with
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureModelAttributes::PhotographedWith,
                ),
            )?;
        let procedure_model_id = self
            .procedure_model
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureModelAttributes::Extension(
                    crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureModelExtensionAttributes::ProcedureModel(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelAttributes::Id,
                    ),
                ))
            })?;
        let procedure_photographed_with = self
            .procedure_photographed_with
            .procedure_model(procedure_model_id)
            .map_err(|err| {
                err.into_field_name(
                    crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureModelAttributes::ProcedurePhotographedWith,
                )
            })?
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(
                    crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureModelAttributes::ProcedurePhotographedWith,
                )
            })?;
        let trackable_id = self
            .trackable_id
            .procedure_model(procedure_model_id)
            .map_err(|err| {
                err.into_field_name(
                    crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureModelAttributes::TrackableId,
                )
            })?
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(
                    crate::codegen::structs_codegen::tables::insertables::InsertablePhotographProcedureModelAttributes::TrackableId,
                )
            })?;
        Ok(Self::InsertableVariant {
            procedure_model_id,
            photographed_with,
            procedure_photographed_with,
            trackable_id,
        })
    }
}
