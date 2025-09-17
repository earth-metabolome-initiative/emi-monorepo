impl<ContainerModel> web_common_traits::database::DispatchableInsertVariantMetadata
for crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricContainerModelBuilder<
    ContainerModel,
> {
    type Row = crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::VolumetricContainerModelAttribute,
    >;
}
impl<ContainerModel> web_common_traits::database::InsertableVariantMetadata
for crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricContainerModelBuilder<
    ContainerModel,
> {
    type InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricContainerModel;
}
#[cfg(feature = "backend")]
impl<ContainerModel> web_common_traits::database::BackendInsertableVariant
for crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricContainerModelBuilder<
    ContainerModel,
>
where
    Self: web_common_traits::database::DispatchableInsertableVariant<
        diesel::PgConnection,
    >,
{}
impl<
    C: diesel::connection::LoadConnection,
    ContainerModel,
> web_common_traits::database::DispatchableInsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricContainerModelBuilder<
    ContainerModel,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricContainerModel as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
    >,
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricContainerModel,
        Row = crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::VolumetricContainerModelAttribute,
        >,
    >,
    ContainerModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
    Self: web_common_traits::database::MostConcreteTable,
    crate::codegen::structs_codegen::tables::insertables::VolumetricContainerModelExtensionAttribute: From<
        <ContainerModel as common_traits::builder::Attributed>::Attribute,
    >,
{
    fn insert(mut self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        use web_common_traits::database::MostConcreteTable;
        self.set_most_concrete_table("volumetric_container_models");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricContainerModel = self
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
    ContainerModel,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricContainerModelBuilder<
    ContainerModel,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableVolumetricContainerModel as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::volumetric_container_models::VolumetricContainerModel,
    >,
    ContainerModel: web_common_traits::database::TryInsertGeneric<C, PrimaryKey = i32>,
    Self: web_common_traits::database::MostConcreteTable,
    crate::codegen::structs_codegen::tables::insertables::VolumetricContainerModelExtensionAttribute: From<
        <ContainerModel as common_traits::builder::Attributed>::Attribute,
    >,
{
    fn try_insert(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::InsertableVariant, Self::Error> {
        let liters = self
            .liters
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::VolumetricContainerModelAttribute::Liters,
                ),
            )?;
        let id = self
            .id
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|attribute| {
                    crate::codegen::structs_codegen::tables::insertables::VolumetricContainerModelAttribute::Extension(
                        From::from(attribute),
                    )
                })
            })?;
        Ok(Self::InsertableVariant {
            id,
            liters,
        })
    }
}
