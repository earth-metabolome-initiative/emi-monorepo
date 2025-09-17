impl<
    CommercialProductLot,
    FreezeDryerModel,
> web_common_traits::database::DispatchableInsertVariantMetadata
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezeDryerLotBuilder<
    CommercialProductLot,
    FreezeDryerModel,
> {
    type Row = crate::codegen::structs_codegen::tables::commercial_freeze_dryer_lots::CommercialFreezeDryerLot;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::CommercialFreezeDryerLotAttribute,
    >;
}
impl<
    CommercialProductLot,
    FreezeDryerModel,
> web_common_traits::database::InsertableVariantMetadata
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezeDryerLotBuilder<
    CommercialProductLot,
    FreezeDryerModel,
> {
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezeDryerLot;
}
#[cfg(feature = "backend")]
impl<
    CommercialProductLot,
    FreezeDryerModel,
> web_common_traits::database::BackendInsertableVariant
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezeDryerLotBuilder<
    CommercialProductLot,
    FreezeDryerModel,
>
where
    Self: web_common_traits::database::DispatchableInsertableVariant<
        diesel::PgConnection,
    >,
{}
impl<
    C: diesel::connection::LoadConnection,
    CommercialProductLot,
    FreezeDryerModel,
> web_common_traits::database::DispatchableInsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezeDryerLotBuilder<
    CommercialProductLot,
    FreezeDryerModel,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::commercial_freeze_dryer_lots::CommercialFreezeDryerLot as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezeDryerLot as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::commercial_freeze_dryer_lots::CommercialFreezeDryerLot as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::commercial_freeze_dryer_lots::CommercialFreezeDryerLot,
    >,
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezeDryerLot,
        Row = crate::codegen::structs_codegen::tables::commercial_freeze_dryer_lots::CommercialFreezeDryerLot,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialFreezeDryerLotAttribute,
        >,
    >,
    CommercialProductLot: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    FreezeDryerModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
    Self: web_common_traits::database::MostConcreteTable,
    crate::codegen::structs_codegen::tables::insertables::CommercialFreezeDryerLotExtensionAttribute: From<
        <CommercialProductLot as common_traits::builder::Attributed>::Attribute,
    >,
    crate::codegen::structs_codegen::tables::insertables::CommercialFreezeDryerLotExtensionAttribute: From<
        <FreezeDryerModel as common_traits::builder::Attributed>::Attribute,
    >,
{
    fn insert(mut self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        use web_common_traits::database::MostConcreteTable;
        self.set_most_concrete_table("commercial_freeze_dryer_lots");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezeDryerLot = self
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
    FreezeDryerModel,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezeDryerLotBuilder<
    CommercialProductLot,
    FreezeDryerModel,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::commercial_freeze_dryer_lots::CommercialFreezeDryerLot as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezeDryerLot as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::commercial_freeze_dryer_lots::CommercialFreezeDryerLot as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::commercial_freeze_dryer_lots::CommercialFreezeDryerLot,
    >,
    CommercialProductLot: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    FreezeDryerModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
    Self: web_common_traits::database::MostConcreteTable,
    crate::codegen::structs_codegen::tables::insertables::CommercialFreezeDryerLotExtensionAttribute: From<
        <CommercialProductLot as common_traits::builder::Attributed>::Attribute,
    >,
    crate::codegen::structs_codegen::tables::insertables::CommercialFreezeDryerLotExtensionAttribute: From<
        <FreezeDryerModel as common_traits::builder::Attributed>::Attribute,
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
                    crate::codegen::structs_codegen::tables::insertables::CommercialFreezeDryerLotAttribute::ProductModel,
                ),
            )?;
        let id = if self.commercial_freeze_dryer_lots_id_fkey.is_complete() {
            let id = self
                .commercial_freeze_dryer_lots_id_fkey
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|attribute| {
                        crate::codegen::structs_codegen::tables::insertables::CommercialFreezeDryerLotAttribute::Extension(
                            From::from(attribute),
                        )
                    })
                })?;
            let _ = self
                .commercial_freeze_dryer_lots_id_fkey1
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|attribute| {
                        crate::codegen::structs_codegen::tables::insertables::CommercialFreezeDryerLotAttribute::Extension(
                            From::from(attribute),
                        )
                    })
                })?;
            id
        } else {
            let id = self
                .commercial_freeze_dryer_lots_id_fkey1
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|attribute| {
                        crate::codegen::structs_codegen::tables::insertables::CommercialFreezeDryerLotAttribute::Extension(
                            From::from(attribute),
                        )
                    })
                })?;
            let _ = self
                .commercial_freeze_dryer_lots_id_fkey
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|attribute| {
                        crate::codegen::structs_codegen::tables::insertables::CommercialFreezeDryerLotAttribute::Extension(
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
