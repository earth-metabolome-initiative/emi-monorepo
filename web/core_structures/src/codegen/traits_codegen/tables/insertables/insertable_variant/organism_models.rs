impl<SampleSourceModel> web_common_traits::database::DispatchableInsertVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableOrganismModelBuilder<
        SampleSourceModel,
    >
{
    type Row = crate::codegen::structs_codegen::tables::organism_models::OrganismModel;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::OrganismModelAttribute,
    >;
}
impl<SampleSourceModel> web_common_traits::database::InsertableVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableOrganismModelBuilder<
        SampleSourceModel,
    >
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableOrganismModel;
}
#[cfg(feature = "backend")]
impl<SampleSourceModel> web_common_traits::database::BackendInsertableVariant
    for crate::codegen::structs_codegen::tables::insertables::InsertableOrganismModelBuilder<
        SampleSourceModel,
    >
where
    Self: web_common_traits::database::DispatchableInsertableVariant<diesel::PgConnection>,
{
}
impl<
    C: diesel::connection::LoadConnection,
    SampleSourceModel,
> web_common_traits::database::DispatchableInsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableOrganismModelBuilder<
    SampleSourceModel,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::organism_models::OrganismModel as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableOrganismModel as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::organism_models::OrganismModel as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::organism_models::OrganismModel,
    >,
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableOrganismModel,
        Row = crate::codegen::structs_codegen::tables::organism_models::OrganismModel,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::OrganismModelAttribute,
        >,
    >,
    SampleSourceModel: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    Self: web_common_traits::database::MostConcreteTable,
    crate::codegen::structs_codegen::tables::insertables::OrganismModelExtensionAttribute: From<
        <SampleSourceModel as common_traits::builder::Attributed>::Attribute,
    >,
{
    fn insert(mut self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        use web_common_traits::database::MostConcreteTable;
        self.set_most_concrete_table("organism_models");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableOrganismModel = self
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
    SampleSourceModel,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableOrganismModelBuilder<
    SampleSourceModel,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::organism_models::OrganismModel as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableOrganismModel as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::organism_models::OrganismModel as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::organism_models::OrganismModel,
    >,
    SampleSourceModel: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = i32,
    >,
    Self: web_common_traits::database::MostConcreteTable,
    crate::codegen::structs_codegen::tables::insertables::OrganismModelExtensionAttribute: From<
        <SampleSourceModel as common_traits::builder::Attributed>::Attribute,
    >,
{
    fn try_insert(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::InsertableVariant, Self::Error> {
        let id = self
            .id
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|attribute| {
                    crate::codegen::structs_codegen::tables::insertables::OrganismModelAttribute::Extension(
                        From::from(attribute),
                    )
                })
            })?;
        Ok(Self::InsertableVariant { id })
    }
}
