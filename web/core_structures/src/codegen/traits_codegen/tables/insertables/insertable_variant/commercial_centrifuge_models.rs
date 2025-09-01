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
        <crate::codegen::structs_codegen::tables::commercial_centrifuge_models::CommercialCentrifugeModel as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCentrifugeModel as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::commercial_centrifuge_models::CommercialCentrifugeModel as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::commercial_centrifuge_models::CommercialCentrifugeModel,
    >,
    C: diesel::connection::LoadConnection,
    CentrifugeModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
    CommercialProduct: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
{
    type Row = crate::codegen::structs_codegen::tables::commercial_centrifuge_models::CommercialCentrifugeModel;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCentrifugeModel;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCentrifugeModelAttributes,
    >;
    type UserId = i32;
    fn insert(
        self,
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCentrifugeModel = self
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
        let id = if self.commercial_centrifuge_models_id_fkey.is_complete() {
            let id = self
                .commercial_centrifuge_models_id_fkey
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCentrifugeModelAttributes::Extension(
                        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCentrifugeModelExtensionAttributes::CentrifugeModel(
                            crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeModelAttributes::Id,
                        ),
                    ))
                })?;
            let _ = self
                .commercial_centrifuge_models_id_fkey1
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCentrifugeModelAttributes::Extension(
                        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCentrifugeModelExtensionAttributes::CommercialProduct(
                            crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductAttributes::Id,
                        ),
                    ))
                })?;
            id
        } else {
            let id = self
                .commercial_centrifuge_models_id_fkey1
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCentrifugeModelAttributes::Extension(
                        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCentrifugeModelExtensionAttributes::CommercialProduct(
                            crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductAttributes::Id,
                        ),
                    ))
                })?;
            let _ = self
                .commercial_centrifuge_models_id_fkey
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCentrifugeModelAttributes::Extension(
                        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCentrifugeModelExtensionAttributes::CentrifugeModel(
                            crate::codegen::structs_codegen::tables::insertables::InsertableCentrifugeModelAttributes::Id,
                        ),
                    ))
                })?;
            id
        };
        Ok(Self::InsertableVariant { id })
    }
}
