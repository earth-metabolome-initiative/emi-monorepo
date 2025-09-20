impl<
    PipetteTipModel,
    CommercialProduct,
> web_common_traits::database::DispatchableInsertVariantMetadata
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipModelBuilder<
    PipetteTipModel,
    CommercialProduct,
> {
    type Row = crate::codegen::structs_codegen::tables::commercial_pipette_tip_models::CommercialPipetteTipModel;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::CommercialPipetteTipModelAttribute,
    >;
}
impl<
    PipetteTipModel,
    CommercialProduct,
> web_common_traits::database::InsertableVariantMetadata
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipModelBuilder<
    PipetteTipModel,
    CommercialProduct,
> {
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipModel;
}
#[cfg(feature = "backend")]
impl<
    PipetteTipModel,
    CommercialProduct,
> web_common_traits::database::BackendInsertableVariant
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipModelBuilder<
    PipetteTipModel,
    CommercialProduct,
>
where
    Self: web_common_traits::database::DispatchableInsertableVariant<
        diesel::PgConnection,
    >,
{}
impl<
    C: diesel::connection::LoadConnection,
    PipetteTipModel,
    CommercialProduct,
> web_common_traits::database::DispatchableInsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipModelBuilder<
    PipetteTipModel,
    CommercialProduct,
>
where
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
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipModel,
        Row = crate::codegen::structs_codegen::tables::commercial_pipette_tip_models::CommercialPipetteTipModel,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialPipetteTipModelAttribute,
        >,
    >,
    Self: web_common_traits::database::MostConcreteTable,
{
    fn insert(mut self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        use web_common_traits::database::MostConcreteTable;
        self.set_most_concrete_table("commercial_pipette_tip_models");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipModel = self
            .try_insert(user_id, conn)?;
        Ok(
            diesel::insert_into(Self::table())
                .values(insertable_struct)
                .get_result(conn)?,
        )
    }
}
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
    Self::Error: web_common_traits::database::FromExtension<
            <PipetteTipModel as web_common_traits::database::TryInsertGeneric<C>>::Error,
        >
        + web_common_traits::database::FromExtension<
            <CommercialProduct as web_common_traits::database::TryInsertGeneric<
                C,
            >>::Error,
        >,
    PipetteTipModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
    CommercialProduct: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
{
    fn try_insert(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::InsertableVariant, Self::Error> {
        use web_common_traits::database::FromExtension;
        let pipette_tip_model = self
            .pipette_tip_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::CommercialPipetteTipModelAttribute::PipetteTipModel,
                ),
            )?;
        let id = if self.commercial_pipette_tip_models_id_fkey.is_complete() {
            let id = self
                .commercial_pipette_tip_models_id_fkey
                .mint_primary_key(user_id, conn)
                .map_err(Self::Error::from_extension)?;
            let _ = self
                .commercial_pipette_tip_models_id_fkey1
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(Self::Error::from_extension)?;
            id
        } else {
            let id = self
                .commercial_pipette_tip_models_id_fkey1
                .mint_primary_key(user_id, conn)
                .map_err(Self::Error::from_extension)?;
            let _ = self
                .commercial_pipette_tip_models_id_fkey
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(Self::Error::from_extension)?;
            id
        };
        Ok(Self::InsertableVariant {
            id,
            pipette_tip_model,
        })
    }
}
