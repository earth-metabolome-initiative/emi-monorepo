impl<
    C: diesel::connection::LoadConnection,
    BeadModel,
    CommercialProductLot,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadLotBuilder<
    BeadModel,
    CommercialProductLot,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::commercial_bead_lots::CommercialBeadLot as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadLot as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::commercial_bead_lots::CommercialBeadLot as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::commercial_bead_lots::CommercialBeadLot,
    >,
    BeadModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
    C: diesel::connection::LoadConnection,
    CommercialProductLot: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    Self: web_common_traits::database::MostConcreteTable,
    crate::codegen::structs_codegen::tables::insertables::CommercialBeadLotAttribute: web_common_traits::database::FromExtensionAttribute<
        crate::codegen::structs_codegen::tables::insertables::CommercialProductLotAttribute,
        CommercialProductLot,
        EffectiveExtensionAttribute = <CommercialProductLot as web_common_traits::database::TryInsertGeneric<
            C,
        >>::Attribute,
    >,
    crate::codegen::structs_codegen::tables::insertables::CommercialBeadLotAttribute: web_common_traits::database::FromExtensionAttribute<
        crate::codegen::structs_codegen::tables::insertables::BeadModelAttribute,
        BeadModel,
        EffectiveExtensionAttribute = <BeadModel as web_common_traits::database::TryInsertGeneric<
            C,
        >>::Attribute,
    >,
{
    type Row = crate::codegen::structs_codegen::tables::commercial_bead_lots::CommercialBeadLot;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadLot;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::CommercialBeadLotAttribute,
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
        self.set_most_concrete_table("commercial_bead_lots");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadLot = self
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
                    crate::codegen::structs_codegen::tables::insertables::CommercialBeadLotAttribute::ProductModel,
                ),
            )?;
        let id = if self.commercial_bead_lots_id_fkey1.is_complete() {
            let id = self
                .commercial_bead_lots_id_fkey1
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|attribute| {
                        <crate::codegen::structs_codegen::tables::insertables::CommercialBeadLotAttribute as web_common_traits::database::FromExtensionAttribute<
                            crate::codegen::structs_codegen::tables::insertables::BeadModelAttribute,
                            BeadModel,
                        >>::from_extension_attribute(attribute)
                    })
                })?;
            let _ = self
                .commercial_bead_lots_id_fkey
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|attribute| {
                        <crate::codegen::structs_codegen::tables::insertables::CommercialBeadLotAttribute as web_common_traits::database::FromExtensionAttribute<
                            crate::codegen::structs_codegen::tables::insertables::CommercialProductLotAttribute,
                            CommercialProductLot,
                        >>::from_extension_attribute(attribute)
                    })
                })?;
            id
        } else {
            let id = self
                .commercial_bead_lots_id_fkey
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|attribute| {
                        <crate::codegen::structs_codegen::tables::insertables::CommercialBeadLotAttribute as web_common_traits::database::FromExtensionAttribute<
                            crate::codegen::structs_codegen::tables::insertables::CommercialProductLotAttribute,
                            CommercialProductLot,
                        >>::from_extension_attribute(attribute)
                    })
                })?;
            let _ = self
                .commercial_bead_lots_id_fkey1
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|attribute| {
                        <crate::codegen::structs_codegen::tables::insertables::CommercialBeadLotAttribute as web_common_traits::database::FromExtensionAttribute<
                            crate::codegen::structs_codegen::tables::insertables::BeadModelAttribute,
                            BeadModel,
                        >>::from_extension_attribute(attribute)
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
