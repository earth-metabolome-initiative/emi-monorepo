impl<CommercialProductLot, PipetteModel>
    web_common_traits::database::DispatchableInsertVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteLotBuilder<
        CommercialProductLot,
        PipetteModel,
    >
{
    type Row =
        crate::codegen::structs_codegen::tables::commercial_pipette_lots::CommercialPipetteLot;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::CommercialPipetteLotAttribute,
    >;
}
impl<CommercialProductLot, PipetteModel> web_common_traits::database::InsertableVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteLotBuilder<
        CommercialProductLot,
        PipetteModel,
    >
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteLot;
}
#[cfg(feature = "backend")]
impl<CommercialProductLot, PipetteModel> web_common_traits::database::BackendInsertableVariant
    for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteLotBuilder<
        CommercialProductLot,
        PipetteModel,
    >
where
    Self: web_common_traits::database::DispatchableInsertableVariant<diesel::PgConnection>,
{
}
impl<
    C: diesel::connection::LoadConnection,
    CommercialProductLot,
    PipetteModel,
> web_common_traits::database::DispatchableInsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteLotBuilder<
    CommercialProductLot,
    PipetteModel,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::commercial_pipette_lots::CommercialPipetteLot as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteLot as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::commercial_pipette_lots::CommercialPipetteLot as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::commercial_pipette_lots::CommercialPipetteLot,
    >,
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteLot,
        Row = crate::codegen::structs_codegen::tables::commercial_pipette_lots::CommercialPipetteLot,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialPipetteLotAttribute,
        >,
    >,
    CommercialProductLot: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    PipetteModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
    Self: web_common_traits::database::MostConcreteTable,
    crate::codegen::structs_codegen::tables::insertables::CommercialPipetteLotExtensionAttribute: From<
        <CommercialProductLot as common_traits::builder::Attributed>::Attribute,
    >,
    crate::codegen::structs_codegen::tables::insertables::CommercialPipetteLotExtensionAttribute: From<
        <PipetteModel as common_traits::builder::Attributed>::Attribute,
    >,
{
    fn insert(mut self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        use web_common_traits::database::MostConcreteTable;
        self.set_most_concrete_table("commercial_pipette_lots");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteLot = self
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
    PipetteModel,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteLotBuilder<
    CommercialProductLot,
    PipetteModel,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::commercial_pipette_lots::CommercialPipetteLot as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPipetteLot as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::commercial_pipette_lots::CommercialPipetteLot as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::commercial_pipette_lots::CommercialPipetteLot,
    >,
    CommercialProductLot: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    PipetteModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
    Self: web_common_traits::database::MostConcreteTable,
    crate::codegen::structs_codegen::tables::insertables::CommercialPipetteLotExtensionAttribute: From<
        <CommercialProductLot as common_traits::builder::Attributed>::Attribute,
    >,
    crate::codegen::structs_codegen::tables::insertables::CommercialPipetteLotExtensionAttribute: From<
        <PipetteModel as common_traits::builder::Attributed>::Attribute,
    >,
{
    fn try_insert(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::InsertableVariant, Self::Error> {
        let product_model = self
            .product_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::CommercialPipetteLotAttribute::ProductModel,
                ),
            )?;
        let id = if self.commercial_pipette_lots_id_fkey.is_complete() {
            let id = self
                .commercial_pipette_lots_id_fkey
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|attribute| {
                        crate::codegen::structs_codegen::tables::insertables::CommercialPipetteLotAttribute::Extension(
                            From::from(attribute),
                        )
                    })
                })?;
            let _ = self
                .commercial_pipette_lots_id_fkey1
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|attribute| {
                        crate::codegen::structs_codegen::tables::insertables::CommercialPipetteLotAttribute::Extension(
                            From::from(attribute),
                        )
                    })
                })?;
            id
        } else {
            let id = self
                .commercial_pipette_lots_id_fkey1
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|attribute| {
                        crate::codegen::structs_codegen::tables::insertables::CommercialPipetteLotAttribute::Extension(
                            From::from(attribute),
                        )
                    })
                })?;
            let _ = self
                .commercial_pipette_lots_id_fkey
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|attribute| {
                        crate::codegen::structs_codegen::tables::insertables::CommercialPipetteLotAttribute::Extension(
                            From::from(attribute),
                        )
                    })
                })?;
            id
        };
        Ok(Self::InsertableVariant {
            id,
            product_model,
        })
    }
}
