impl<
    C: diesel::connection::LoadConnection,
    ProcedureModel,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertablePlacingProcedureModelBuilder<
    ProcedureModel,
>
where
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization,
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::placing_procedure_models::PlacingProcedureModel as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertablePlacingProcedureModel as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::placing_procedure_models::PlacingProcedureModel as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::placing_procedure_models::PlacingProcedureModel,
    >,
    C: diesel::connection::LoadConnection,
    ProcedureModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
    crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableBuilder: web_common_traits::database::TryInsertGeneric<
        C,
        Attributes = crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelTrackableAttributes,
        PrimaryKey = i32,
    >,
{
    type Row = crate::codegen::structs_codegen::tables::placing_procedure_models::PlacingProcedureModel;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertablePlacingProcedureModel;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::InsertablePlacingProcedureModelAttributes,
    >;
    type UserId = i32;
    fn insert(
        self,
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertablePlacingProcedureModel = self
            .try_insert(user_id, conn)?;
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
        let placed_into = self
            .placed_into
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertablePlacingProcedureModelAttributes::PlacedInto,
                ),
            )?;
        let quantity = self
            .quantity
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertablePlacingProcedureModelAttributes::Quantity,
                ),
            )?;
        let procedure_model_id = self
            .procedure_model
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertablePlacingProcedureModelAttributes::Extension(
                    crate::codegen::structs_codegen::tables::insertables::InsertablePlacingProcedureModelExtensionAttributes::ProcedureModel(
                        crate::codegen::structs_codegen::tables::insertables::InsertableProcedureModelAttributes::Id,
                    ),
                ))
            })?;
        let source = self
            .source
            .procedure_model(procedure_model_id)
            .map_err(|err| {
                err.into_field_name(
                    crate::codegen::structs_codegen::tables::insertables::InsertablePlacingProcedureModelAttributes::Source,
                )
            })?
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(
                    crate::codegen::structs_codegen::tables::insertables::InsertablePlacingProcedureModelAttributes::Source,
                )
            })?;
        let procedure_placed_into = self
            .procedure_placed_into
            .procedure_model(procedure_model_id)
            .map_err(|err| {
                err.into_field_name(
                    crate::codegen::structs_codegen::tables::insertables::InsertablePlacingProcedureModelAttributes::ProcedurePlacedInto,
                )
            })?
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(
                    crate::codegen::structs_codegen::tables::insertables::InsertablePlacingProcedureModelAttributes::ProcedurePlacedInto,
                )
            })?;
        Ok(Self::InsertableVariant {
            procedure_model_id,
            source,
            placed_into,
            procedure_placed_into,
            quantity,
        })
    }
}
