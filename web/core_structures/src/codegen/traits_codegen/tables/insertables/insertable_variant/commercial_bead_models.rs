impl<
    C: diesel::connection::LoadConnection,
    BeadModel,
    CommercialProduct,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadModelBuilder<
    BeadModel,
    CommercialProduct,
>
where
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization,
    diesel::query_builder::InsertStatement<
        <crate::CommercialBeadModel as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadModel as diesel::Insertable<
            <crate::CommercialBeadModel as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<'query, C, crate::CommercialBeadModel>,
    BeadModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
    C: diesel::connection::LoadConnection,
    CommercialProduct: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    Self: web_common_traits::database::MostConcreteTable,
{
    type Row = crate::CommercialBeadModel;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadModel;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::CommercialBeadModelAttribute,
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
        self.set_most_concrete_table("commercial_bead_models");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialBeadModel = self
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
        let bead_model = self
            .bead_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::CommercialBeadModelAttribute::BeadModel,
                ),
            )?;
        let id = if self.commercial_bead_models_id_fkey.is_complete() {
            let id = self
                .commercial_bead_models_id_fkey
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::CommercialBeadModelAttribute::Extension(
                        crate::codegen::structs_codegen::tables::insertables::CommercialBeadModelExtensionAttribute::BeadModel(
                            crate::codegen::structs_codegen::tables::insertables::BeadModelAttribute::Id,
                        ),
                    ))
                })?;
            let _ = self
                .commercial_bead_models_id_fkey1
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::CommercialBeadModelAttribute::Extension(
                        crate::codegen::structs_codegen::tables::insertables::CommercialBeadModelExtensionAttribute::CommercialProduct(
                            crate::codegen::structs_codegen::tables::insertables::CommercialProductAttribute::Id,
                        ),
                    ))
                })?;
            id
        } else {
            let id = self
                .commercial_bead_models_id_fkey1
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::CommercialBeadModelAttribute::Extension(
                        crate::codegen::structs_codegen::tables::insertables::CommercialBeadModelExtensionAttribute::CommercialProduct(
                            crate::codegen::structs_codegen::tables::insertables::CommercialProductAttribute::Id,
                        ),
                    ))
                })?;
            let _ = self
                .commercial_bead_models_id_fkey
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::CommercialBeadModelAttribute::Extension(
                        crate::codegen::structs_codegen::tables::insertables::CommercialBeadModelExtensionAttribute::BeadModel(
                            crate::codegen::structs_codegen::tables::insertables::BeadModelAttribute::Id,
                        ),
                    ))
                })?;
            id
        };
        Ok(Self::InsertableVariant {
            id,
            bead_model,
        })
    }
}
