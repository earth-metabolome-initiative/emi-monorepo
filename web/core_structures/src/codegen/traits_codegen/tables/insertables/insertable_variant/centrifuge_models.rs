impl<
    C: diesel::connection::LoadConnection,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeModelBuilder
where
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization,
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeModel as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel,
    >,
    crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel: diesel::Identifiable
        + web_common_traits::database::Updatable<C, UserId = i32>,
    <crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel as diesel::Identifiable>::Id,
    >,
    <<crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel as diesel::Identifiable>::Id,
    >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
    <<<crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel as diesel::Identifiable>::Id,
    >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
        'a,
        C,
        crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel,
    >,
    crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentModelBuilder: web_common_traits::database::InsertableVariant<
        C,
        UserId = i32,
        Row = crate::codegen::structs_codegen::tables::instrument_models::InstrumentModel,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::InsertableInstrumentModelAttributes,
        >,
    >,
{
    type Row = crate::codegen::structs_codegen::tables::centrifuge_models::CentrifugeModel;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeModel;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeModelAttributes,
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
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeModel = self
            .try_insert(user_id, conn)?;
        if !insertable_struct.id(conn)?.can_update(user_id, conn)? {
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
}
