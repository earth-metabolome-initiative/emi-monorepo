impl<
    C: diesel::connection::LoadConnection,
    ProcedureModel,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureModelBuilder<
    ProcedureModel,
>
where
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization,
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::ball_mill_procedure_models::BallMillProcedureModel as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureModel as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::ball_mill_procedure_models::BallMillProcedureModel as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::ball_mill_procedure_models::BallMillProcedureModel,
    >,
    crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel: diesel::Identifiable
        + web_common_traits::database::Updatable<C, UserId = i32>,
    <crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel as diesel::Identifiable>::Id,
    >,
    <<crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel as diesel::Identifiable>::Id,
    >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
    <<<crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel as diesel::Identifiable>::Id,
    >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
        'a,
        C,
        crate::codegen::structs_codegen::tables::ball_mill_machine_models::BallMillMachineModel,
    >,
    C: diesel::connection::LoadConnection,
    ProcedureModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder: web_common_traits::database::TryInsertGeneric<
        C,
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes,
        PrimaryKey = i32,
    >,
{
    type Row = crate::codegen::structs_codegen::tables::ball_mill_procedure_models::BallMillProcedureModel;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureModel;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureModelAttributes,
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
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureModel = self
            .try_insert(user_id, conn)?;
        if !insertable_struct.milled_with(conn)?.can_update(user_id, conn)? {
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
        let kelvin = self
            .kelvin
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureModelAttributes::Kelvin,
                ),
            )?;
        let kelvin_tolerance_percentage = self
            .kelvin_tolerance_percentage
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureModelAttributes::KelvinTolerancePercentage,
                ),
            )?;
        let seconds = self
            .seconds
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureModelAttributes::Seconds,
                ),
            )?;
        let hertz = self
            .hertz
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureModelAttributes::Hertz,
                ),
            )?;
        let milled_with = self
            .milled_with
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureModelAttributes::MilledWith,
                ),
            )?;
        let milled_container_id = self
            .milled_container_id
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureModelAttributes::MilledContainerId,
                ),
            )?;
        let procedure_model_id = self
            .procedure_model
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureModelAttributes::Extension(
                    crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureModelExtensionAttributes::ProcedureModel(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelAttributes::Id,
                    ),
                ))
            })?;
        let procedure_milled_container_id = self
            .procedure_milled_container_id
            .procedure_model(procedure_model_id)
            .map_err(|err| {
                err.into_field_name(
                    crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureModelAttributes::ProcedureMilledContainerId,
                )
            })?
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(
                    crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureModelAttributes::ProcedureMilledContainerId,
                )
            })?;
        let procedure_milled_with = self
            .procedure_milled_with
            .procedure_model(procedure_model_id)
            .map_err(|err| {
                err.into_field_name(
                    crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureModelAttributes::ProcedureMilledWith,
                )
            })?
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(
                    crate::codegen::structs_codegen::tables::insertables::InsertableBallMillProcedureModelAttributes::ProcedureMilledWith,
                )
            })?;
        Ok(Self::InsertableVariant {
            procedure_model_id,
            kelvin,
            kelvin_tolerance_percentage,
            seconds,
            hertz,
            milled_with,
            procedure_milled_with,
            milled_container_id,
            procedure_milled_container_id,
        })
    }
}
