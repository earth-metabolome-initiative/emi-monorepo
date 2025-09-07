impl<
    C: diesel::connection::LoadConnection,
    CentrifugeModel,
    CommercialProduct,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCentrifugeModelBuilder<
    CentrifugeModel,
    CommercialProduct,
>
where
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization,
    diesel::query_builder::InsertStatement<
        <crate::CommercialCentrifugeModel as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCentrifugeModel as diesel::Insertable<
            <crate::CommercialCentrifugeModel as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::CommercialCentrifugeModel,
    >,
    C: diesel::connection::LoadConnection,
    CentrifugeModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
    CommercialProduct: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    crate::CentrifugeModel: diesel::Identifiable
        + web_common_traits::database::Updatable<C, UserId = i32>,
    <crate::CentrifugeModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
        <crate::CentrifugeModel as diesel::Identifiable>::Id,
    >,
    <<crate::CentrifugeModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::CentrifugeModel as diesel::Identifiable>::Id,
    >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
    <<<crate::CentrifugeModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::CentrifugeModel as diesel::Identifiable>::Id,
    >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
        'a,
        C,
        crate::CentrifugeModel,
    >,
    Self: web_common_traits::database::MostConcreteTable,
{
    type Row = crate::CommercialCentrifugeModel;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCentrifugeModel;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::CommercialCentrifugeModelAttribute,
    >;
    type UserId = i32;
    fn insert(
        mut self,
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::Updatable;
        use web_common_traits::database::MostConcreteTable;
        self.set_most_concrete_table("commercial_centrifuge_models");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCentrifugeModel = self
            .try_insert(user_id, conn)?;
        if !insertable_struct.centrifuge_model(conn)?.can_update(user_id, conn)? {
            return Err(
                generic_backend_request_errors::GenericBackendRequestError::Unauthorized
                    .into(),
            );
        }
        if !insertable_struct
            .commercial_centrifuge_models_id_fkey(conn)?
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
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::InsertableVariant, Self::Error> {
        let centrifuge_model = self
            .centrifuge_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::CommercialCentrifugeModelAttribute::CentrifugeModel,
                ),
            )?;
        let id = if self.commercial_centrifuge_models_id_fkey.is_complete() {
            let id = self
                .commercial_centrifuge_models_id_fkey
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::CommercialCentrifugeModelAttribute::Extension(
                        crate::codegen::structs_codegen::tables::insertables::CommercialCentrifugeModelExtensionAttribute::CentrifugeModel(
                            crate::codegen::structs_codegen::tables::insertables::CentrifugeModelAttribute::Id,
                        ),
                    ))
                })?;
            let _ = self
                .commercial_centrifuge_models_id_fkey1
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::CommercialCentrifugeModelAttribute::Extension(
                        crate::codegen::structs_codegen::tables::insertables::CommercialCentrifugeModelExtensionAttribute::CommercialProduct(
                            crate::codegen::structs_codegen::tables::insertables::CommercialProductAttribute::Id,
                        ),
                    ))
                })?;
            id
        } else {
            let id = self
                .commercial_centrifuge_models_id_fkey1
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::CommercialCentrifugeModelAttribute::Extension(
                        crate::codegen::structs_codegen::tables::insertables::CommercialCentrifugeModelExtensionAttribute::CommercialProduct(
                            crate::codegen::structs_codegen::tables::insertables::CommercialProductAttribute::Id,
                        ),
                    ))
                })?;
            let _ = self
                .commercial_centrifuge_models_id_fkey
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::CommercialCentrifugeModelAttribute::Extension(
                        crate::codegen::structs_codegen::tables::insertables::CommercialCentrifugeModelExtensionAttribute::CentrifugeModel(
                            crate::codegen::structs_codegen::tables::insertables::CentrifugeModelAttribute::Id,
                        ),
                    ))
                })?;
            id
        };
        Ok(Self::InsertableVariant {
            id,
            centrifuge_model,
        })
    }
}
