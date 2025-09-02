impl<
    C: diesel::connection::LoadConnection,
    BeadsModel,
    CommercialProduct,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadsModelBuilder<
    BeadsModel,
    CommercialProduct,
>
where
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization,
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::commercial_beads_models::CommercialBeadsModel as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadsModel as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::commercial_beads_models::CommercialBeadsModel as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::commercial_beads_models::CommercialBeadsModel,
    >,
    BeadsModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
    C: diesel::connection::LoadConnection,
    CommercialProduct: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    Self: web_common_traits::database::MostConcreteTable,
{
    type Row = crate::codegen::structs_codegen::tables::commercial_beads_models::CommercialBeadsModel;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadsModel;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadsModelAttributes,
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
        self.set_most_concrete_table("commercial_beads_models");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadsModel = self
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
        let parent_model = self
            .parent_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadsModelAttributes::ParentModel,
                ),
            )?;
        let id = if self.commercial_beads_models_id_fkey.is_complete() {
            let id = self
                .commercial_beads_models_id_fkey
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadsModelAttributes::Extension(
                        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadsModelExtensionAttributes::BeadsModel(
                            crate::codegen::structs_codegen::tables::insertables::InsertableBeadsModelAttributes::Id,
                        ),
                    ))
                })?;
            let _ = self
                .commercial_beads_models_id_fkey1
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadsModelAttributes::Extension(
                        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadsModelExtensionAttributes::CommercialProduct(
                            crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductAttributes::Id,
                        ),
                    ))
                })?;
            id
        } else {
            let id = self
                .commercial_beads_models_id_fkey1
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadsModelAttributes::Extension(
                        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadsModelExtensionAttributes::CommercialProduct(
                            crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductAttributes::Id,
                        ),
                    ))
                })?;
            let _ = self
                .commercial_beads_models_id_fkey
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadsModelAttributes::Extension(
                        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadsModelExtensionAttributes::BeadsModel(
                            crate::codegen::structs_codegen::tables::insertables::InsertableBeadsModelAttributes::Id,
                        ),
                    ))
                })?;
            id
        };
        Ok(Self::InsertableVariant {
            id,
            parent_model,
        })
    }
}
