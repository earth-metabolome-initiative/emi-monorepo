impl<
    C: diesel::connection::LoadConnection,
    CommercialProductLot,
    CapModel,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCapLotBuilder<
    CommercialProductLot,
    CapModel,
>
where
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization,
    diesel::query_builder::InsertStatement<
        <crate::CommercialCapLot as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCapLot as diesel::Insertable<
            <crate::CommercialCapLot as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<'query, C, crate::CommercialCapLot>,
    C: diesel::connection::LoadConnection,
    CapModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
    CommercialProductLot: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    Self: web_common_traits::database::MostConcreteTable,
{
    type Row = crate::CommercialCapLot;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCapLot;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::CommercialCapLotAttribute,
    >;
    type UserId = i32;
    fn insert(
        mut self,
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::MostConcreteTable;
        self.set_most_concrete_table("commercial_cap_lots");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCapLot = self
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
        let product_model = self
            .product_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::CommercialCapLotAttribute::ProductModel,
                ),
            )?;
        let id = if self.commercial_cap_lots_id_fkey1.is_complete() {
            let id = self
                .commercial_cap_lots_id_fkey1
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::CommercialCapLotAttribute::Extension(
                        crate::codegen::structs_codegen::tables::insertables::CommercialCapLotExtensionAttribute::CapModel(
                            crate::codegen::structs_codegen::tables::insertables::CapModelAttribute::Id,
                        ),
                    ))
                })?;
            let _ = self
                .commercial_cap_lots_id_fkey
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::CommercialCapLotAttribute::Extension(
                        crate::codegen::structs_codegen::tables::insertables::CommercialCapLotExtensionAttribute::CommercialProductLot(
                            crate::codegen::structs_codegen::tables::insertables::CommercialProductLotAttribute::Id,
                        ),
                    ))
                })?;
            id
        } else {
            let id = self
                .commercial_cap_lots_id_fkey
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::CommercialCapLotAttribute::Extension(
                        crate::codegen::structs_codegen::tables::insertables::CommercialCapLotExtensionAttribute::CommercialProductLot(
                            crate::codegen::structs_codegen::tables::insertables::CommercialProductLotAttribute::Id,
                        ),
                    ))
                })?;
            let _ = self
                .commercial_cap_lots_id_fkey1
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::CommercialCapLotAttribute::Extension(
                        crate::codegen::structs_codegen::tables::insertables::CommercialCapLotExtensionAttribute::CapModel(
                            crate::codegen::structs_codegen::tables::insertables::CapModelAttribute::Id,
                        ),
                    ))
                })?;
            id
        };
        Ok(Self::InsertableVariant {
            id,
            product_model,
        })
    }
}
