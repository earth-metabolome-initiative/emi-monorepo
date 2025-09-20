impl<
    CommercialProductLot,
    PipetteTipModel,
> web_common_traits::database::DispatchableInsertVariantMetadata
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipLotBuilder<
    CommercialProductLot,
    PipetteTipModel,
> {
    type Row = crate::codegen::structs_codegen::tables::commercial_pipette_tip_lots::CommercialPipetteTipLot;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::CommercialPipetteTipLotAttribute,
    >;
}
impl<
    CommercialProductLot,
    PipetteTipModel,
> web_common_traits::database::InsertableVariantMetadata
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipLotBuilder<
    CommercialProductLot,
    PipetteTipModel,
> {
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipLot;
}
#[cfg(feature = "backend")]
impl<
    CommercialProductLot,
    PipetteTipModel,
> web_common_traits::database::BackendInsertableVariant
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipLotBuilder<
    CommercialProductLot,
    PipetteTipModel,
>
where
    Self: web_common_traits::database::DispatchableInsertableVariant<
        diesel::PgConnection,
    >,
{}
impl<
    C: diesel::connection::LoadConnection,
    CommercialProductLot,
    PipetteTipModel,
> web_common_traits::database::DispatchableInsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipLotBuilder<
    CommercialProductLot,
    PipetteTipModel,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::commercial_pipette_tip_lots::CommercialPipetteTipLot as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipLot as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::commercial_pipette_tip_lots::CommercialPipetteTipLot as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::commercial_pipette_tip_lots::CommercialPipetteTipLot,
    >,
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipLot,
        Row = crate::codegen::structs_codegen::tables::commercial_pipette_tip_lots::CommercialPipetteTipLot,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialPipetteTipLotAttribute,
        >,
    >,
    Self: web_common_traits::database::MostConcreteTable,
{
    fn insert(mut self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        use web_common_traits::database::MostConcreteTable;
        self.set_most_concrete_table("commercial_pipette_tip_lots");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipLot = self
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
    CommercialProductLot,
    PipetteTipModel,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipLotBuilder<
    CommercialProductLot,
    PipetteTipModel,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::commercial_pipette_tip_lots::CommercialPipetteTipLot as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteTipLot as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::commercial_pipette_tip_lots::CommercialPipetteTipLot as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::commercial_pipette_tip_lots::CommercialPipetteTipLot,
    >,
    Self::Error: web_common_traits::database::FromExtension<
            <CommercialProductLot as web_common_traits::database::TryInsertGeneric<
                C,
            >>::Error,
        >
        + web_common_traits::database::FromExtension<
            <PipetteTipModel as web_common_traits::database::TryInsertGeneric<C>>::Error,
        >,
    CommercialProductLot: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    PipetteTipModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
{
    fn try_insert(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::InsertableVariant, Self::Error> {
        use web_common_traits::database::FromExtension;
        let product_model = self
            .product_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::CommercialPipetteTipLotAttribute::ProductModel,
                ),
            )?;
        let id = if self.commercial_pipette_tip_lots_id_fkey.is_complete() {
            let id = self
                .commercial_pipette_tip_lots_id_fkey
                .mint_primary_key(user_id, conn)
                .map_err(Self::Error::from_extension)?;
            let _ = self
                .commercial_pipette_tip_lots_id_fkey1
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(Self::Error::from_extension)?;
            id
        } else {
            let id = self
                .commercial_pipette_tip_lots_id_fkey1
                .mint_primary_key(user_id, conn)
                .map_err(Self::Error::from_extension)?;
            let _ = self
                .commercial_pipette_tip_lots_id_fkey
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(Self::Error::from_extension)?;
            id
        };
        Ok(Self::InsertableVariant {
            id,
            product_model,
        })
    }
}
