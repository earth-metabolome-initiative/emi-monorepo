impl<
    PackagingModel,
    CommercialProduct,
> web_common_traits::database::DispatchableInsertVariantMetadata
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingModelBuilder<
    PackagingModel,
    CommercialProduct,
> {
    type Row = crate::codegen::structs_codegen::tables::commercial_packaging_models::CommercialPackagingModel;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::CommercialPackagingModelAttribute,
    >;
}
impl<
    PackagingModel,
    CommercialProduct,
> web_common_traits::database::InsertableVariantMetadata
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingModelBuilder<
    PackagingModel,
    CommercialProduct,
> {
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingModel;
}
#[cfg(feature = "backend")]
impl<
    PackagingModel,
    CommercialProduct,
> web_common_traits::database::BackendInsertableVariant
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingModelBuilder<
    PackagingModel,
    CommercialProduct,
>
where
    Self: web_common_traits::database::DispatchableInsertableVariant<
        diesel::PgConnection,
    >,
{}
impl<
    C: diesel::connection::LoadConnection,
    PackagingModel,
    CommercialProduct,
> web_common_traits::database::DispatchableInsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingModelBuilder<
    PackagingModel,
    CommercialProduct,
>
where
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
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingModel,
        Row = crate::codegen::structs_codegen::tables::commercial_packaging_models::CommercialPackagingModel,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::CommercialPackagingModelAttribute,
        >,
    >,
    CommercialProduct: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    PackagingModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
    Self: web_common_traits::database::MostConcreteTable,
    crate::codegen::structs_codegen::tables::insertables::CommercialPackagingModelExtensionAttribute: From<
        <PackagingModel as common_traits::builder::Attributed>::Attribute,
    >,
    crate::codegen::structs_codegen::tables::insertables::CommercialPackagingModelExtensionAttribute: From<
        <CommercialProduct as common_traits::builder::Attributed>::Attribute,
    >,
{
    fn insert(mut self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        use web_common_traits::database::MostConcreteTable;
        self.set_most_concrete_table("commercial_packaging_models");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingModel = self
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
    PackagingModel,
    CommercialProduct,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableCommercialPackagingModelBuilder<
    PackagingModel,
    CommercialProduct,
>
where
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
    CommercialProduct: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    PackagingModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
    Self: web_common_traits::database::MostConcreteTable,
    crate::codegen::structs_codegen::tables::insertables::CommercialPackagingModelExtensionAttribute: From<
        <PackagingModel as common_traits::builder::Attributed>::Attribute,
    >,
    crate::codegen::structs_codegen::tables::insertables::CommercialPackagingModelExtensionAttribute: From<
        <CommercialProduct as common_traits::builder::Attributed>::Attribute,
    >,
{
    fn try_insert(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::InsertableVariant, Self::Error> {
        let packaging_model = self
            .packaging_model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::CommercialPackagingModelAttribute::PackagingModel,
                ),
            )?;
        let id = if self.commercial_packaging_models_id_fkey.is_complete() {
            let id = self
                .commercial_packaging_models_id_fkey
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|attribute| {
                        crate::codegen::structs_codegen::tables::insertables::CommercialPackagingModelAttribute::Extension(
                            From::from(attribute),
                        )
                    })
                })?;
            let _ = self
                .commercial_packaging_models_id_fkey1
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|attribute| {
                        crate::codegen::structs_codegen::tables::insertables::CommercialPackagingModelAttribute::Extension(
                            From::from(attribute),
                        )
                    })
                })?;
            id
        } else {
            let id = self
                .commercial_packaging_models_id_fkey1
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|attribute| {
                        crate::codegen::structs_codegen::tables::insertables::CommercialPackagingModelAttribute::Extension(
                            From::from(attribute),
                        )
                    })
                })?;
            let _ = self
                .commercial_packaging_models_id_fkey
                .set_primary_key(id)
                .mint_primary_key(user_id, conn)
                .map_err(|err| {
                    err.into_field_name(|attribute| {
                        crate::codegen::structs_codegen::tables::insertables::CommercialPackagingModelAttribute::Extension(
                            From::from(attribute),
                        )
                    })
                })?;
            id
        };
        Ok(Self::InsertableVariant {
            id,
            packaging_model,
        })
    }
}
