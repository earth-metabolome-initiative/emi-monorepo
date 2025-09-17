impl<
    CommercialProductLot,
    PackagingModel,
> web_common_traits::database::DispatchableInsertVariantMetadata
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingLotBuilder<
    CommercialProductLot,
    PackagingModel,
> {
    type Row = crate::codegen::structs_codegen::tables::commercial_packaging_lots::CommercialPackagingLot;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::CommercialPackagingLotAttribute,
    >;
}
impl<
    CommercialProductLot,
    PackagingModel,
> web_common_traits::database::InsertableVariantMetadata
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingLotBuilder<
    CommercialProductLot,
    PackagingModel,
> {
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingLot;
}
#[cfg(feature = "backend")]
impl<
    CommercialProductLot,
    PackagingModel,
> web_common_traits::database::BackendInsertableVariant
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingLotBuilder<
    CommercialProductLot,
    PackagingModel,
>
where
    Self: web_common_traits::database::DispatchableInsertableVariant<
        diesel::PgConnection,
    >,
{}
impl<
    C: diesel::connection::LoadConnection,
    CommercialProductLot,
    PackagingModel,
> web_common_traits::database::DispatchableInsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingLotBuilder<
    CommercialProductLot,
    PackagingModel,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::commercial_packaging_lots::CommercialPackagingLot as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingLot as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::commercial_packaging_lots::CommercialPackagingLot as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::commercial_packaging_lots::CommercialPackagingLot,
    >,
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingLot,
        Row = crate::codegen::structs_codegen::tables::commercial_packaging_lots::CommercialPackagingLot,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialPackagingLotAttribute,
        >,
    >,
    CommercialProductLot: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    PackagingModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
    Self: web_common_traits::database::MostConcreteTable,
    crate::codegen::structs_codegen::tables::insertables::CommercialPackagingLotExtensionAttribute: From<
        <CommercialProductLot as common_traits::builder::Attributed>::Attribute,
    >,
    crate::codegen::structs_codegen::tables::insertables::CommercialPackagingLotExtensionAttribute: From<
        <PackagingModel as common_traits::builder::Attributed>::Attribute,
    >,
{
    fn insert(mut self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        use web_common_traits::database::MostConcreteTable;
        self.set_most_concrete_table("commercial_packaging_lots");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingLot = self
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
    PackagingModel,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingLotBuilder<
    CommercialProductLot,
    PackagingModel,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::commercial_packaging_lots::CommercialPackagingLot as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingLot as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::commercial_packaging_lots::CommercialPackagingLot as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::commercial_packaging_lots::CommercialPackagingLot,
    >,
    CommercialProductLot: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    PackagingModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
    Self: web_common_traits::database::MostConcreteTable,
    crate::codegen::structs_codegen::tables::insertables::CommercialPackagingLotExtensionAttribute: From<
        <CommercialProductLot as common_traits::builder::Attributed>::Attribute,
    >,
    crate::codegen::structs_codegen::tables::insertables::CommercialPackagingLotExtensionAttribute: From<
        <PackagingModel as common_traits::builder::Attributed>::Attribute,
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
                    crate::codegen::structs_codegen::tables::insertables::CommercialPackagingLotAttribute::ProductModel,
                ),
            )?;
        let id = if self.commercial_packaging_lots_id_fkey.is_complete() {
            let id = self
                .commercial_packaging_lots_id_fkey
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|attribute| {
                        crate::codegen::structs_codegen::tables::insertables::CommercialPackagingLotAttribute::Extension(
                            From::from(attribute),
                        )
                    })
                })?;
            let _ = self
                .commercial_packaging_lots_id_fkey1
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|attribute| {
                        crate::codegen::structs_codegen::tables::insertables::CommercialPackagingLotAttribute::Extension(
                            From::from(attribute),
                        )
                    })
                })?;
            id
        } else {
            let id = self
                .commercial_packaging_lots_id_fkey1
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|attribute| {
                        crate::codegen::structs_codegen::tables::insertables::CommercialPackagingLotAttribute::Extension(
                            From::from(attribute),
                        )
                    })
                })?;
            let _ = self
                .commercial_packaging_lots_id_fkey
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|attribute| {
                        crate::codegen::structs_codegen::tables::insertables::CommercialPackagingLotAttribute::Extension(
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
