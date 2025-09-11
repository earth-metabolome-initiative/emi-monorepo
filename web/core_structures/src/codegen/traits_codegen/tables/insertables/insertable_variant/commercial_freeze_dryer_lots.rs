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
    C: diesel::connection::LoadConnection,
    CommercialProductLot: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    FreezeDryerModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
    crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel: web_common_traits::database::Read<
        C,
    >,
    crate::codegen::structs_codegen::tables::freeze_dryer_models::FreezeDryerModel: web_common_traits::database::Updatable<
        C,
        UserId = i32,
    >,
    Self: web_common_traits::database::MostConcreteTable,
    crate::codegen::structs_codegen::tables::insertables::CommercialFreezeDryerLotAttribute: web_common_traits::database::FromExtensionAttribute<
        crate::codegen::structs_codegen::tables::insertables::CommercialProductLotAttribute,
        CommercialProductLot,
        EffectiveExtensionAttribute = <CommercialProductLot as web_common_traits::database::TryInsertGeneric<
            C,
        >>::Attribute,
    >,
    crate::codegen::structs_codegen::tables::insertables::CommercialFreezeDryerLotAttribute: web_common_traits::database::FromExtensionAttribute<
        crate::codegen::structs_codegen::tables::insertables::FreezeDryerModelAttribute,
        FreezeDryerModel,
        EffectiveExtensionAttribute = <FreezeDryerModel as web_common_traits::database::TryInsertGeneric<
            C,
        >>::Attribute,
    >,
{
    type Row = crate::codegen::structs_codegen::tables::commercial_freeze_dryer_lots::CommercialFreezeDryerLot;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezeDryerLot;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::CommercialFreezeDryerLotAttribute,
    >;
    type UserId = i32;
    fn insert(
        mut self,
        user_id: Self::UserId,
        conn: &mut C,
    ) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::Updatable;
        use web_common_traits::database::MostConcreteTable;
        self.set_most_concrete_table("commercial_freeze_dryer_lots");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezeDryerLot = self
            .try_insert(user_id, conn)?;
        if !insertable_struct
            .commercial_freeze_dryer_lots_id_fkey1(conn)?
            .can_update(user_id, conn)?
        {
            return Err(
                generic_backend_request_errors::GenericBackendRequestError::Unauthorized
                    .into(),
            );
        }
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
                    crate::codegen::structs_codegen::tables::insertables::CommercialFreezeDryerLotAttribute::ProductModel,
                ),
            )?;
        let id = if self.commercial_freeze_dryer_lots_id_fkey.is_complete() {
            let id = self
                .commercial_freeze_dryer_lots_id_fkey
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|attribute| {
                        <crate::codegen::structs_codegen::tables::insertables::CommercialFreezeDryerLotAttribute as web_common_traits::database::FromExtensionAttribute<
                            crate::codegen::structs_codegen::tables::insertables::CommercialProductLotAttribute,
                            CommercialProductLot,
                        >>::from_extension_attribute(attribute)
                    })
                })?;
            let _ = self
                .commercial_freeze_dryer_lots_id_fkey1
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|attribute| {
                        <crate::codegen::structs_codegen::tables::insertables::CommercialFreezeDryerLotAttribute as web_common_traits::database::FromExtensionAttribute<
                            crate::codegen::structs_codegen::tables::insertables::FreezeDryerModelAttribute,
                            FreezeDryerModel,
                        >>::from_extension_attribute(attribute)
                    })
                })?;
            id
        } else {
            let id = self
                .commercial_freeze_dryer_lots_id_fkey1
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|attribute| {
                        <crate::codegen::structs_codegen::tables::insertables::CommercialFreezeDryerLotAttribute as web_common_traits::database::FromExtensionAttribute<
                            crate::codegen::structs_codegen::tables::insertables::FreezeDryerModelAttribute,
                            FreezeDryerModel,
                        >>::from_extension_attribute(attribute)
                    })
                })?;
            let _ = self
                .commercial_freeze_dryer_lots_id_fkey
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|attribute| {
                        <crate::codegen::structs_codegen::tables::insertables::CommercialFreezeDryerLotAttribute as web_common_traits::database::FromExtensionAttribute<
                            crate::codegen::structs_codegen::tables::insertables::CommercialProductLotAttribute,
                            CommercialProductLot,
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
