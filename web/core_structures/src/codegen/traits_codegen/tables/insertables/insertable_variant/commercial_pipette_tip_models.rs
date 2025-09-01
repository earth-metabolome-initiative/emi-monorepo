impl<
    C: diesel::connection::LoadConnection,
    PipetteTipModel,
    CommercialProduct,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipModelBuilder<
    PipetteTipModel,
    CommercialProduct,
>
where
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization,
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::commercial_pipette_tip_models::CommercialPipetteTipModel as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipModel as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::commercial_pipette_tip_models::CommercialPipetteTipModel as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::commercial_pipette_tip_models::CommercialPipetteTipModel,
    >,
    C: diesel::connection::LoadConnection,
    CommercialProduct: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    PipetteTipModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
{
    type Row = crate::codegen::structs_codegen::tables::commercial_pipette_tip_models::CommercialPipetteTipModel;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipModel;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipModelAttributes,
    >;
    type UserId = i32;
    fn insert(
        self,
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipModel = self
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
        let id = if self.commercial_pipette_tip_models_id_fkey1.is_complete() {
            let id = self
                .commercial_pipette_tip_models_id_fkey1
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipModelAttributes::Extension(
                        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipModelExtensionAttributes::CommercialProduct(
                            crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductAttributes::Id,
                        ),
                    ))
                })?;
            let _ = self
                .commercial_pipette_tip_models_id_fkey
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipModelAttributes::Extension(
                        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipModelExtensionAttributes::PipetteTipModel(
                            crate::codegen::structs_codegen::tables::insertables::InsertablePipetteTipModelAttributes::Id,
                        ),
                    ))
                })?;
            id
        } else {
            let id = self
                .commercial_pipette_tip_models_id_fkey
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipModelAttributes::Extension(
                        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipModelExtensionAttributes::PipetteTipModel(
                            crate::codegen::structs_codegen::tables::insertables::InsertablePipetteTipModelAttributes::Id,
                        ),
                    ))
                })?;
            let _ = self
                .commercial_pipette_tip_models_id_fkey1
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipModelAttributes::Extension(
                        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipModelExtensionAttributes::CommercialProduct(
                            crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductAttributes::Id,
                        ),
                    ))
                })?;
            id
        };
        Ok(Self::InsertableVariant { id })
    }
}
