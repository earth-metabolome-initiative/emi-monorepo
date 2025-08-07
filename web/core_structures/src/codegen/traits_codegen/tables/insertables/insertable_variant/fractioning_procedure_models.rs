impl<
    C: diesel::connection::LoadConnection,
    ProcedureModel,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureModelBuilder<
    ProcedureModel,
>
where
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization,
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::fractioning_procedure_models::FractioningProcedureModel as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureModel as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::fractioning_procedure_models::FractioningProcedureModel as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::fractioning_procedure_models::FractioningProcedureModel,
    >,
    C: diesel::connection::LoadConnection,
    ProcedureModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder: web_common_traits::database::TryInsertGeneric<
        C,
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes,
        PrimaryKey = i32,
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
{
    type Row = crate::codegen::structs_codegen::tables::fractioning_procedure_models::FractioningProcedureModel;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureModel;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureModelAttributes,
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
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureModel = self
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
        let kilograms = self
            .kilograms
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureModelAttributes::Kilograms,
                ),
            )?;
        let tolerance_percentage = self
            .tolerance_percentage
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureModelAttributes::TolerancePercentage,
                ),
            )?;
        let weighed_with = self
            .weighed_with
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureModelAttributes::WeighedWith,
                ),
            )?;
        let fragment_placed_into = self
            .fragment_placed_into
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureModelAttributes::FragmentPlacedInto,
                ),
            )?;
        let procedure_model_id = self
            .procedure_model
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureModelAttributes::Extension(
                    crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureModelExtensionAttributes::ProcedureModel(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelAttributes::Id,
                    ),
                ))
            })?;
        let procedure_fragment_placed_into = self
            .procedure_fragment_placed_into
            .procedure_model(procedure_model_id)
            .map_err(|err| {
                err.into_field_name(
                    crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureModelAttributes::ProcedureFragmentPlacedInto,
                )
            })?
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(
                    crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureModelAttributes::ProcedureFragmentPlacedInto,
                )
            })?;
        let procedure_fragment_source = self
            .procedure_fragment_source
            .procedure_model(procedure_model_id)
            .map_err(|err| {
                err.into_field_name(
                    crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureModelAttributes::ProcedureFragmentSource,
                )
            })?
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(
                    crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureModelAttributes::ProcedureFragmentSource,
                )
            })?;
        let procedure_weighed_with = self
            .procedure_weighed_with
            .procedure_model(procedure_model_id)
            .map_err(|err| {
                err.into_field_name(
                    crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureModelAttributes::ProcedureWeighedWith,
                )
            })?
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(
                    crate::codegen::structs_codegen::tables::insertables::InsertableFractioningProcedureModelAttributes::ProcedureWeighedWith,
                )
            })?;
        Ok(Self::InsertableVariant {
            procedure_model_id,
            kilograms,
            tolerance_percentage,
            weighed_with,
            procedure_weighed_with,
            procedure_fragment_source,
            fragment_placed_into,
            procedure_fragment_placed_into,
        })
    }
}
