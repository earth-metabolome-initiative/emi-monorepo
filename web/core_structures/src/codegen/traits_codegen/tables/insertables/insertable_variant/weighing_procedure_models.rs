impl<
    C: diesel::connection::LoadConnection,
    ProcedureModel,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureModelBuilder<
    ProcedureModel,
>
where
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization,
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::weighing_procedure_models::WeighingProcedureModel as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureModel as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::weighing_procedure_models::WeighingProcedureModel as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::weighing_procedure_models::WeighingProcedureModel,
    >,
    crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel: diesel::Identifiable
        + web_common_traits::database::Updatable<C, UserId = i32>,
    <crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel as diesel::Identifiable>::Id,
    >,
    <<crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel as diesel::Identifiable>::Id,
    >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
    <<<crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel as diesel::Identifiable>::Id,
    >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
        'a,
        C,
        crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel,
    >,
    C: diesel::connection::LoadConnection,
    ProcedureModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder: web_common_traits::database::TryInsertGeneric<
        C,
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes,
        PrimaryKey = i32,
    >,
{
    type Row = crate::codegen::structs_codegen::tables::weighing_procedure_models::WeighingProcedureModel;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureModel;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureModelAttributes,
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
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureModel = self
            .try_insert(user_id, conn)?;
        if !insertable_struct.weighed_with(conn)?.can_update(user_id, conn)? {
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
        let weighed_with = self
            .weighed_with
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureModelAttributes::WeighedWith,
                ),
            )?;
        let sample_container_id = self
            .sample_container_id
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureModelAttributes::SampleContainerId,
                ),
            )?;
        let procedure_model_id = self
            .procedure_model
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureModelAttributes::Extension(
                    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureModelExtensionAttributes::ProcedureModel(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelAttributes::Id,
                    ),
                ))
            })?;
        let procedure_sample_container = self
            .procedure_sample_container
            .procedure_model(procedure_model_id)
            .map_err(|err| {
                err.into_field_name(
                    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureModelAttributes::ProcedureSampleContainer,
                )
            })?
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(
                    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureModelAttributes::ProcedureSampleContainer,
                )
            })?;
        let procedure_weighed_with = self
            .procedure_weighed_with
            .procedure_model(procedure_model_id)
            .map_err(|err| {
                err.into_field_name(
                    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureModelAttributes::ProcedureWeighedWith,
                )
            })?
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(
                    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingProcedureModelAttributes::ProcedureWeighedWith,
                )
            })?;
        Ok(Self::InsertableVariant {
            procedure_model_id,
            weighed_with,
            procedure_weighed_with,
            sample_container_id,
            procedure_sample_container,
        })
    }
}
