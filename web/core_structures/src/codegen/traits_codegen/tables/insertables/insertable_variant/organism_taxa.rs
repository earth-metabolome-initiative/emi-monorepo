impl web_common_traits::database::DispatchableInsertVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableOrganismTaxonBuilder
{
    type Row = crate::codegen::structs_codegen::tables::organism_taxa::OrganismTaxon;
    type Error = web_common_traits::database::InsertError<
        crate::codegen::structs_codegen::tables::insertables::OrganismTaxonAttribute,
    >;
}
impl web_common_traits::database::InsertableVariantMetadata
    for crate::codegen::structs_codegen::tables::insertables::InsertableOrganismTaxonBuilder
{
    type InsertableVariant =
        crate::codegen::structs_codegen::tables::insertables::InsertableOrganismTaxon;
}
#[cfg(feature = "backend")]
impl web_common_traits::database::BackendInsertableVariant
    for crate::codegen::structs_codegen::tables::insertables::InsertableOrganismTaxonBuilder
where
    Self: web_common_traits::database::DispatchableInsertableVariant<diesel::PgConnection>,
{
}
impl<
    C: diesel::connection::LoadConnection,
> web_common_traits::database::DispatchableInsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableOrganismTaxonBuilder
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::organism_taxa::OrganismTaxon as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableOrganismTaxon as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::organism_taxa::OrganismTaxon as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::organism_taxa::OrganismTaxon,
    >,
    Self: web_common_traits::database::InsertableVariant<
        C,
        InsertableVariant = crate::codegen::structs_codegen::tables::insertables::InsertableOrganismTaxon,
        Row = crate::codegen::structs_codegen::tables::organism_taxa::OrganismTaxon,
        Error = web_common_traits::database::InsertError<
            crate::codegen::structs_codegen::tables::insertables::OrganismTaxonAttribute,
        >,
    >,
{
    fn insert(self, user_id: i32, conn: &mut C) -> Result<Self::Row, Self::Error> {
        use diesel::RunQueryDsl;
        use diesel::associations::HasTable;
        use web_common_traits::database::InsertableVariant;
        let insertable_struct: crate::codegen::structs_codegen::tables::insertables::InsertableOrganismTaxon = self
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
> web_common_traits::database::InsertableVariant<C>
for crate::codegen::structs_codegen::tables::insertables::InsertableOrganismTaxonBuilder
where
    diesel::query_builder::InsertStatement<
        <crate::codegen::structs_codegen::tables::organism_taxa::OrganismTaxon as diesel::associations::HasTable>::Table,
        <crate::codegen::structs_codegen::tables::insertables::InsertableOrganismTaxon as diesel::Insertable<
            <crate::codegen::structs_codegen::tables::organism_taxa::OrganismTaxon as diesel::associations::HasTable>::Table,
        >>::Values,
    >: for<'query> diesel::query_dsl::LoadQuery<
        'query,
        C,
        crate::codegen::structs_codegen::tables::organism_taxa::OrganismTaxon,
    >,
{
    fn try_insert(
        self,
        _user_id: i32,
        _conn: &mut C,
    ) -> Result<Self::InsertableVariant, Self::Error> {
        let created_by = self
            .created_by
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::OrganismTaxonAttribute::CreatedBy,
                ),
            )?;
        let created_at = self
            .created_at
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::OrganismTaxonAttribute::CreatedAt,
                ),
            )?;
        let organism_id = self
            .organism_id
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::OrganismTaxonAttribute::OrganismId,
                ),
            )?;
        let taxon_id = self
            .taxon_id
            .ok_or(
                common_traits::prelude::BuilderError::IncompleteBuild(
                    crate::codegen::structs_codegen::tables::insertables::OrganismTaxonAttribute::TaxonId,
                ),
            )?;
        Ok(Self::InsertableVariant {
            created_by,
            created_at,
            organism_id,
            taxon_id,
        })
    }
}
