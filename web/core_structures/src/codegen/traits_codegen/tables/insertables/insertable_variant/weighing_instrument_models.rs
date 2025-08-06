impl<
    C: diesel::connection::LoadConnection,
    InstrumentModel,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableWeighingInstrumentModelBuilder<
    InstrumentModel,
>
where
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization,
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableWeighingInstrumentModel as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel,
    >,
    C: diesel::connection::LoadConnection,
    InstrumentModel: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
{
    type Row = crate::codegen::structs_codegen::tables::weighing_instrument_models::WeighingInstrumentModel;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableWeighingInstrumentModel;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::InsertableWeighingInstrumentModelAttributes,
    >;
    type UserId = i32;
    fn insert(
        self,
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableWeighingInstrumentModel = self
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
        let id = self
            .id
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertableWeighingInstrumentModelAttributes::Extension(
                    crate::codegen::structs_codegen::tables::insertables::InsertableWeighingInstrumentModelExtensionAttributes::InstrumentModel(
                        crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentModelAttributes::Id,
                    ),
                ))
            })?;
        Ok(Self::InsertableVariant { id })
    }
}
