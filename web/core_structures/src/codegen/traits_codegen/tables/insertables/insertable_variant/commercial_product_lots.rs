impl<PhysicalAssetModel> web_common_traits::database::DispatchableInsertVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotBuilder<
        PhysicalAssetModel,
    >
{
    type Row =
        crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::CommercialProductLotAttribute,
    >;
}
impl<PhysicalAssetModel> web_common_traits::database::InsertableVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotBuilder<
        PhysicalAssetModel,
    >
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLot;
}
#[cfg(feature = "backend")]
impl<PhysicalAssetModel> web_common_traits::database::BackendInsertableVariant
    for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotBuilder<
        PhysicalAssetModel,
    >
where
    Self: web_common_traits::database::DispatchableInsertableVariant<diesel::PgConnection>,
{
}
impl<
    C: diesel::connection::LoadConnection,
    PhysicalAssetModel,
> web_common_traits::database::DispatchableInsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotBuilder<
    PhysicalAssetModel,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLot as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot,
    >,
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLot,
        Row = crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialProductLotAttribute,
        >,
    >,
    Self: web_common_traits::database::MostConcreteTable,
{
    fn insert(mut self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        use web_common_traits::database::MostConcreteTable;
        self.set_most_concrete_table("commercial_product_lots");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLot = self
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
    PhysicalAssetModel,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLotBuilder<
    PhysicalAssetModel,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductLot as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::commercial_product_lots::CommercialProductLot,
    >,
    Self::Error: web_common_traits::database::FromExtension<
        <PhysicalAssetModel as web_common_traits::database::TryInsertGeneric<C>>::Error,
    >,
    PhysicalAssetModel: web_common_traits::database::TryInsertGeneric<
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
        let lot = self
            .lot
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::CommercialProductLotAttribute::Lot,
                ),
            )?;
        let product_model = self
            .product_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::CommercialProductLotAttribute::ProductModel,
                ),
            )?;
        let id = self
            .id
            .mint_primary_key(user_id, conn)
            .map_err(Self::Error::from_extension)?;
        Ok(Self::InsertableVariant {
            id,
            lot,
            product_model,
        })
    }
}
