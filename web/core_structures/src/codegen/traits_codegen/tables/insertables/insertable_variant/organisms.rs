impl<SampleSource> web_common_traits::database::DispatchableInsertVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableOrganismBuilder<
        SampleSource,
    >
{
    type Row = crate::codegen::structs_codegen::tables::organisms::Organism;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::OrganismAttribute,
    >;
}
impl<SampleSource> web_common_traits::database::InsertableVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableOrganismBuilder<
        SampleSource,
    >
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableOrganism;
}
#[cfg(feature = "backend")]
impl<SampleSource> web_common_traits::database::BackendInsertableVariant
    for crate::codegen::structs_codegen::tables::insertables::InsertableOrganismBuilder<
        SampleSource,
    >
where
    Self: web_common_traits::database::DispatchableInsertableVariant<diesel::PgConnection>,
{
}
impl<
    C: diesel::connection::LoadConnection,
    SampleSource,
> web_common_traits::database::DispatchableInsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableOrganismBuilder<
    SampleSource,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::organisms::Organism as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableOrganism as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::organisms::Organism as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::organisms::Organism,
    >,
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableOrganism,
        Row = crate::codegen::structs_codegen::tables::organisms::Organism,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::OrganismAttribute,
        >,
    >,
    SampleSource: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
    Self: web_common_traits::database::MostConcreteTable,
    crate::codegen::structs_codegen::tables::insertables::OrganismExtensionAttribute: From<
        <SampleSource as common_traits::builder::Attributed>::Attribute,
    >,
{
    fn insert(mut self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        use web_common_traits::database::MostConcreteTable;
        self.set_most_concrete_table("organisms");
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableOrganism = self
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
    SampleSource,
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableOrganismBuilder<
    SampleSource,
>
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::organisms::Organism as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableOrganism as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::organisms::Organism as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::organisms::Organism,
    >,
    SampleSource: web_common_traits::database::TryInsertGeneric<
        C,
        PrimaryKey = ::rosetta_uuid::Uuid,
    >,
    Self: web_common_traits::database::MostConcreteTable,
    crate::codegen::structs_codegen::tables::insertables::OrganismExtensionAttribute: From<
        <SampleSource as common_traits::builder::Attributed>::Attribute,
    >,
{
    fn try_insert(
        self,
        user_id: i32,
        conn: &mut C,
    ) -> Result<Self::InsertableVariant, Self::Error> {
        let model = self
            .model
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::OrganismAttribute::Model,
                ),
            )?;
        let id = self
            .id
            .mint_primary_key(user_id, conn)
            .map_err(|err| {
                err.into_field_name(|attribute| {
                    crate::codegen::structs_codegen::tables::insertables::OrganismAttribute::Extension(
                        From::from(attribute),
                    )
                })
            })?;
        Ok(Self::InsertableVariant {
            id,
            model,
        })
    }
}
