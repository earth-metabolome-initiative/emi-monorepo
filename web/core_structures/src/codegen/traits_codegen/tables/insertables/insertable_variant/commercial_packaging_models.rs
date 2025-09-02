impl<
    C: diesel::connection::LoadConnection,
    PackagingModel,
    CommercialProduct,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingModelBuilder<
    PackagingModel,
    CommercialProduct,
>
where
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization,
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::commercial_packaging_models::CommercialPackagingModel as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingModel as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::commercial_packaging_models::CommercialPackagingModel as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::commercial_packaging_models::CommercialPackagingModel,
    >,
    C: diesel::connection::LoadConnection,
    CommercialProduct: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    PackagingModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
    Self: web_common_traits::database::MostConcreteTable,
{
    type Row = crate::codegen::structs_codegen::tables::commercial_packaging_models::CommercialPackagingModel;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingModel;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingModelAttributes,
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
        self.set_most_concrete_table("commercial_packaging_models");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingModel = self
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
                    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingModelAttributes::ParentModel,
                ),
            )?;
        let id = if self.commercial_packaging_models_id_fkey1.is_complete() {
            let id = self
                .commercial_packaging_models_id_fkey1
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingModelAttributes::Extension(
                        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingModelExtensionAttributes::CommercialProduct(
                            crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductAttributes::Id,
                        ),
                    ))
                })?;
            let _ = self
                .commercial_packaging_models_id_fkey
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingModelAttributes::Extension(
                        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingModelExtensionAttributes::PackagingModel(
                            crate::codegen::structs_codegen::tables::insertables::InsertablePackagingModelAttributes::Id,
                        ),
                    ))
                })?;
            id
        } else {
            let id = self
                .commercial_packaging_models_id_fkey
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingModelAttributes::Extension(
                        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingModelExtensionAttributes::PackagingModel(
                            crate::codegen::structs_codegen::tables::insertables::InsertablePackagingModelAttributes::Id,
                        ),
                    ))
                })?;
            let _ = self
                .commercial_packaging_models_id_fkey1
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingModelAttributes::Extension(
                        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingModelExtensionAttributes::CommercialProduct(
                            crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductAttributes::Id,
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
