impl<
    C: diesel::connection::LoadConnection,
    CapModel,
    CommercialProduct,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCapModelBuilder<
    CapModel,
    CommercialProduct,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::commercial_cap_models::CommercialCapModel as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCapModel as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::commercial_cap_models::CommercialCapModel as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::commercial_cap_models::CommercialCapModel,
    >,
    C: diesel::connection::LoadConnection,
    CapModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
    CommercialProduct: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    Self: web_common_traits::database::MostConcreteTable,
{
    type Row = crate::codegen::structs_codegen::tables::commercial_cap_models::CommercialCapModel;
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCapModel;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::CommercialCapModelAttribute,
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
        self.set_most_concrete_table("commercial_cap_models");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialCapModel = self
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
        let cap_model = self
            .cap_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::CommercialCapModelAttribute::CapModel,
                ),
            )?;
        let id = if self.commercial_cap_models_id_fkey.is_complete() {
            let id = self
                .commercial_cap_models_id_fkey
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::CommercialCapModelAttribute::Extension(
                        crate::codegen::structs_codegen::tables::insertables::CommercialCapModelExtensionAttribute::CapModel(
                            crate::codegen::structs_codegen::tables::insertables::CapModelAttribute::Id,
                        ),
                    ))
                })?;
            let _ = self
                .commercial_cap_models_id_fkey1
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::CommercialCapModelAttribute::Extension(
                        crate::codegen::structs_codegen::tables::insertables::CommercialCapModelExtensionAttribute::CommercialProduct(
                            crate::codegen::structs_codegen::tables::insertables::CommercialProductAttribute::Id,
                        ),
                    ))
                })?;
            id
        } else {
            let id = self
                .commercial_cap_models_id_fkey1
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::CommercialCapModelAttribute::Extension(
                        crate::codegen::structs_codegen::tables::insertables::CommercialCapModelExtensionAttribute::CommercialProduct(
                            crate::codegen::structs_codegen::tables::insertables::CommercialProductAttribute::Id,
                        ),
                    ))
                })?;
            let _ = self
                .commercial_cap_models_id_fkey
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|_| crate::codegen::structs_codegen::tables::insertables::CommercialCapModelAttribute::Extension(
                        crate::codegen::structs_codegen::tables::insertables::CommercialCapModelExtensionAttribute::CapModel(
                            crate::codegen::structs_codegen::tables::insertables::CapModelAttribute::Id,
                        ),
                    ))
                })?;
            id
        };
        Ok(Self::InsertableVariant {
            id,
            cap_model,
        })
    }
}
