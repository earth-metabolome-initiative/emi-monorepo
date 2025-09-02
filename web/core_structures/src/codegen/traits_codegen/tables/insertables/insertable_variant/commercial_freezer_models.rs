impl<
    C: diesel::connection::LoadConnection,
    FreezerModel,
    CommercialProduct,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezerModelBuilder<
    FreezerModel,
    CommercialProduct,
>
where
    <C as diesel::Connection>::Backend: diesel::backend::DieselReserveSpecialization,
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::commercial_freezer_models::CommercialFreezerModel as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezerModel as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::commercial_freezer_models::CommercialFreezerModel as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::commercial_freezer_models::CommercialFreezerModel,
    >,
    C: diesel::connection::LoadConnection,
    CommercialProduct: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    FreezerModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
    crate::codegen::structs_codegen::tables::freezer_models::FreezerModel: diesel::Identifiable
        + web_common_traits::database::Updatable<C, UserId = i32>,
    <crate::codegen::structs_codegen::tables::freezer_models::FreezerModel as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::freezer_models::FreezerModel as diesel::Identifiable>::Id,
    >,
    <<crate::codegen::structs_codegen::tables::freezer_models::FreezerModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::freezer_models::FreezerModel as diesel::Identifiable>::Id,
    >>::Output: diesel::query_dsl::methods::LimitDsl + diesel::RunQueryDsl<C>,
    <<<crate::codegen::structs_codegen::tables::freezer_models::FreezerModel as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FindDsl<
        <crate::codegen::structs_codegen::tables::freezer_models::FreezerModel as diesel::Identifiable>::Id,
    >>::Output as diesel::query_dsl::methods::LimitDsl>::Output: for<'a> diesel::query_dsl::LoadQuery<
        'a,
        C,
        crate::codegen::structs_codegen::tables::freezer_models::FreezerModel,
    >,
    Self: web_common_traits::database::MostConcreteTable,
{
    type Row = crate::codegen::structs_codegen::tables::commercial_freezer_models::CommercialFreezerModel;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezerModel;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezerModelAttributes,
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
        self.set_most_concrete_table("commercial_freezer_models");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezerModel = self
            .try_insert(user_id, conn)?;
        if !insertable_struct.parent_model(conn)?.can_update(user_id, conn)? {
            return Err(
                generic_backend_request_errors::GenericBackendRequestError::Unauthorized
                    .into(),
            );
        }
        if !insertable_struct
            .commercial_freezer_models_id_fkey(conn)?
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
        let parent_model = self
            .parent_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezerModelAttributes::ParentModel,
                ),
            )?;
        let id = if self.commercial_freezer_models_id_fkey1.is_complete() {
            let id = self
                .commercial_freezer_models_id_fkey1
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezerModelAttributes::Extension(
                        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezerModelExtensionAttributes::CommercialProduct(
                            crate::codegen::structs_codegen::tables::insertables::InsertableCommercialProductAttributes::Id,
                        ),
                    ))
                })?;
            let _ = self
                .commercial_freezer_models_id_fkey
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezerModelAttributes::Extension(
                        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezerModelExtensionAttributes::FreezerModel(
                            crate::codegen::structs_codegen::tables::insertables::InsertableFreezerModelAttributes::Id,
                        ),
                    ))
                })?;
            id
        } else {
            let id = self
                .commercial_freezer_models_id_fkey
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezerModelAttributes::Extension(
                        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezerModelExtensionAttributes::FreezerModel(
                            crate::codegen::structs_codegen::tables::insertables::InsertableFreezerModelAttributes::Id,
                        ),
                    ))
                })?;
            let _ = self
                .commercial_freezer_models_id_fkey1
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezerModelAttributes::Extension(
                        crate::codegen::structs_codegen::tables::insertables::InsertableCommercialFreezerModelExtensionAttributes::CommercialProduct(
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
