impl<
    C: diesel::connection::LoadConnection,
    ProcedureModel,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureModelBuilder<
    ProcedureModel,
>
where
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization,
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::aliquoting_procedure_models::AliquotingProcedureModel as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureModel as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::aliquoting_procedure_models::AliquotingProcedureModel as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::aliquoting_procedure_models::AliquotingProcedureModel,
    >,
    crate::codegen::structs_codegen::tables::pipette_models::PipetteModel: diesel::Identifiable
        + web_common_traits::database::Updatable<C, UserId = i32>,
    <crate::codegen::structs_codegen::tables::pipette_models::PipetteModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::pipette_models::PipetteModel as diesel::Identifiable>::Id,
    >,
    <<crate::codegen::structs_codegen::tables::pipette_models::PipetteModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::pipette_models::PipetteModel as diesel::Identifiable>::Id,
    >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
    <<<crate::codegen::structs_codegen::tables::pipette_models::PipetteModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::pipette_models::PipetteModel as diesel::Identifiable>::Id,
    >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
        'a,
        C,
        crate::codegen::structs_codegen::tables::pipette_models::PipetteModel,
    >,
    crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel: diesel::Identifiable
        + web_common_traits::database::Updatable<C, UserId = i32>,
    <crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel as diesel::Identifiable>::Id,
    >,
    <<crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel as diesel::Identifiable>::Id,
    >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
    <<<crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel as diesel::Identifiable>::Id,
    >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
        'a,
        C,
        crate::codegen::structs_codegen::tables::pipette_tip_models::PipetteTipModel,
    >,
    C: diesel::connection::LoadConnection,
    ProcedureModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder: web_common_traits::database::TryInsertGeneric<
        C,
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes,
        PrimaryKey = i32,
    >,
{
    type Row = crate::codegen::structs_codegen::tables::aliquoting_procedure_models::AliquotingProcedureModel;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureModel;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureModelAttributes,
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
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureModel = self
            .try_insert(user_id, conn)?;
        if !insertable_struct.aliquoted_with(conn)?.can_update(user_id, conn)? {
            return Err(
                generic_backend_request_errors::GenericBackendRequestError::Unauthorized
                    .into(),
            );
        }
        if !insertable_struct.pipette_tip(conn)?.can_update(user_id, conn)? {
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
        let liters = self
            .liters
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureModelAttributes::Liters,
                ),
            )?;
        let aliquoted_from = self
            .aliquoted_from
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureModelAttributes::AliquotedFrom,
                ),
            )?;
        let aliquoted_into = self
            .aliquoted_into
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureModelAttributes::AliquotedInto,
                ),
            )?;
        let aliquoted_with = self
            .aliquoted_with
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureModelAttributes::AliquotedWith,
                ),
            )?;
        let pipette_tip = self
            .pipette_tip
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureModelAttributes::PipetteTip,
                ),
            )?;
        let procedure_model_id = self
            .procedure_model
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureModelAttributes::Extension(
                    crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureModelExtensionAttributes::ProcedureModel(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelAttributes::Id,
                    ),
                ))
            })?;
        let procedure_aliquoted_with = self
            .procedure_aliquoted_with
            .procedure_model(procedure_model_id)
            .map_err(|err| {
                err.into_field_name(
                    crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureModelAttributes::ProcedureAliquotedWith,
                )
            })?
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(
                    crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureModelAttributes::ProcedureAliquotedWith,
                )
            })?;
        let procedure_aliquoted_from = self
            .procedure_aliquoted_from
            .procedure_model(procedure_model_id)
            .map_err(|err| {
                err.into_field_name(
                    crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureModelAttributes::ProcedureAliquotedFrom,
                )
            })?
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(
                    crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureModelAttributes::ProcedureAliquotedFrom,
                )
            })?;
        let procedure_aliquoted_into = self
            .procedure_aliquoted_into
            .procedure_model(procedure_model_id)
            .map_err(|err| {
                err.into_field_name(
                    crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureModelAttributes::ProcedureAliquotedInto,
                )
            })?
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(
                    crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureModelAttributes::ProcedureAliquotedInto,
                )
            })?;
        let procedure_pipette_tip = self
            .procedure_pipette_tip
            .procedure_model(procedure_model_id)
            .map_err(|err| {
                err.into_field_name(
                    crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureModelAttributes::ProcedurePipetteTip,
                )
            })?
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(
                    crate::codegen::structs_codegen::tables::insertables::InsertableAliquotingProcedureModelAttributes::ProcedurePipetteTip,
                )
            })?;
        Ok(Self::InsertableVariant {
            procedure_model_id,
            liters,
            aliquoted_from,
            procedure_aliquoted_from,
            aliquoted_into,
            procedure_aliquoted_into,
            aliquoted_with,
            procedure_aliquoted_with,
            pipette_tip,
            procedure_pipette_tip,
        })
    }
}
